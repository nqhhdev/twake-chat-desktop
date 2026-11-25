//! A room put in a space, or in none (web `adapters/matrix/spaceLinks.ts`): the links read
//! from both ends (`m.space.parent` of the room, `m.space.child` of the spaces), the missing
//! ones written, a restricted room's rule following its space, every other link removed.

use matrix_sdk::deserialized_responses::RawAnySyncOrStrippedState;
use matrix_sdk::ruma::api::client::state::send_state_event;
use matrix_sdk::ruma::api::error::ErrorKind;
use matrix_sdk::ruma::events::StateEventType;
use matrix_sdk::ruma::serde::Raw;
use matrix_sdk::ruma::{OwnedRoomId, RoomId};
use matrix_sdk::{Client, Room, RoomState};
use serde_json::{Value, json};
use twake_domain::space_choices::{SpaceChoice, SpaceNode, build_space_choices};

/// A space linked to the room (web `SpaceLink`): it lists the room (`m.space.child`), the
/// room declares it (`m.space.parent`), or both.
#[derive(Clone, Debug, PartialEq, Eq)]
struct SpaceLink {
    space: String,
    lists: bool,
    declares: bool,
}

fn content_of(state: RawAnySyncOrStrippedState) -> Option<Value> {
    let event: Value = match state {
        RawAnySyncOrStrippedState::Sync(raw) => raw.deserialize_as().ok()?,
        RawAnySyncOrStrippedState::Stripped(raw) => raw.deserialize_as().ok()?,
    };
    event.get("content").cloned()
}

/// A link counts when its content is not empty (an empty one is a removed link).
fn filled(content: Option<&Value>) -> bool {
    content
        .and_then(Value::as_object)
        .is_some_and(|o| !o.is_empty())
}

fn joined(client: &Client, id: &str) -> bool {
    RoomId::parse(id)
        .ok()
        .and_then(|id| client.get_room(&id))
        .is_some_and(|room| room.state() == RoomState::Joined)
}

/// The spaces the room declares (its non-empty `m.space.parent`).
async fn declared_by(room: &Room) -> Vec<String> {
    let Ok(states) = room.get_state_events(StateEventType::SpaceParent).await else {
        return Vec::new();
    };
    let mut spaces = Vec::new();
    for state in states {
        let event: Option<Value> = match state {
            RawAnySyncOrStrippedState::Sync(raw) => raw.deserialize_as().ok(),
            RawAnySyncOrStrippedState::Stripped(raw) => raw.deserialize_as().ok(),
        };
        if let Some(event) = event
            && filled(event.get("content"))
            && let Some(key) = event.get("state_key").and_then(Value::as_str)
        {
            spaces.push(key.to_string());
        }
    }
    spaces
}

/// Every space linked to the room, read from both ends: another client may write only one
/// of the two (web `spaceLinksOf`).
async fn space_links_of(client: &Client, room_id: &str) -> Vec<SpaceLink> {
    let declared = match RoomId::parse(room_id)
        .ok()
        .and_then(|id| client.get_room(&id))
    {
        Some(room) => declared_by(&room).await,
        None => Vec::new(),
    };
    let mut listing = Vec::new();
    for space in client.joined_rooms() {
        let state = space
            .get_state_event(StateEventType::SpaceChild, room_id)
            .await
            .ok()
            .flatten();
        if filled(state.and_then(content_of).as_ref()) {
            listing.push(space.room_id().to_string());
        }
    }
    let mut all: Vec<String> = declared.clone();
    for id in &listing {
        if !all.contains(id) {
            all.push(id.clone());
        }
    }
    all.into_iter()
        .map(|space| SpaceLink {
            lists: listing.contains(&space),
            declares: declared.contains(&space),
            space,
        })
        .collect()
}

/// False when the client knows the space and the user may not add to it; an unknown space
/// is left to the server (web `mayAddChildren`).
async fn may_add_children(client: &Client, space_id: &str) -> bool {
    let Some(space) = RoomId::parse(space_id)
        .ok()
        .and_then(|id| client.get_room(&id))
    else {
        return true;
    };
    if space.state() != RoomState::Joined {
        return true;
    }
    let own = space.own_user_id().to_owned();
    space
        .power_levels()
        .await
        .is_ok_and(|levels| levels.user_can_send_state(&own, StateEventType::SpaceChild))
}

/// The servers to reach a room through: the server of its id and the user's (web `viaFor`).
fn via_for(id: &str, own_server: &str) -> Vec<String> {
    let mut via = Vec::new();
    if let Some((_, server)) = id.split_once(':') {
        via.push(server.to_string());
    }
    if !via.iter().any(|s| s == own_server) {
        via.push(own_server.to_string());
    }
    via
}

