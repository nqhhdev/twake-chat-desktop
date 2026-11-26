//! The local search index (web `feedSearchIndex.ts`, `search` use case): what this device
//! has read of each room, sealed in the encrypted store, never sent anywhere. Matched with
//! the web's rules (`matchesFilter`): every word, accents and case ignored, the last word
//! a prefix unless the query ends with a space.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use matrix_sdk::ruma::api::client::filter::RoomEventFilter;
use matrix_sdk::ruma::api::client::search::search_events::v3::{Categories, Criteria, Request};
use matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent;
use matrix_sdk::ruma::events::room::redaction::SyncRoomRedactionEvent;
use matrix_sdk::ruma::serde::Raw;
use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};
use matrix_sdk::{Client, Room};
use tokio::sync::Mutex;
use twake_domain::search::{IndexedMessage, SearchFilter, matches_filter};

/// The most results a search shows, newest first.
const LIMIT: usize = 200;
/// Writes to the store are grouped (web: 100 ms; less often here, the whole room is written).
const SAVE_DELAY: Duration = Duration::from_secs(2);

fn key(room_id: &str) -> Vec<u8> {
    format!("app.twake.chat.search.{room_id}").into_bytes()
}

pub struct SearchIndex {
    client: Client,
    rooms: Mutex<HashMap<String, HashMap<String, IndexedMessage>>>,
    dirty: Mutex<HashSet<String>>,
}

