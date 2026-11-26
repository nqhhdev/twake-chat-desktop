//! The Activity (web `readActivity.ts`): mentions of the user, reactions of others on
//! their messages and the threads they take part in, gathered from the sync of every
//! room and kept in the encrypted store. [`crate::Event::Activity`] after each change.

use std::sync::Arc;
use std::time::Duration;

use matrix_sdk::ruma::events::reaction::OriginalSyncReactionEvent;
use matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent;
use matrix_sdk::ruma::events::room::redaction::SyncRoomRedactionEvent;
use matrix_sdk::ruma::serde::Raw;
use matrix_sdk::{Client, Room};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use twake_domain::activity::{
    ActivityMention, ActivityMessage, ActivityReaction, ActivityThread, ActivityThreadSummary,
    LastReply, ThreadUnread, build_activity,
};

use crate::engine::{Emit, Event};
use crate::search_index::SearchIndex;

const KEY: &[u8] = b"app.twake.chat.activity";
/// What is kept of each kind; the view shows the newest 50.
const KEEP: usize = 200;
const SAVE_DELAY: Duration = Duration::from_secs(2);

#[derive(Default, Serialize, Deserialize)]
struct Stored {
    mentions: Vec<ActivityMention>,
    /// With the id of the reaction event, to drop it when it is taken back.
    reactions: Vec<(String, ActivityReaction)>,
    threads: Vec<ActivityThread>,
}

pub struct ActivityLog {
    client: Client,
    index: Arc<SearchIndex>,
    stored: Mutex<Stored>,
    dirty: std::sync::atomic::AtomicBool,
}

fn room_name(room: &Room) -> String {
    room.cached_display_name()
        .map(|n| n.to_string())
        .unwrap_or_else(|| room.room_id().to_string())
}

/// `isMention`: the message names the user in `m.mentions` (or everyone); a message of
/// an older client without `m.mentions` names them in its text.
fn is_mention(content: &serde_json::Value, own_id: &str, own_name: &str) -> bool {
    match content.get("m.mentions") {
        Some(mentions) => {
            mentions["room"] == true
                || mentions["user_ids"]
                    .as_array()
                    .is_some_and(|ids| ids.iter().any(|id| id == own_id))
        }
        None => {
            let body = content["body"].as_str().unwrap_or("").to_lowercase();
            let local = crate::room_admin::localpart(own_id).to_lowercase();
            (!own_name.is_empty() && body.contains(&own_name.to_lowercase()))
                || (!local.is_empty() && body.contains(&local))
        }
    }
}

impl ActivityLog {
    pub async fn start(
        client: Client,
        index: Arc<SearchIndex>,
        emit: Emit,
    ) -> (Arc<Self>, tokio::task::JoinHandle<()>) {
        let stored: Stored = client
            .state_store()
            .get_custom_value(KEY)
            .await
            .ok()
            .flatten()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default();
        let log = Arc::new(ActivityLog {
            client: client.clone(),
            index,
            stored: Mutex::new(stored),
            dirty: std::sync::atomic::AtomicBool::new(true),
        });
        {
            let log = log.clone();
            client.add_event_handler(move |raw: Raw<OriginalSyncRoomMessageEvent>, room: Room| {
                let log = log.clone();
                async move {
                    if let Ok(value) = raw.deserialize_as::<serde_json::Value>() {
                        log.on_message(&room, &value).await;
                    }
                }
            });
        }
        {
            let log = log.clone();
            client.add_event_handler(move |raw: Raw<OriginalSyncReactionEvent>, room: Room| {
                let log = log.clone();
                async move {
                    if let Ok(value) = raw.deserialize_as::<serde_json::Value>() {
                        log.on_reaction(&room, &value).await;
                    }
                }
            });
        }
        {
            let log = log.clone();
            client.add_event_handler(move |raw: Raw<SyncRoomRedactionEvent>| {
                let log = log.clone();
                async move {
                    let Ok(value) = raw.deserialize_as::<serde_json::Value>() else {
                        return;
                    };
                    let redacts = value
                        .get("redacts")
                        .or_else(|| value.pointer("/content/redacts"))
                        .and_then(|r| r.as_str());
                    if let Some(id) = redacts {
                        log.forget(id).await;
                    }
                }
            });
        }
        let writer = {
            let log = log.clone();
            tokio::spawn(async move {
                loop {
                    if log.dirty.swap(false, std::sync::atomic::Ordering::Relaxed) {
                        emit(Event::Activity(log.activity().await));
                        log.save().await;
                    }
                    tokio::time::sleep(SAVE_DELAY).await;
                }
            })
        };
        (log, writer)
    }