/// A state event to write: room, type, state key, content. Soft: a refusal or a missing
/// room is ignored (a space the client has not joined).
struct Step {
    room: String,
    kind: StateEventType,
    key: String,
    content: Value,
    soft: bool,
}

fn hard(room: &str, kind: StateEventType, key: &str, content: Value) -> Step {
    Step {
        room: room.into(),
        kind,
        key: key.into(),
        content,
        soft: false,
    }
}

/// What is missing in the target (web `missingWrites`).
fn missing_writes(room_id: &str, space_id: &str, here: Option<&SpaceLink>, own: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    if !here.is_some_and(|l| l.lists) {
        steps.push(hard(
            space_id,
            StateEventType::SpaceChild,
            room_id,
            json!({ "via": via_for(room_id, own) }),
        ));
    }
    if !here.is_some_and(|l| l.declares) {
        steps.push(hard(
            room_id,
            StateEventType::SpaceParent,
            space_id,
            json!({ "via": via_for(space_id, own), "canonical": true }),
        ));
    }
    steps
}

/// The removal of a link of another space (web `removals`).
fn removals(client: &Client, room_id: &str, link: &SpaceLink) -> Vec<Step> {
    // A space the client does not know may list the room: clear it, best effort
    let blind = link.declares && !link.lists && !joined(client, &link.space);
    let child = Step {
        room: link.space.clone(),
        kind: StateEventType::SpaceChild,
        key: room_id.into(),
        content: json!({}),
        soft: blind,
    };
    let parent = hard(room_id, StateEventType::SpaceParent, &link.space, json!({}));
    let clears = if link.lists || blind {
        vec![child]
    } else {
        vec![]
    };
    let own = if link.declares { vec![parent] } else { vec![] };
    // Blind: the room's own link goes first, so a retry never depends on the child clear
    if blind {
        own.into_iter().chain(clears).collect()
    } else {
        clears.into_iter().chain(own).collect()
    }
}

const MEMBERSHIP: &str = "m.room_membership";

fn names(allow: &Value, id: &str) -> bool {
    allow.get("type").and_then(Value::as_str) == Some(MEMBERSHIP)
        && allow.get("room_id").and_then(Value::as_str) == Some(id)
}

/// The rule once the former spaces give way to the target: invite-only (or knock) when no
/// entry is left (web `followed`).
fn followed(rules: &Value, space_id: Option<&str>, former: &[String]) -> Value {
    let mut rest = rules.as_object().cloned().unwrap_or_default();
    let allow: Vec<Value> = rest
        .remove("allow")
        .and_then(|a| a.as_array().cloned())
        .unwrap_or_default();
    let mut next: Vec<Value> = allow
        .iter()
        .filter(|a| !former.iter().any(|id| names(a, id)))
        .cloned()
        .collect();
    if let Some(space) = space_id
        && !allow.iter().any(|a| names(a, space))
    {
        next.push(json!({ "type": MEMBERSHIP, "room_id": space }));
    }
    if next.is_empty() {
        let knock = rest.get("join_rule").and_then(Value::as_str) == Some("knock_restricted");
        rest.insert(
            "join_rule".into(),
            Value::String(if knock { "knock" } else { "invite" }.into()),
        );
    } else {
        rest.insert("allow".into(), Value::Array(next));
    }
    Value::Object(rest)
}

/// A restricted room follows its space: the entries naming a former space give way to the
/// target; skipped when the user may not change the rule (web `restrictionWrites`).
async fn restriction_writes(
    client: &Client,
    room_id: &str,
    space_id: Option<&str>,
    former: &[String],
) -> Vec<Step> {
    let Some(room) = RoomId::parse(room_id)
        .ok()
        .and_then(|id| client.get_room(&id))
    else {
        return Vec::new();
    };
    let Some(rules) = room
        .get_state_event(StateEventType::RoomJoinRules, "")
        .await
        .ok()
        .flatten()
        .and_then(content_of)
    else {
        return Vec::new();
    };
    let own = room.own_user_id().to_owned();
    let may_edit = room
        .power_levels()
        .await
        .is_ok_and(|levels| levels.user_can_send_state(&own, StateEventType::RoomJoinRules));
    let restricted = matches!(
        rules.get("join_rule").and_then(Value::as_str),
        Some("restricted" | "knock_restricted")
    );
    let names_former = rules
        .get("allow")
        .and_then(Value::as_array)
        .is_some_and(|allow| allow.iter().any(|a| former.iter().any(|id| names(a, id))));
    if !may_edit || !restricted || !names_former {
        return Vec::new();
    }
    vec![hard(
        room_id,
        StateEventType::RoomJoinRules,
        "",
        followed(&rules, space_id, former),
    )]
}

