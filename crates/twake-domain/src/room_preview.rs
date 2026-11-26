//! Port of `src/domain/roomPreview.ts`.

use crate::js;

const PREVIEW_MAX_LENGTH: usize = 200;

/// `toPreviewText`: one line per non-blank row, inner runs of spaces folded,
/// at most 200 UTF-16 code units, never half an emoji.
pub fn to_preview_text(text: &str) -> String {
    let rows: Vec<String> = text
        .split('\n')
        .map(|row| {
            let mut folded = String::with_capacity(row.len());
            let mut in_space = false;
            for c in row.chars() {
                if js::is_js_space(c) {
                    if !in_space {
                        folded.push(' ');
                    }
                    in_space = true;
                } else {
                    folded.push(c);
                    in_space = false;
                }
            }
            js::trim(&folded).to_string()
        })
        .filter(|row| !row.is_empty())
        .collect();
    let joined = rows.join("\n");
    // `slice_utf16` never keeps half a surrogate pair, which is what the
    // original's trailing `[\uD800-\uDBFF]$` removal achieves.
    js::slice_utf16(&joined, PREVIEW_MAX_LENGTH).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_rows_and_spaces() {
        assert_eq!(to_preview_text("  a   b \n\n  c\t\td  "), "a b\nc d");
        assert_eq!(to_preview_text("\r\n"), "");
    }

    #[test]
    fn caps_at_200_units_without_half_emoji() {
        let text = format!("{}🎉", "a".repeat(199));
        assert_eq!(to_preview_text(&text), "a".repeat(199));
    }
}
