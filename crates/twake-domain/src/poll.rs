//! Port of `src/domain/poll.ts` (the parts the desktop needs; the tally is matrix-sdk's).

use serde::{Deserialize, Serialize};

pub const MIN_POLL_ANSWERS: usize = 2;
pub const MAX_POLL_ANSWERS: usize = 20;
pub const MAX_POLL_TEXT_LENGTH: usize = 150;

/// One answer of a poll with its count.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PollAnswerView {
    pub id: String,
    pub text: String,
    pub votes: u32,
    /// The signed-in user chose it.
    pub own: bool,
}

/// A poll as the timeline draws it (`Poll` of `poll.ts`, flattened).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PollView {
    pub question: String,
    pub answers: Vec<PollAnswerView>,
    pub max_selections: u32,
    /// Results shown before the end (`isDisclosed`).
    pub is_disclosed: bool,
    pub ended: bool,
    pub total_votes: u32,
    /// The signed-in user started it: they may end it.
    pub can_end: bool,
}

impl PollView {
    pub fn own_answer_ids(&self) -> Vec<String> {
        self.answers
            .iter()
            .filter(|a| a.own)
            .map(|a| a.id.clone())
            .collect()
    }
}

/// `pressAnswer`: the answers after pressing `answer_id`, or `None` to take the single vote
/// back. One choice: replace it. Several: toggle, keeping the last `max_selections`.
pub fn press_answer(
    max_selections: f64,
    own_answer_ids: &[String],
    answer_id: &str,
) -> Option<Vec<String>> {
    let has = own_answer_ids.iter().any(|id| id == answer_id);
    if max_selections <= 1.0 {
        return if has {
            None
        } else {
            Some(vec![answer_id.to_string()])
        };
    }
    if has {
        return Some(
            own_answer_ids
                .iter()
                .filter(|id| *id != answer_id)
                .cloned()
                .collect(),
        );
    }
    let mut all: Vec<String> = own_answer_ids.to_vec();
    all.push(answer_id.to_string());
    // `.slice(-n)`: the last n; n is a JS number (NaN keeps all, as slice(-NaN) is slice(0)).
    let keep = if max_selections.is_nan() {
        all.len()
    } else {
        (max_selections.floor() as usize).min(all.len())
    };
    Some(all.split_off(all.len() - keep))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presses_like_the_web() {
        let own = vec!["a".to_string()];
        assert_eq!(press_answer(1.0, &own, "a"), None);
        assert_eq!(press_answer(1.0, &own, "b"), Some(vec!["b".into()]));
        assert_eq!(
            press_answer(2.0, &own, "b"),
            Some(vec!["a".into(), "b".into()])
        );
        let two = vec!["a".to_string(), "b".to_string()];
        assert_eq!(
            press_answer(2.0, &two, "c"),
            Some(vec!["b".into(), "c".into()])
        );
        assert_eq!(press_answer(2.0, &two, "a"), Some(vec!["b".into()]));
    }
}