async fn send(client: &Client, step: Step) -> Result<(), String> {
    let room = OwnedRoomId::try_from(step.room.as_str()).map_err(|e| e.to_string())?;
    let content = Raw::new(&step.content)
        .map_err(|e| e.to_string())?
        .cast_unchecked();
    let request = send_state_event::v3::Request::new_raw(room, step.kind, step.key, content);
    match client.send(request).await {
        Ok(_) => Ok(()),
        Err(e) => {
            let gone = matches!(
                e.client_api_error_kind(),
                Some(ErrorKind::Forbidden | ErrorKind::NotFound)
            );
            if step.soft && gone {
                Ok(())
            } else {
                Err(e.to_string())
            }
        }
    }
}

/// Puts the room in the space (or in none) and converges: writes what is missing there,
/// then removes every other link, read from both ends; refuses before any write what the
/// client knows the user may not do (web `moveRoomToSpace`).
pub async fn move_room_to_space(
    client: &Client,
    room_id: &str,
    space_id: Option<&str>,
) -> Result<(), String> {
    let own = client
        .user_id()
        .map(|u| u.server_name().to_string())
        .unwrap_or_default();
    let links = space_links_of(client, room_id).await;
    let here = links.iter().find(|l| Some(l.space.as_str()) == space_id);
    let old: Vec<&SpaceLink> = links
        .iter()
        .filter(|l| Some(l.space.as_str()) != space_id)
        .collect();
    let mut touched: Vec<String> = old
        .iter()
        .filter(|l| l.lists)
        .map(|l| l.space.clone())
        .collect();
    if let Some(space) = space_id
        && !here.is_some_and(|l| l.lists && l.declares)
    {
        touched.push(space.to_string());
    }
    for id in &touched {
        if !may_add_children(client, id).await {
            return Err("forbidden".into());
        }
    }
    let former: Vec<String> = old.iter().map(|l| l.space.clone()).collect();
    let mut steps = match space_id {
        Some(space) => missing_writes(room_id, space, here, &own),
        None => Vec::new(),
    };
    steps.extend(restriction_writes(client, room_id, space_id, &former).await);
    for link in &old {
        steps.extend(removals(client, room_id, link));
    }
    for step in steps {
        send(client, step).await?;
    }
    Ok(())
}

/// The spaces the room may go to, for « Move to a Space » (web `buildSpaceChoices`): the
/// joined spaces, their parents read from both ends.
pub async fn space_choices(client: &Client, room_id: &str) -> Vec<SpaceChoice> {
    let mut nodes = Vec::new();
    let mut may = Vec::new();
    for space in client.joined_rooms().into_iter().filter(|r| r.is_space()) {
        let id = space.room_id().to_string();
        let parents = space_links_of(client, &id)
            .await
            .into_iter()
            .map(|l| l.space)
            .collect();
        let name = space
            .cached_display_name()
            .map(|n| n.to_string())
            .unwrap_or_else(|| id.clone());
        if may_add_children(client, &id).await {
            may.push(id.clone());
        }
        nodes.push(SpaceNode {
            id,
            name,
            parent_space_ids: parents,
        });
    }
    let current: Vec<String> = space_links_of(client, room_id)
        .await
        .into_iter()
        .map(|l| l.space)
        .collect();
    build_space_choices(&nodes, room_id, &current, |id| may.iter().any(|m| m == id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_restricted_room_follows_its_space() {
        let rules = json!({
            "join_rule": "restricted",
            "allow": [{ "type": MEMBERSHIP, "room_id": "!old:s" }]
        });
        assert_eq!(
            followed(&rules, Some("!new:s"), &["!old:s".into()]),
            json!({ "join_rule": "restricted", "allow": [{ "type": MEMBERSHIP, "room_id": "!new:s" }] })
        );
        // No space left: invite-only, or knock for a knock_restricted room
        assert_eq!(
            followed(&rules, None, &["!old:s".into()]),
            json!({ "join_rule": "invite" })
        );
        let knock = json!({ "join_rule": "knock_restricted", "allow": rules["allow"] });
        assert_eq!(
            followed(&knock, None, &["!old:s".into()]),
            json!({ "join_rule": "knock" })
        );
    }

    #[test]
    fn reaches_a_room_through_its_server_and_ours() {
        assert_eq!(via_for("!a:other.org", "me.org"), ["other.org", "me.org"]);
        assert_eq!(via_for("!a:me.org", "me.org"), ["me.org"]);
        // A room version 12 id has no server
        assert_eq!(via_for("!opaque", "me.org"), ["me.org"]);
    }
}
