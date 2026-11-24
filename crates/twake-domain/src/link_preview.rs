//! What the homeserver says of a web link (`preview_url`), read as the web reads it
//! (`adapters/matrix/linkPreview.ts`, `ds/LinkPreviewCard`).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LinkPreview {
    pub title: Option<String>,
    pub description: Option<String>,
    pub site_name: Option<String>,
    /// Only a picture the homeserver keeps, never one of the site itself.
    pub image_mxc: Option<String>,
    pub image_width: Option<f64>,
    pub image_height: Option<f64>,
}

fn text(value: Option<&serde_json::Value>) -> Option<String> {
    let s = crate::js::trim(value?.as_str()?);
    (!s.is_empty()).then(|| s.to_string())
}

/// A number, or its text: the tags of a page are strings, Synapse parses some.
fn size(value: Option<&serde_json::Value>) -> Option<f64> {
    let n = match value? {
        serde_json::Value::Number(n) => n.as_f64()?,
        serde_json::Value::String(s) if !s.is_empty() => {
            let t = crate::js::trim(s);
            if t.is_empty() { 0.0 } else { t.parse().ok()? }
        }
        _ => return None,
    };
    (n.is_finite() && n > 0.0).then_some(n)
}

/// `readLinkPreview`: the Open Graph tags of `preview_url`; None when the homeserver
/// found nothing to show.
pub fn read_link_preview(response: &serde_json::Value) -> Option<LinkPreview> {
    let image = text(response.get("og:image"));
    let preview = LinkPreview {
        title: text(response.get("og:title")),
        description: text(response.get("og:description")),
        site_name: text(response.get("og:site_name")),
        image_mxc: image.filter(|i| i.starts_with("mxc://")),
        image_width: size(response.get("og:image:width")),
        image_height: size(response.get("og:image:height")),
    };
    (preview.title.is_some() || preview.description.is_some() || preview.image_mxc.is_some())
        .then_some(preview)
}

/// `isWidePicture`: a banner (Open Graph's 1.91:1) goes on top of the card, any other
/// beside the text.
pub fn is_wide_picture(width: Option<f64>, height: Option<f64>) -> bool {
    matches!((width, height), (Some(w), Some(h)) if w > 0.0 && h > 0.0 && w / h >= 1.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_open_graph() {
        let found = read_link_preview(&serde_json::json!({
            "og:title": " Linagora ", "og:image": "https://x/y.png",
            "og:image:width": "1200", "og:image:height": 630
        }));
        assert_eq!(
            found,
            Some(LinkPreview {
                title: Some("Linagora".into()),
                image_width: Some(1200.0),
                image_height: Some(630.0),
                ..LinkPreview::default()
            })
        );
        assert_eq!(
            read_link_preview(&serde_json::json!({"og:image": "https://x"})),
            None
        );
        assert!(is_wide_picture(Some(1200.0), Some(630.0)));
        assert!(!is_wide_picture(Some(100.0), None));
    }
}
