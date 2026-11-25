//! The composer the user chose (web `domain/composerStyle.ts`, D681): `new`, one row of small
//! tools under the field, the default; or `classic`, the formats bar over the text.

/// The account data that keeps it, shared with the web (web `COMPOSER_STYLE_TYPE`).
pub const COMPOSER_STYLE_TYPE: &str = "app.twake.chat.composer_style";

/// The styles, in the web's order.
pub const COMPOSER_STYLES: [&str; 2] = ["new", "classic"];

/// The style in the content of its account data: the new composer when absent or malformed,
/// the safe default (web `readComposerStyle`).
pub fn read_composer_style(content: &serde_json::Value) -> &'static str {
    let style = content
        .as_object()
        .and_then(|o| o.get("style"))
        .and_then(|s| s.as_str());
    COMPOSER_STYLES
        .into_iter()
        .find(|name| Some(*name) == style)
        .unwrap_or("new")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reads_the_style_or_the_new_one() {
        assert_eq!(
            read_composer_style(&json!({ "style": "classic" })),
            "classic"
        );
        assert_eq!(read_composer_style(&json!({ "style": "other" })), "new");
        assert_eq!(read_composer_style(&json!(null)), "new");
    }
}
