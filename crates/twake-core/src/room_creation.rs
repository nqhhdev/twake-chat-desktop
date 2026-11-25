//! A new room of one kind (web `adapters/matrix/roomCreation.ts` and the `createRoom` use
//! case): a group, a Broadcast feed, a space with its General channel, a voice room, or a
//! Broadcast with its page.

use matrix_sdk::Client;
use matrix_sdk::ruma::api::client::room::Visibility;
use matrix_sdk::ruma::api::client::room::create_room::v3::{Request, RoomPreset};
use matrix_sdk::ruma::events::AnyInitialStateEvent;
use matrix_sdk::ruma::serde::Raw;
use matrix_sdk::ruma::{RoomId, UserId};
use serde_json::{Value, json};
use twake_domain::room_management::{
    NewPage, NewRoom, NewRoomKind, PageAccess, is_name_too_long, is_valid_slug,
};

use crate::calls::CALL_MEMBER_TYPE;
use crate::compose::BOT_COMMANDS_TYPES;

/// Types of the app's own (web `extensions.ts`).
const VOICE_ROOM_TYPE: &str = "app.twake.chat.voice_room";
const PAGE_ROOM_TYPE: &str = "app.twake.chat.page";
const POLL_RESPONSE: &str = "org.matrix.msc3381.poll.response";
const THREAD_RESOLVED: &str = "app.twake.chat.thread_resolved";
const ASSISTANT_CONSENT_TYPE: &str = "app.twake.chat.assistant_consent";
const PUBLIC_PAGE_STATE_TYPE: &str = "app.twake.chat.public_page";
const MODERATOR_LEVEL: i64 = 50;
const ADMIN_LEVEL: i64 = 100;

/// The servers to reach a room through: its id's server and the user's (web `viaFor`).
fn via_for(id: &str, own_server: &str) -> Vec<String> {
    let mut via: Vec<String> = id
        .split_once(':')
        .map(|(_, s)| s.to_string())
        .into_iter()
        .collect();
    if !own_server.is_empty() && !via.iter().any(|s| s == own_server) {
        via.push(own_server.to_string());
    }
    via
}

/// What the creator chose, as the options and the state need it (web `accessOf`): a public
/// room is never encrypted (D187), a Broadcast on approval is listed too.
struct Access {
    is_open_page: bool,
    is_public: bool,
    is_encrypted: bool,
    is_listed: bool,
}

fn access_of(room: &NewRoom, page: Option<&NewPage>) -> Access {
    let is_open_page = page.is_some_and(|p| p.access == PageAccess::Open);
    let is_public = room.is_public || is_open_page;
    Access {
        is_open_page,
        is_public,
        is_encrypted: room.is_encrypted && !is_public,
        is_listed: is_public || page.is_some_and(|p| p.access == PageAccess::Approval),
    }
}

/// The power levels a new room changes (web `powerLevelsOverride`): every member joins a
/// call, answers a bot, votes, resolves a thread, answers the assistant's request; only an
/// owner publishes the page; a feed or a Broadcast is written by its moderators.
fn power_levels(room: &NewRoom, page: Option<&NewPage>) -> Value {
    let mut events = serde_json::Map::new();
    for kind in [
        CALL_MEMBER_TYPE,
        POLL_RESPONSE,
        "m.poll.response",
        THREAD_RESOLVED,
        ASSISTANT_CONSENT_TYPE,
    ]
    .into_iter()
    .chain(BOT_COMMANDS_TYPES)
    {
        events.insert(kind.into(), json!(0));
    }
    events.insert(PUBLIC_PAGE_STATE_TYPE.into(), json!(ADMIN_LEVEL));
    if page.is_some_and(|p| p.allows_reactions) {
        events.insert("m.reaction".into(), json!(0));
    }
    let mut levels = json!({ "events": events });
    if room.kind == NewRoomKind::Broadcast || page.is_some() {
        levels["events_default"] = json!(MODERATOR_LEVEL);
    }
    levels
}

fn state(kind: &str, key: &str, content: Value) -> Value {
    json!({ "type": kind, "state_key": key, "content": content })
}

