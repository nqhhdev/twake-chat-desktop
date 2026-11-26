//! « Move to a Space » (web `domain/spaceChoices.ts`): the spaces a room may go to, and
//! whether it may move at all.

use std::collections::HashSet;

/// A space the user joined, and the spaces it sits in (web `Room.parentSpaceIds`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpaceNode {
    pub id: String,
    pub name: String,
    pub parent_space_ids: Vec<String>,
}

/// A space offered (web `SpaceChoice`): 0 deep for a top-level space, one more for each
/// space it sits in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpaceChoice {
    pub id: String,
    pub name: String,
    pub depth: usize,
    /// The room is in this space already.
    pub is_current: bool,
}

/// The spaces and sub-spaces (a tree read from their parents, depth first, in list order)
/// where the user may add children; a space the user may not add to is left out, its
/// sub-spaces keep their depth (web `buildSpaceChoices`). `room_id`: the room that moves,
/// `current`: the spaces it is in.
pub fn build_space_choices(
    spaces: &[SpaceNode],
    room_id: &str,
    current: &[String],
    can_add_children: impl Fn(&str) -> bool,
) -> Vec<SpaceChoice> {
    let spaces: Vec<&SpaceNode> = spaces.iter().filter(|s| s.id != room_id).collect();
    let ids: HashSet<&str> = spaces.iter().map(|s| s.id.as_str()).collect();
    let mut seen = HashSet::new();
    let mut choices = Vec::new();
    fn visit<'a>(
        space: &'a SpaceNode,
        depth: usize,
        spaces: &[&'a SpaceNode],
        current: &[String],
        can_add_children: &dyn Fn(&str) -> bool,
        seen: &mut HashSet<&'a str>,
        choices: &mut Vec<SpaceChoice>,
    ) {
        if !seen.insert(space.id.as_str()) {
            return;
        }
        if can_add_children(&space.id) {
            choices.push(SpaceChoice {
                id: space.id.clone(),
                name: space.name.clone(),
                depth,
                is_current: current.contains(&space.id),
            });
        }
        for child in spaces
            .iter()
            .filter(|c| c.parent_space_ids.contains(&space.id))
        {
            visit(
                child,
                depth + 1,
                spaces,
                current,
                can_add_children,
                seen,
                choices,
            );
        }
    }
    for space in &spaces {
        if !space
            .parent_space_ids
            .iter()
            .any(|id| ids.contains(id.as_str()))
        {
            visit(
                space,
                0,
                &spaces,
                current,
                &can_add_children,
                &mut seen,
                &mut choices,
            );
        }
    }
    // Spaces that hold each other have no top: start at the first of them
    for space in &spaces {
        visit(
            space,
            0,
            &spaces,
            current,
            &can_add_children,
            &mut seen,
            &mut choices,
        );
    }
    choices
}

/// A group, channel or Broadcast the user may edit; not a direct chat, a space or a room
/// managed by Twake Space (web `canMoveToSpace`).
pub fn can_move_to_space(
    is_direct: bool,
    is_space: bool,
    is_managed_by_space: bool,
    can_edit_settings: bool,
) -> bool {
    !is_direct && !is_space && !is_managed_by_space && can_edit_settings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn space(id: &str, parents: &[&str]) -> SpaceNode {
        SpaceNode {
            id: id.into(),
            name: id.to_uppercase(),
            parent_space_ids: parents.iter().map(|p| (*p).into()).collect(),
        }
    }

    fn shown(choices: &[SpaceChoice]) -> Vec<(String, usize, bool)> {
        choices
            .iter()
            .map(|c| (c.id.clone(), c.depth, c.is_current))
            .collect()
    }

    #[test]
    fn walks_the_tree_depth_first() {
        let spaces = [space("a", &[]), space("b", &["a"]), space("c", &[])];
        let choices = build_space_choices(&spaces, "!room", &["b".into()], |_| true);
        assert_eq!(
            shown(&choices),
            [
                ("a".into(), 0, false),
                ("b".into(), 1, true),
                ("c".into(), 0, false)
            ]
        );
    }

    #[test]
    fn leaves_out_a_space_the_user_may_not_add_to() {
        let spaces = [space("a", &[]), space("b", &["a"])];
        let choices = build_space_choices(&spaces, "!room", &[], |id| id != "a");
        // Its sub-space keeps its depth
        assert_eq!(shown(&choices), [("b".into(), 1, false)]);
    }

    #[test]
    fn spaces_holding_each_other_start_at_the_first() {
        let spaces = [space("a", &["b"]), space("b", &["a"])];
        let choices = build_space_choices(&spaces, "!room", &[], |_| true);
        assert_eq!(
            shown(&choices),
            [("a".into(), 0, false), ("b".into(), 1, false)]
        );
    }

    #[test]
    fn moves_only_what_the_user_may_edit() {
        assert!(can_move_to_space(false, false, false, true));
        assert!(!can_move_to_space(true, false, false, true));
        assert!(!can_move_to_space(false, true, false, true));
        assert!(!can_move_to_space(false, false, true, true));
        assert!(!can_move_to_space(false, false, false, false));
    }
}