impl SearchIndex {
    /// The index of this session, fed by every message the sync delivers, and its task
    /// that writes the changes to the store.
    pub fn start(client: Client) -> (Arc<Self>, tokio::task::JoinHandle<()>) {
        let index = Arc::new(SearchIndex {
            client: client.clone(),
            rooms: Mutex::new(HashMap::new()),
            dirty: Mutex::new(HashSet::new()),
        });
        {
            let index = index.clone();
            client.add_event_handler(move |raw: Raw<OriginalSyncRoomMessageEvent>, room: Room| {
                let index = index.clone();
                async move {
                    if let Ok(value) = raw.deserialize_as::<serde_json::Value>() {
                        index.feed_event(&room, &value).await;
                    }
                }
            });
        }
        {
            let index = index.clone();
            client.add_event_handler(move |raw: Raw<SyncRoomRedactionEvent>, room: Room| {
                let index = index.clone();
                async move {
                    let Ok(value) = raw.deserialize_as::<serde_json::Value>() else {
                        return;
                    };
                    let redacts = value
                        .get("redacts")
                        .or_else(|| value.pointer("/content/redacts"))
                        .and_then(|r| r.as_str());
                    if let Some(id) = redacts {
                        index.remove(room.room_id().as_str(), id).await;
                    }
                }
            });
        }
        let writer = {
            let index = index.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(SAVE_DELAY).await;
                    index.flush().await;
                }
            })
        };
        (index, writer)
    }

    /// The room's entries, read from the store the first time.
    async fn with_room<T>(
        &self,
        room_id: &str,
        f: impl FnOnce(&mut HashMap<String, IndexedMessage>) -> T,
    ) -> T {
        let mut rooms = self.rooms.lock().await;
        if !rooms.contains_key(room_id) {
            let stored: Vec<IndexedMessage> = self
                .client
                .state_store()
                .get_custom_value(&key(room_id))
                .await
                .ok()
                .flatten()
                .and_then(|bytes| serde_json::from_slice(&bytes).ok())
                .unwrap_or_default();
            rooms.insert(
                room_id.to_string(),
                stored
                    .into_iter()
                    .map(|e| (e.message_id.clone(), e))
                    .collect(),
            );
        }
        f(rooms.entry(room_id.to_string()).or_default())
    }

    /// `saveEntries`: adds or replaces these entries.
    pub async fn save(&self, room_id: &str, entries: Vec<IndexedMessage>) {
        if entries.is_empty() {
            return;
        }
        let changed = self
            .with_room(room_id, |room| {
                let mut changed = false;
                for entry in entries {
                    if room.get(&entry.message_id) != Some(&entry) {
                        room.insert(entry.message_id.clone(), entry);
                        changed = true;
                    }
                }
                changed
            })
            .await;
        if changed {
            self.dirty.lock().await.insert(room_id.to_string());
        }
    }

    async fn remove(&self, room_id: &str, message_id: &str) {
        if self
            .with_room(room_id, |room| room.remove(message_id).is_some())
            .await
        {
            self.dirty.lock().await.insert(room_id.to_string());
        }
    }

    /// Writes the changed rooms to the store.
    pub async fn flush(&self) {
        let dirty: Vec<String> = self.dirty.lock().await.drain().collect();
        for room_id in dirty {
            let bytes = {
                let rooms = self.rooms.lock().await;
                let Some(room) = rooms.get(&room_id) else {
                    continue;
                };
                serde_json::to_vec(&room.values().collect::<Vec<_>>()).unwrap_or_default()
            };
            let _ = self
                .client
                .state_store()
                .set_custom_value(&key(&room_id), bytes)
                .await;
        }
    }

    /// A message of the sync (decrypted when it was encrypted), or an edit of one.
    async fn feed_event(&self, room: &Room, event: &serde_json::Value) {
        let room_id = room.room_id().as_str();
        let content = &event["content"];
        let replaced = content
            .pointer("/m.relates_to/rel_type")
            .and_then(|r| r.as_str())
            == Some("m.replace");
        if replaced {
            // An edit: the message it replaces takes the new text
            let (Some(target), Some(new_content)) = (
                content
                    .pointer("/m.relates_to/event_id")
                    .and_then(|t| t.as_str()),
                content.get("m.new_content"),
            ) else {
                return;
            };
            let Some((text, has_file)) = text_of(new_content) else {
                return;
            };
            let updated = self
                .with_room(room_id, |entries| {
                    entries.get_mut(target).map(|entry| {
                        entry.text = text;
                        entry.has_file = has_file;
                    })
                })
                .await;
            if updated.is_some() {
                self.dirty.lock().await.insert(room_id.to_string());
            }
            return;
        }
        let sender = event["sender"].as_str().unwrap_or("");
        let name = sender_name(room, sender).await;
        if let Some(entry) = indexed_from_json(room_id, event, &name) {
            self.save(room_id, vec![entry]).await;
        }
    }

    /// The entry of one message, if this device has read it.
    pub async fn entry(&self, room_id: &str, message_id: &str) -> Option<IndexedMessage> {
        self.with_room(room_id, |room| room.get(message_id).cloned())
            .await
    }

    /// The entries of the room that match, newest first, at most 200.
    pub async fn search(&self, room_id: &str, filter: &SearchFilter) -> Vec<IndexedMessage> {
        let mut found = self
            .with_room(room_id, |room| {
                room.values()
                    .filter(|e| !twake_domain::search::is_local_echo_id(&e.message_id))
                    .filter(|e| matches_filter(e, filter))
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .await;
        found.sort_by(|a, b| b.sent_at.total_cmp(&a.sent_at));
        found.truncate(LIMIT);
        found
    }

    /// `MatrixServerSearch`: in a room that is not encrypted, the server's full-text search
    /// finds what this device never read; its results join the index.
    pub async fn ask_server(&self, room: &Room, query: &str) -> Result<(), String> {
        let room_id: OwnedRoomId = room.room_id().to_owned();
        let mut criteria = Criteria::new(query.to_string());
        let mut filter = RoomEventFilter::default();
        filter.rooms = Some(vec![room_id.clone()]);
        criteria.filter = filter;
        let mut categories = Categories::new();
        categories.room_events = Some(criteria);
        let response = self
            .client
            .send(Request::new(categories))
            .await
            .map_err(|e| e.to_string())?;
        let mut entries = Vec::new();
        for result in response.search_categories.room_events.results {
            let Some(raw) = result.result else { continue };
            let Ok(event) = raw.deserialize_as::<serde_json::Value>() else {
                continue;
            };
            if event["type"] != "m.room.message"
                || event
                    .pointer("/content/m.relates_to/rel_type")
                    .and_then(|r| r.as_str())
                    == Some("m.replace")
            {
                continue;
            }
            let name = sender_name(room, event["sender"].as_str().unwrap_or("")).await;
            entries.extend(indexed_from_json(room_id.as_str(), &event, &name));
        }
        self.save(room_id.as_str(), entries).await;
        Ok(())
    }
}

/// The display name of a member, else the localpart of their id.
pub(crate) async fn sender_name(room: &Room, sender: &str) -> String {
    let Ok(user_id) = OwnedUserId::try_from(sender) else {
        return sender.to_string();
    };
    match room.get_member_no_sync(&user_id).await {
        Ok(Some(member)) => member.name().to_string(),
        _ => crate::room_admin::localpart(sender),
    }
}

/// The text a search reads in a message content and whether it holds a file: the
/// caption and the name of a file (the body is the name when there is no caption).
fn text_of(content: &serde_json::Value) -> Option<(String, bool)> {
    let msgtype = content.get("msgtype")?.as_str()?;
    let body = content.get("body").and_then(|b| b.as_str()).unwrap_or("");
    let has_file = matches!(msgtype, "m.image" | "m.file" | "m.video" | "m.audio");
    if !has_file {
        return Some((body.to_string(), false));
    }
    let text = match content.get("filename").and_then(|f| f.as_str()) {
        Some(name) if name != body => format!("{body} {name}"),
        Some(name) => name.to_string(),
        None => body.to_string(),
    };
    Some((twake_domain::js::trim(&text).to_string(), true))
}

/// An `m.room.message` event as an index entry.
fn indexed_from_json(
    room_id: &str,
    event: &serde_json::Value,
    sender_name: &str,
) -> Option<IndexedMessage> {
    let (text, has_file) = text_of(&event["content"])?;
    Some(IndexedMessage {
        room_id: room_id.to_string(),
        message_id: event["event_id"].as_str()?.to_string(),
        sender_id: event["sender"].as_str()?.to_string(),
        sender_name: sender_name.to_string(),
        text,
        sent_at: event["origin_server_ts"].as_f64().unwrap_or(0.0),
        has_file,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_captions_and_file_names() {
        let file = serde_json::json!({"msgtype": "m.file", "body": "plan.pdf"});
        assert_eq!(text_of(&file), Some(("plan.pdf".into(), true)));
        let captioned =
            serde_json::json!({"msgtype": "m.image", "body": "the board", "filename": "a.png"});
        assert_eq!(text_of(&captioned), Some(("the board a.png".into(), true)));
        let text = serde_json::json!({"msgtype": "m.text", "body": "hello"});
        assert_eq!(text_of(&text), Some(("hello".into(), false)));
    }
}