/// The state of a Broadcast: its public page and its access (web `pageState`, D90, D178).
fn page_state(page: Option<&NewPage>) -> Vec<Value> {
    match page.map(|p| p.access) {
        Some(PageAccess::Open) => vec![
            state(
                "m.room.history_visibility",
                "",
                json!({ "history_visibility": "world_readable" }),
            ),
            state(PUBLIC_PAGE_STATE_TYPE, "", json!({ "enabled": true })),
        ],
        Some(PageAccess::Approval) => {
            vec![state(
                "m.room.join_rules",
                "",
                json!({ "join_rule": "knock" }),
            )]
        }
        _ => Vec::new(),
    }
}

/// The link to the space, and the right to join for its members, except for a room open to
/// all or a Broadcast: its access is its own (web `parentState`, D688).
fn parent_state(parent: Option<&str>, keeps_own_access: bool, own_server: &str) -> Vec<Value> {
    let Some(parent) = parent else {
        return Vec::new();
    };
    let mut events = vec![state(
        "m.space.parent",
        parent,
        json!({ "via": via_for(parent, own_server), "canonical": true }),
    )];
    if !keeps_own_access {
        events.push(state(
            "m.room.join_rules",
            "",
            json!({ "join_rule": "restricted", "allow": [{ "type": "m.room_membership", "room_id": parent }] }),
        ));
    }
    events
}

/// The `createRoom` request of a new room (web `buildCreateRoomOptions`, D17): a channel and
/// a space are in the clear unless the creator asks (D187).
pub fn create_room_request(room: &NewRoom, own_server: &str) -> Result<Request, String> {
    let page = (room.kind == NewRoomKind::Page)
        .then_some(room.page.as_ref())
        .flatten();
    let access = access_of(room, page);
    let mut request = Request::new();
    request.name = Some(room.name.clone());
    request.topic = Some(room.topic.clone()).filter(|t| !t.is_empty());
    // Public: anyone may join, and the directory lists it (D42)
    request.preset = Some(if access.is_public {
        RoomPreset::PublicChat
    } else {
        RoomPreset::PrivateChat
    });
    if access.is_listed {
        request.visibility = Visibility::Public;
    }
    // The address of its public page, `/b/<slug>` (D90)
    if access.is_open_page {
        request.room_alias_name = page.map(|p| p.slug.clone());
    }
    let room_type = match room.kind {
        NewRoomKind::Space => Some("m.space"),
        NewRoomKind::Voice => Some(VOICE_ROOM_TYPE),
        NewRoomKind::Page => Some(PAGE_ROOM_TYPE),
        _ => None,
    };
    if let Some(kind) = room_type {
        request.creation_content = Some(
            Raw::from_json_string(json!({ "type": kind }).to_string())
                .map_err(|e| e.to_string())?,
        );
    }
    request.power_level_content_override = Some(
        Raw::from_json_string(power_levels(room, page).to_string()).map_err(|e| e.to_string())?,
    );
    let mut initial = Vec::new();
    if access.is_encrypted {
        initial.push(state(
            "m.room.encryption",
            "",
            json!({ "algorithm": "m.megolm.v1.aes-sha2" }),
        ));
    }
    initial.extend(page_state(page));
    initial.extend(parent_state(
        room.parent_space_id.as_deref(),
        access.is_public || page.is_some(),
        own_server,
    ));
    request.initial_state = initial
        .into_iter()
        .map(|event| Raw::<AnyInitialStateEvent>::from_json_string(event.to_string()))
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;
    request.invite = room
        .invite
        .iter()
        .map(|u| UserId::parse(u.trim()).map_err(|_| format!("« {u} » is not a Matrix id")))
        .collect::<Result<_, _>>()?;
    Ok(request)
}

