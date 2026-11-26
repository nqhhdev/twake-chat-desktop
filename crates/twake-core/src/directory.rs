//! The public directory of the homeserver (web `readAllPublicRooms`, D42): the rooms anyone
//! may join or ask to join.

use matrix_sdk::Client;
use matrix_sdk::ruma::directory::PublicRoomsChunk;
use matrix_sdk::ruma::room::{JoinRuleKind, RoomType};
use matrix_sdk::ruma::{OwnedRoomOrAliasId, RoomId};

/// Rooms read in one request, and at most this many in all (web `BROWSE_PAGE_SIZE`,
/// `BROWSE_LIMIT`).
const PAGE_SIZE: u32 = 100;
const LIMIT: usize = 1000;
/// The room type of a Broadcast (web `PAGE_ROOM_TYPE`).
const PAGE_ROOM_TYPE: &str = "app.twake.chat.page";

/// A room of the directory (web `PublicRoom`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicRoom {
    pub room_id: String,
    pub alias: Option<String>,
    /// Its name, else its address, else its id (web `roomLabel`).
    pub name: String,
    pub topic: Option<String>,
    pub member_count: u64,
    /// On request (`knock`): joining sends a request.
    pub on_approval: bool,
    /// "space", "broadcast" or "channel".
    pub kind: &'static str,
}

fn to_public_room(room: PublicRoomsChunk) -> PublicRoom {
    let alias = room.canonical_alias.map(|a| a.to_string());
    let kind = match &room.room_type {
        Some(RoomType::Space) => "space",
        Some(other) if other.to_string() == PAGE_ROOM_TYPE => "broadcast",
        _ => "channel",
    };
    PublicRoom {
        name: room
            .name
            .clone()
            .or_else(|| alias.clone())
            .unwrap_or_else(|| room.room_id.to_string()),
        room_id: room.room_id.to_string(),
        alias,
        topic: room.topic,
        member_count: room.num_joined_members.into(),
        on_approval: room.join_rule == JoinRuleKind::Knock,
        kind,
    }
}

/// The whole directory, up to [`LIMIT`] rooms; the pages read so far when one fails.
pub async fn all(client: &Client) -> Vec<PublicRoom> {
    let mut rooms = Vec::new();
    let mut since: Option<String> = None;
    loop {
        let Ok(page) = client
            .public_rooms(Some(PAGE_SIZE), since.as_deref(), None)
            .await
        else {
            break;
        };
        rooms.extend(page.chunk.into_iter().map(to_public_room));
        since = page.next_batch;
        if since.is_none() || rooms.len() >= LIMIT {
            break;
        }
    }
    rooms.truncate(LIMIT);
    rooms
}

/// Joins a room of the directory, or asks to (a room on request): Ok(true) when joined.
pub async fn join(client: &Client, room_id: &str, on_approval: bool) -> Result<bool, String> {
    let id = RoomId::parse(room_id).map_err(|e| e.to_string())?;
    if on_approval {
        client
            .knock(OwnedRoomOrAliasId::from(id), None, Vec::new())
            .await
            .map_err(|e| e.to_string())?;
        Ok(false)
    } else {
        client
            .join_room_by_id(&id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(true)
    }
}
