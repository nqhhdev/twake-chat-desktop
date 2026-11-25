//! Port of the member and permission rules of `src/domain/roomManagement.ts`.

use serde::{Deserialize, Serialize};

pub const MODERATOR_LEVEL: i64 = 50;
pub const ADMIN_LEVEL: i64 = 100;
/// « Admin » is 80, below the owner who keeps 100 (D706).
pub const HANDED_OVER_OWNER_LEVEL: i64 = 80;

/// Who may join a room (web `JoinRuleChoice` and `RoomAccess.joinRule`): `Other` is a rule
/// the app tells but never sets (`restricted` to a space, D516).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JoinRuleChoice {
    #[default]
    Invite,
    Knock,
    Public,
    Other,
}

impl JoinRuleChoice {
    /// The line under the member count (web `info.accessLine`).
    pub fn access_line(self) -> &'static str {
        match self {
            Self::Invite => "Private · Invite only",
            Self::Knock => "Private · On request",
            Self::Public => "Public",
            Self::Other => "Space members",
        }
    }
}

/// `MemberRole`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemberRole {
    Admin,
    Moderator,
    Member,
}

impl MemberRole {
    pub fn as_str(self) -> &'static str {
        match self {
            MemberRole::Admin => "admin",
            MemberRole::Moderator => "moderator",
            MemberRole::Member => "member",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            MemberRole::Admin => "Admin",
            MemberRole::Moderator => "Moderator",
            MemberRole::Member => "Member",
        }
    }
}

/// `RoomMemberDetail`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomMemberDetail {
    pub user_id: String,
    pub name: String,
    #[serde(default)]
    pub matrix_name: Option<String>,
    pub avatar_url: Option<String>,
    pub power_level: f64,
    pub role: MemberRole,
    pub is_invited: bool,
    pub is_bot: bool,
    pub is_read_only: bool,
    pub is_creator: bool,
    pub is_owner: bool,
}

/// `RoomPermissions`: what the signed-in user may do in a room.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomPermissions {
    pub can_invite: bool,
    pub can_remove: bool,
    pub can_ban: bool,
    pub can_set_levels: bool,
    pub read_only_level: f64,
    pub default_level: f64,
    pub can_redact: bool,
    pub can_edit_settings: bool,
    pub can_encrypt: bool,
    pub can_set_join_rules: bool,
    pub can_add_children: bool,
    pub own_power_level: f64,
    pub can_mention_everyone: bool,
    pub is_managed_by_space: bool,
    pub can_transfer_ownership: bool,
    pub can_add_owner: bool,
}

/// `levelOfRole`.
pub fn level_of_role(role: MemberRole) -> i64 {
    match role {
        MemberRole::Admin => HANDED_OVER_OWNER_LEVEL,
        MemberRole::Moderator => MODERATOR_LEVEL,
        MemberRole::Member => 0,
    }
}

/// `computeRole`.
pub fn compute_role(power_level: f64) -> MemberRole {
    if power_level >= HANDED_OVER_OWNER_LEVEL as f64 {
        MemberRole::Admin
    } else if power_level >= MODERATOR_LEVEL as f64 {
        MemberRole::Moderator
    } else {
        MemberRole::Member
    }
}

/// `isRoomOwner`.
pub fn is_room_owner(permissions: &RoomPermissions) -> bool {
    permissions.can_invite && permissions.own_power_level >= ADMIN_LEVEL as f64
}

/// `canManageMember`: remove a member.
pub fn can_manage_member(
    member: &RoomMemberDetail,
    permissions: &RoomPermissions,
    is_self: bool,
) -> bool {
    !is_self && permissions.can_remove && member.power_level < permissions.own_power_level
}

/// `canChangeRole`.
pub fn can_change_role(
    member: &RoomMemberDetail,
    permissions: &RoomPermissions,
    is_self: bool,
) -> bool {
    !is_self
        && permissions.can_set_levels
        && !member.is_invited
        && !member.is_creator
        && member.power_level < permissions.own_power_level
}

/// `assignableRoles`: the roles the user may give this member, in the web's order.
pub fn assignable_roles(
    member: &RoomMemberDetail,
    permissions: &RoomPermissions,
    is_self: bool,
) -> Vec<MemberRole> {
    if !can_change_role(member, permissions, is_self) {
        return Vec::new();
    }
    [MemberRole::Admin, MemberRole::Moderator, MemberRole::Member]
        .into_iter()
        .filter(|role| {
            *role != member.role && level_of_role(*role) as f64 <= permissions.own_power_level
        })
        .collect()
}

/// `canBanMember`.
pub fn can_ban_member(
    member: &RoomMemberDetail,
    permissions: &RoomPermissions,
    is_self: bool,
) -> bool {
    !is_self
        && permissions.can_ban
        && !member.is_creator
        && member.power_level < permissions.own_power_level
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member(level: f64) -> RoomMemberDetail {
        RoomMemberDetail {
            user_id: "@bob:x".into(),
            name: "bob".into(),
            matrix_name: None,
            avatar_url: None,
            power_level: level,
            role: compute_role(level),
            is_invited: false,
            is_bot: false,
            is_read_only: false,
            is_creator: false,
            is_owner: false,
        }
    }

    #[test]
    fn roles_follow_the_web_levels() {
        assert_eq!(compute_role(80.0), MemberRole::Admin);
        assert_eq!(compute_role(79.0), MemberRole::Moderator);
        assert_eq!(level_of_role(MemberRole::Admin), 80);
        let me = RoomPermissions {
            can_set_levels: true,
            own_power_level: 100.0,
            ..Default::default()
        };
        assert_eq!(
            assignable_roles(&member(0.0), &me, false),
            vec![MemberRole::Admin, MemberRole::Moderator]
        );
        assert!(assignable_roles(&member(100.0), &me, false).is_empty());
    }
}
