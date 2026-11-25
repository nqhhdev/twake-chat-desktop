//! Media, links and files among the messages of a room (web `domain/sharedContent.ts`): the
//! tabs of its info. In an encrypted room only the client can tell, so this reads the messages
//! it holds: the tabs grow as the history is loaded.

use std::collections::HashSet;

use crate::Message;
use crate::attachment::AttachmentKind;
use crate::web_links::extract_links;

/// A web link of a message.
#[derive(Clone, Debug, PartialEq)]
pub struct SharedLink<'a> {
    pub message: &'a Message,
    pub url: String,
}

/// The three tabs, newest first; an item of `media` or `files` is a message with its
/// attachment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SharedContent<'a> {
    pub media: Vec<&'a Message>,
    pub links: Vec<SharedLink<'a>>,
    pub files: Vec<&'a Message>,
}

/// Images and videos are media, other attachments files; a message's links once each (web
/// `computeSharedContent`).
pub fn compute_shared_content(messages: &[Message]) -> SharedContent<'_> {
    let mut content = SharedContent::default();
    for message in messages.iter().rev() {
        if let Some(attachment) = &message.attachment {
            let is_media = matches!(
                attachment.kind,
                AttachmentKind::Image | AttachmentKind::Video
            );
            if is_media {
                content.media.push(message);
            } else {
                content.files.push(message);
            }
        }
        let mut seen = HashSet::new();
        for url in extract_links(&message.text) {
            if seen.insert(url.clone()) {
                content.links.push(SharedLink { message, url });
            }
        }
    }
    content
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attachment::Attachment;

    fn message(id: &str, text: &str, kind: Option<AttachmentKind>) -> Message {
        Message {
            id: id.into(),
            text: text.into(),
            attachment: kind.map(|kind| Attachment {
                kind,
                name: format!("{id}.bin"),
                mime_type: String::new(),
                size: None,
                is_encrypted: false,
                dimensions: None,
                duration: None,
                source: format!("mxc://x/{id}"),
                thumbnail_source: None,
                voice: None,
            }),
            ..Message::default()
        }
    }

    #[test]
    fn sorts_the_newest_first_by_kind() {
        let messages = [
            message("a", "see https://a.example and https://a.example", None),
            message("b", "", Some(AttachmentKind::Image)),
            message("c", "", Some(AttachmentKind::File)),
            message("d", "", Some(AttachmentKind::Video)),
        ];
        let content = compute_shared_content(&messages);
        let ids = |items: &[&Message]| items.iter().map(|m| m.id.clone()).collect::<Vec<_>>();
        assert_eq!(ids(&content.media), ["d", "b"]);
        assert_eq!(ids(&content.files), ["c"]);
        // A link once per message
        assert_eq!(content.links.len(), 1);
        assert_eq!(content.links[0].url, "https://a.example");
    }
}