/// Creates the room and puts it in its space; a room the space refuses is left and forgotten,
/// never left outside the space it was made for (web `saveRoom`, `addToSpace`).
async fn save_room(client: &Client, room: &NewRoom) -> Result<String, String> {
    let own_server = client
        .user_id()
        .map(|u| u.server_name().to_string())
        .unwrap_or_default();
    let created = client
        .create_room(create_room_request(room, &own_server)?)
        .await
        .map_err(|e| e.to_string())?;
    let room_id = created.room_id().to_string();
    if let Some(parent) = &room.parent_space_id {
        let mut content = json!({ "via": via_for(&room_id, &own_server) });
        if room.is_suggested {
            content["suggested"] = json!(true);
        }
        let added = match RoomId::parse(parent)
            .ok()
            .and_then(|id| client.get_room(&id))
        {
            Some(space) => space
                .send_state_event_raw("m.space.child", &room_id, content)
                .await
                .map(|_| ())
                .map_err(|e| e.to_string()),
            None => Err("the space is not joined".to_string()),
        };
        if let Err(e) = added {
            let _ = created.leave().await;
            let _ = created.forget().await;
            return Err(e);
        }
    }
    Ok(room_id)
}

/// A new room (web `makeCreateRoom`): its name trimmed, never empty nor too long, an open
/// Broadcast's address valid; a new space comes with its first channel, its General, open
/// to its members (D170).
pub async fn create(client: &Client, room: NewRoom) -> Result<String, String> {
    let name = twake_domain::js::trim(&room.name).to_string();
    if name.is_empty() {
        return Err("Give it a name.".into());
    }
    if is_name_too_long(&name) {
        return Err("The name is too long.".into());
    }
    if let Some(page) = &room.page
        && room.kind == NewRoomKind::Page
        && page.access == PageAccess::Open
        && !is_valid_slug(&page.slug)
    {
        return Err("The address takes lowercase letters, digits and dashes.".into());
    }
    let room = NewRoom {
        name: name.clone(),
        topic: twake_domain::js::trim(&room.topic).to_string(),
        ..room
    };
    let room_id = save_room(client, &room).await?;
    // A failure leaves the space, whose card offers a new group chat
    if room.kind == NewRoomKind::Space {
        let general = NewRoom {
            name,
            kind: NewRoomKind::Discussion,
            parent_space_id: Some(room_id.clone()),
            is_suggested: true,
            is_encrypted: room.is_encrypted,
            ..NewRoom::default()
        };
        let _ = save_room(client, &general).await;
    }
    Ok(room_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn room(kind: NewRoomKind) -> NewRoom {
        NewRoom {
            name: "Ops".into(),
            kind,
            ..NewRoom::default()
        }
    }

    #[test]
    fn a_feed_is_written_by_its_moderators() {
        let levels = power_levels(&room(NewRoomKind::Broadcast), None);
        assert_eq!(levels["events_default"], 50);
        assert_eq!(levels["events"][CALL_MEMBER_TYPE], 0);
        assert_eq!(levels["events"][PUBLIC_PAGE_STATE_TYPE], 100);
        assert!(
            power_levels(&room(NewRoomKind::Discussion), None)
                .get("events_default")
                .is_none()
        );
    }

    #[test]
    fn a_public_room_is_never_encrypted() {
        let public = NewRoom {
            is_public: true,
            is_encrypted: true,
            ..room(NewRoomKind::Discussion)
        };
        let access = access_of(&public, None);
        assert!(access.is_public && access.is_listed && !access.is_encrypted);
    }

    #[test]
    fn a_channel_of_a_space_is_open_to_its_members() {
        let events = parent_state(Some("!s:me.org"), false, "me.org");
        assert_eq!(events[0]["type"], "m.space.parent");
        assert_eq!(events[1]["content"]["join_rule"], "restricted");
        // A public room keeps its own access
        assert_eq!(parent_state(Some("!s:me.org"), true, "me.org").len(), 1);
    }

    #[test]
    fn an_open_broadcast_is_read_by_anyone() {
        let page = NewPage {
            access: PageAccess::Open,
            allows_reactions: true,
            slug: "ops".into(),
        };
        let events = page_state(Some(&page));
        assert_eq!(events[0]["content"]["history_visibility"], "world_readable");
        assert_eq!(
            power_levels(&room(NewRoomKind::Page), Some(&page))["events"]["m.reaction"],
            0
        );
    }
}
