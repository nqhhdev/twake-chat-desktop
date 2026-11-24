//! Room details and administration: members and roles, permissions, leave, invite, remove,
//! ban, rename, topic, encryption (M3). Rules of the web's `roomManagement.ts` live in
//! `twake_domain::room_management`; this module reads and writes Matrix.

use matrix_sdk::{
    Room, RoomMemberships,
    ruma::{
        Int, OwnedUserId, UserId,
        events::{StateEventType, room::power_levels::UserPowerLevel},
    },
};
use twake_domain::room_management::{
    MemberRole, RoomMemberDetail, RoomPermissions, compute_role, level_of_role,
};
use twake_domain::{RoomDetails, RoomKind};

/// `BOT_ID_PATTERN` of the web's `botCommand.ts`.
fn is_bot_id(user_id: &str) -> bool {
    user_id.starts_with("@bot_") || user_id.starts_with("@hermes_")
}

fn level(level: UserPowerLevel) -> f64 {
    match level {
        UserPowerLevel::Infinite => f64::INFINITY,
        UserPowerLevel::Int(i) => i64::from(i) as f64,
        // A future variant: treat it as the default.
        _ => 0.0,
    }
}

/// Everything the room info pane shows.
pub async fn details(room: &Room) -> Result<RoomDetails, String> {
    let own = room.own_user_id().to_owned();
    let levels = room.power_levels().await.map_err(|e| e.to_string())?;
    let creators = room.creators().unwrap_or_default();
    let can_state = |kind: StateEventType| levels.user_can_send_state(&own, kind);
    let permissions = RoomPermissions {
        can_invite: levels.user_can_invite(&own),
        can_remove: levels.user_can_kick(&own),
        can_ban: levels.user_can_ban(&own),
        can_set_levels: can_state(StateEventType::RoomPowerLevels),
        read_only_level: i64::from(levels.events_default) as f64,
        default_level: i64::from(levels.users_default) as f64,
        can_redact: levels.user_can_redact_event_of_other(&own),
        can_edit_settings: can_state(StateEventType::RoomName),
        can_encrypt: can_state(StateEventType::RoomEncryption),
        can_set_join_rules: can_state(StateEventType::RoomJoinRules),
        can_add_children: can_state(StateEventType::SpaceChild),
        own_power_level: level(levels.for_user(&own)),
        can_mention_everyone: levels.user_can_trigger_room_notification(&own),
        is_managed_by_space: false,
        can_transfer_ownership: false,
        can_add_owner: false,
    };
    let members = room
        .members(RoomMemberships::JOIN | RoomMemberships::INVITE)
        .await
        .map_err(|e| e.to_string())?;
    let read_only_level = permissions.read_only_level;
    let mut details: Vec<RoomMemberDetail> = members
        .iter()
        .map(|m| {
            let power_level = level(m.power_level());
            let user_id = m.user_id().to_string();
            let is_creator = creators.iter().any(|c| c == m.user_id());
            RoomMemberDetail {
                name: m
                    .display_name()
                    .map(str::to_string)
                    .unwrap_or_else(|| localpart(&user_id)),
                matrix_name: None,
                avatar_url: m.avatar_url().map(|u| u.to_string()),
                power_level,
                role: compute_role(power_level),
                is_invited: m.membership()
                    == &matrix_sdk::ruma::events::room::member::MembershipState::Invite,
                is_bot: is_bot_id(&user_id),
                is_read_only: power_level < read_only_level,
                is_creator,
                is_owner: is_creator || power_level >= 100.0,
                user_id,
            }
        })
        .collect();
    // Owners and admins first, then by name, invited people last (as the web lists them).
    details.sort_by(|a, b| {
        a.is_invited
            .cmp(&b.is_invited)
            .then(b.power_level.total_cmp(&a.power_level))
            .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    let is_direct = room.is_direct().await.unwrap_or(false);
    Ok(RoomDetails {
        room_id: room.room_id().to_string(),
        name: room
            .cached_display_name()
            .map(|n| n.to_string())
            .unwrap_or_else(|| room.room_id().to_string()),
        topic: room.topic().filter(|t| !t.trim().is_empty()),
        avatar_url: room.avatar_url().map(|u| u.to_string()),
        kind: if is_direct {
            RoomKind::Direct
        } else {
            RoomKind::Discussion
        },
        is_encrypted: room.encryption_state().is_encrypted(),
        joined_count: room.joined_members_count(),
        own_user_id: own.to_string(),
        members: details,
        permissions,
    })
}

pub fn localpart(user_id: &str) -> String {
    user_id
        .trim_start_matches('@')
        .split(':')
        .next()
        .unwrap_or(user_id)
        .to_string()
}

fn user(user_id: &str) -> Result<OwnedUserId, String> {
    UserId::parse(user_id.trim())
        .map_err(|_| format!("« {user_id} » is not a Matrix id (@name:server)"))
}

pub async fn invite(room: &Room, user_id: &str) -> Result<(), String> {
    room.invite_user_by_id(&user(user_id)?)
        .await
        .map_err(|e| e.to_string())
}

pub async fn remove(room: &Room, user_id: &str, reason: Option<&str>) -> Result<(), String> {
    room.kick_user(&user(user_id)?, reason)
        .await
        .map_err(|e| e.to_string())
}

pub async fn ban(room: &Room, user_id: &str, reason: Option<&str>) -> Result<(), String> {
    room.ban_user(&user(user_id)?, reason)
        .await
        .map_err(|e| e.to_string())
}

pub async fn set_role(room: &Room, user_id: &str, role: MemberRole) -> Result<(), String> {
    let id = user(user_id)?;
    room.update_power_levels(vec![(&id, Int::from(level_of_role(role) as i32))])
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

pub async fn rename(room: &Room, name: &str) -> Result<(), String> {
    room.set_name(name.trim().to_string())
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

pub async fn set_topic(room: &Room, topic: &str) -> Result<(), String> {
    room.set_room_topic(topic.trim())
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

pub async fn encrypt(room: &Room) -> Result<(), String> {
    room.enable_encryption().await.map_err(|e| e.to_string())
}

pub async fn leave(room: &Room) -> Result<(), String> {
    room.leave().await.map_err(|e| e.to_string())
}
