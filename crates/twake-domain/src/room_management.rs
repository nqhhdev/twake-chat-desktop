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

/// What a new room is (web `NewRoomKind`): a group, a Broadcast feed, a space, a voice room
/// or a Broadcast with its page.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NewRoomKind {
    #[default]
    Discussion,
    Broadcast,
    Space,
    Voice,
    Page,
}

/// Who may follow a Broadcast (web `PageAccess`, D90): anyone (with its public page), on
/// invitation, or once an administrator approves the request.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PageAccess {
    #[default]
    Open,
    Invite,
    Approval,
}

/// What a new Broadcast adds to a room (web `NewPage`, D90).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NewPage {
    pub access: PageAccess,
    /// Followers react to its posts (`m.reaction` at level 0).
    pub allows_reactions: bool,
    /// Its address, `#slug`, and the public page `/b/slug`; open ones only.
    pub slug: String,
}

/// A room to create (web `NewRoom`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NewRoom {
    pub name: String,
    pub topic: String,
    pub kind: NewRoomKind,
    /// Created inside this space (a channel or a sub-space, D2b).
    pub parent_space_id: Option<String>,
    /// Marked `suggested` in its space: the General of a new space.
    pub is_suggested: bool,
    /// Anyone of the homeserver finds it in the directory and may join it (D42).
    pub is_public: bool,
    pub is_encrypted: bool,
    pub invite: Vec<String>,
    pub page: Option<NewPage>,
}

/// Characters of a name the user types (web `MAX_NAME_LENGTH`, D351).
pub const MAX_NAME_LENGTH: usize = 100;

/// The name, trimmed, is longer than [`MAX_NAME_LENGTH`] UTF-16 units (web `isNameTooLong`).
pub fn is_name_too_long(name: &str) -> bool {
    crate::js::trim(name).encode_utf16().count() > MAX_NAME_LENGTH
}

static SLUG: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new("^[a-z0-9](?:[a-z0-9-]{0,62}[a-z0-9])?$").expect("a valid pattern")
});

/// The address of a public page: lowercase letters, digits and dashes (web `isValidSlug`).
pub fn is_valid_slug(slug: &str) -> bool {
    SLUG.is_match(slug)
}

/// An address made of a name: « Le Figaro ! » gives `le-figaro` (web `suggestSlug`).
pub fn suggest_slug(name: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    let stripped: String = name
        .nfd()
        .filter(|c| !('\u{300}'..='\u{36f}').contains(c))
        .collect();
    let lower = stripped.to_lowercase();
    let mut dashed = String::with_capacity(lower.len());
    let mut in_run = false;
    for c in lower.chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            dashed.push(c);
            in_run = false;
        } else if !in_run {
            dashed.push('-');
            in_run = true;
        }
    }
    // ASCII only by now: 64 characters are 64 UTF-16 units
    let cut: String = dashed.chars().take(64).collect();
    cut.trim_matches('-').to_string()
}

#[cfg(test)]
mod creation_tests {
    use super::*;

    #[test]
    fn makes_an_address_of_a_name() {
        assert_eq!(suggest_slug("Le Figaro !"), "le-figaro");
        assert_eq!(suggest_slug("  Équipe   Ops  "), "equipe-ops");
        assert!(is_valid_slug("le-figaro"));
        assert!(!is_valid_slug("-le"));
        assert!(!is_valid_slug("Le"));
        assert!(is_name_too_long(&"a".repeat(101)));
        assert!(!is_name_too_long(&format!("  {}  ", "a".repeat(100))));
    }
}

/// What the user hears of a room (web `NotificationLevel`): every message, mentions only, or
/// nothing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationLevel {
    #[default]
    All,
    Mentions,
    None,
}