    fn touch(&self) {
        self.dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    async fn save(&self) {
        let bytes = serde_json::to_vec(&*self.stored.lock().await).unwrap_or_default();
        let _ = self.client.state_store().set_custom_value(KEY, bytes).await;
    }

    /// `buildActivity` of what is kept.
    pub async fn activity(&self) -> twake_domain::activity::Activity {
        let stored = self.stored.lock().await;
        let reactions: Vec<ActivityReaction> =
            stored.reactions.iter().map(|(_, r)| r.clone()).collect();
        build_activity(&stored.mentions, &stored.threads, &reactions)
    }

    async fn on_message(&self, room: &Room, event: &serde_json::Value) {
        let own = room.own_user_id().as_str().to_string();
        let sender = event["sender"].as_str().unwrap_or("");
        let content = &event["content"];
        if content
            .pointer("/m.relates_to/rel_type")
            .and_then(|r| r.as_str())
            == Some("m.replace")
        {
            return;
        }
        let id = event["event_id"].as_str().unwrap_or("").to_string();
        let sent_at = event["origin_server_ts"].as_f64().unwrap_or(0.0);
        let text = content["body"].as_str().unwrap_or("").to_string();
        let sender_name = crate::search_index::sender_name(room, sender).await;
        let own_name = match room.get_member_no_sync(room.own_user_id()).await {
            Ok(Some(member)) => member.name().to_string(),
            _ => String::new(),
        };
        let name = room_name(room);
        let room_id = room.room_id().to_string();
        let mut stored = self.stored.lock().await;
        if sender != own
            && is_mention(content, &own, &own_name)
            && !stored.mentions.iter().any(|m| m.message.id == id)
        {
            stored.mentions.push(ActivityMention {
                room_id: room_id.clone(),
                room_name: name.clone(),
                message: ActivityMessage {
                    id: id.clone(),
                    sender_name: sender_name.clone(),
                    sender_avatar_url: None,
                    text: text.clone(),
                    sent_at,
                },
            });
            trim_oldest(&mut stored.mentions, |m| m.message.sent_at);
            self.touch();
        }
        let in_thread = content
            .pointer("/m.relates_to/rel_type")
            .and_then(|r| r.as_str())
            == Some("m.thread");
        let Some(root) = content
            .pointer("/m.relates_to/event_id")
            .and_then(|r| r.as_str())
            .filter(|_| in_thread)
        else {
            return;
        };
        drop(stored);
        let root_entry = self.index.entry(&room_id, root).await;
        let mut stored = self.stored.lock().await;
        let position = stored
            .threads
            .iter()
            .position(|t| t.room_id == room_id && t.thread.thread_id == root);
        let thread = match position {
            Some(i) => &mut stored.threads[i],
            None => {
                stored.threads.push(ActivityThread {
                    room_id: room_id.clone(),
                    room_name: name,
                    thread: ActivityThreadSummary {
                        thread_id: root.to_string(),
                        title: root_entry
                            .as_ref()
                            .map(|e| e.text.clone())
                            .unwrap_or_default(),
                        is_participating: false,
                        is_resolved: None,
                        last_reply: None,
                        last_own_reply_at: None,
                        unread: ThreadUnread { unread_count: 0.0 },
                    },
                });
                let last = stored.threads.len() - 1;
                &mut stored.threads[last]
            }
        };
        let summary = &mut thread.thread;
        if root_entry.as_ref().is_some_and(|e| e.sender_id == own) || sender == own {
            summary.is_participating = true;
        }
        if summary
            .last_reply
            .as_ref()
            .is_none_or(|r| r.sent_at <= sent_at)
        {
            summary.last_reply = Some(LastReply {
                sender_name,
                text,
                sent_at,
            });
        }
        if sender == own {
            summary.last_own_reply_at = Some(summary.last_own_reply_at.unwrap_or(0.0).max(sent_at));
            summary.unread.unread_count = 0.0;
        } else if sent_at > summary.last_own_reply_at.unwrap_or(0.0) {
            // ponytail: replies since the user's last one, not read receipts per thread
            summary.unread.unread_count += 1.0;
        }
        trim_oldest(&mut stored.threads, |t| {
            t.thread.last_reply.as_ref().map_or(0.0, |r| r.sent_at)
        });
        self.touch();
    }

    async fn on_reaction(&self, room: &Room, event: &serde_json::Value) {
        let own = room.own_user_id().as_str();
        let sender = event["sender"].as_str().unwrap_or("");
        if sender == own {
            return;
        }
        let (Some(target), Some(key)) = (
            event
                .pointer("/content/m.relates_to/event_id")
                .and_then(|t| t.as_str()),
            event
                .pointer("/content/m.relates_to/key")
                .and_then(|k| k.as_str()),
        ) else {
            return;
        };
        let room_id = room.room_id().to_string();
        let Some(message) = self.index.entry(&room_id, target).await else {
            return;
        };
        if message.sender_id != own {
            return;
        }
        let sender_name = crate::search_index::sender_name(room, sender).await;
        let reaction_id = event["event_id"].as_str().unwrap_or("").to_string();
        let mut stored = self.stored.lock().await;
        if stored.reactions.iter().any(|(id, _)| *id == reaction_id) {
            return;
        }
        stored.reactions.push((
            reaction_id,
            ActivityReaction {
                room_id,
                room_name: room_name(room),
                key: key.to_string(),
                sender_name,
                reacted_at: event["origin_server_ts"].as_f64().unwrap_or(0.0),
                message: ActivityMessage {
                    id: message.message_id,
                    sender_name: message.sender_name,
                    sender_avatar_url: None,
                    text: message.text,
                    sent_at: message.sent_at,
                },
            },
        ));
        trim_oldest(&mut stored.reactions, |(_, r)| r.reacted_at);
        self.touch();
    }

    /// A redaction: the mention or reaction it takes back leaves the Activity.
    async fn forget(&self, id: &str) {
        let mut stored = self.stored.lock().await;
        let before = stored.mentions.len() + stored.reactions.len();
        stored.mentions.retain(|m| m.message.id != id);
        stored
            .reactions
            .retain(|(rid, r)| rid != id && r.message.id != id);
        if stored.mentions.len() + stored.reactions.len() != before {
            self.touch();
        }
    }
}

/// Keeps the newest `KEEP` items.
fn trim_oldest<T>(items: &mut Vec<T>, at: impl Fn(&T) -> f64) {
    if items.len() > KEEP {
        items.sort_by(|a, b| at(b).total_cmp(&at(a)));
        items.truncate(KEEP);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mentions_by_m_mentions_or_by_name() {
        let own = "@alice:x";
        let named = serde_json::json!({"body": "hi", "m.mentions": {"user_ids": ["@alice:x"]}});
        assert!(is_mention(&named, own, "Alice"));
        let everyone = serde_json::json!({"body": "hi", "m.mentions": {"room": true}});
        assert!(is_mention(&everyone, own, "Alice"));
        let other = serde_json::json!({"body": "alice", "m.mentions": {}});
        assert!(!is_mention(&other, own, "Alice"));
        let legacy = serde_json::json!({"body": "ask Alice"});
        assert!(is_mention(&legacy, own, "Alice"));
    }
}
