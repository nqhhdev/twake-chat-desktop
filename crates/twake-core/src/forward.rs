//! A message sent again to other conversations (web `saveForward`, `toForwardedContent`): the
//! same text or file (an encrypted file keeps its key), without what ties it to its room.

use matrix_sdk::Client;
use matrix_sdk::ruma::{EventId, RoomId};
use matrix_sdk_ui::timeline::Timeline;
use serde_json::{Map, Value};

/// Key of a bot command (web `BOT_COMMAND_KEY`, MSC4332).
const BOT_COMMAND_KEY: &str = "org.matrix.msc4332.command";
/// The prefix of every key the app writes (web `TWAKE_KEY_PREFIX`).
const TWAKE_KEY_PREFIX: &str = "app.twake.";

/// The quoted lines of a reply's fallback taken off its body (web `stripReplyFallback`).
pub(crate) fn strip_reply_fallback(body: &str) -> String {
    let lines: Vec<&str> = body.split('\n').collect();
    match lines.iter().position(|line| !line.starts_with('>')) {
        Some(quoted) if quoted > 0 && lines[quoted].is_empty() => lines[quoted + 1..].join("\n"),
        _ => body.to_string(),
    }
}

/// The `<mx-reply>` of a reply's formatted body taken off (web `stripHtmlReplyFallback`).
fn strip_html_reply_fallback(html: &str) -> String {
    if let Some(rest) = html.strip_prefix("<mx-reply>")
        && let Some(end) = rest.find("</mx-reply>")
    {
        return rest[end + "</mx-reply>".len()..].to_string();
    }
    html.to_string()
}

/// The content of a message sent again elsewhere: no reply, no edit, nobody mentioned again,
/// no command for a bot, none of the app's own marks (web `toForwardedContent`).
pub fn forwarded_content(content: &Map<String, Value>) -> Map<String, Value> {
    let has_relation = content.contains_key("m.relates_to");
    let mut forwarded: Map<String, Value> = content
        .iter()
        .filter(|(key, _)| {
            !matches!(
                key.as_str(),
                "m.relates_to" | "m.new_content" | "m.mentions" | BOT_COMMAND_KEY
            ) && !key.starts_with(TWAKE_KEY_PREFIX)
        })
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    forwarded.insert("m.mentions".into(), Value::Object(Map::new()));
    if has_relation && let Some(Value::String(body)) = forwarded.get("body") {
        let body = strip_reply_fallback(body);
        forwarded.insert("body".into(), Value::String(body));
    }
    if let Some(Value::String(html)) = forwarded.get("formatted_body") {
        let html = strip_html_reply_fallback(html);
        forwarded.insert("formatted_body".into(), Value::String(html));
    }
    forwarded
}

/// The content of a message as shown: its latest edit's new content, else its own (as
/// matrix-js-sdk's `getContent`), from the first of `timelines` that holds it.
async fn current_content(
    timelines: &[std::sync::Arc<Timeline>],
    event_id: &EventId,
) -> Result<Map<String, Value>, String> {
    let mut found = None;
    for timeline in timelines {
        if let Some(item) = timeline.item_by_event_id(event_id).await {
            found = Some(item);
            break;
        }
    }
    let item = found.ok_or("message_not_found")?;
    let content_of = |raw: &matrix_sdk::ruma::serde::Raw<_>| -> Option<Map<String, Value>> {
        let value: Value = raw.deserialize_as::<Value>().ok()?;
        value.get("content")?.as_object().cloned()
    };
    if let Some(edit) = item.latest_edit_json().and_then(content_of)
        && let Some(Value::Object(new_content)) = edit.get("m.new_content")
    {
        return Ok(new_content.clone());
    }
    item.original_json()
        .and_then(content_of)
        .ok_or_else(|| "message_not_found".to_string())
}

/// Sends the message `event_id`, of the room's or the thread's timeline, to each room of
/// `to`; the rooms it reached.
pub async fn forward(
    client: &Client,
    timelines: &[std::sync::Arc<Timeline>],
    event_id: &str,
    to: &[String],
) -> Result<Vec<String>, String> {
    let event_id = EventId::parse(event_id).map_err(|_| "message_not_found".to_string())?;
    let content = forwarded_content(&current_content(timelines, &event_id).await?);
    let mut sent = Vec::new();
    for room_id in to {
        let room = RoomId::parse(room_id)
            .ok()
            .and_then(|id| client.get_room(&id))
            .ok_or("room_not_found")?;
        room.send_raw("m.room.message", Value::Object(content.clone()))
            .await
            .map_err(|e| e.to_string())?;
        sent.push(room_id.clone());
    }
    Ok(sent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn map(value: Value) -> Map<String, Value> {
        value.as_object().cloned().expect("an object")
    }

    #[test]
    fn sends_the_message_without_its_ties() {
        let content = map(json!({
            "msgtype": "m.text",
            "body": "> <@a:b> quoted\n\nmy answer",
            "formatted_body": "<mx-reply><blockquote>q</blockquote></mx-reply>my <b>answer</b>",
            "m.relates_to": { "m.in_reply_to": { "event_id": "$x" } },
            "m.mentions": { "user_ids": ["@a:b"] },
            "org.matrix.msc4332.command": { "command": "x" },
            "app.twake.chat.call_thread": true
        }));
        assert_eq!(
            Value::Object(forwarded_content(&content)),
            json!({
                "msgtype": "m.text",
                "body": "my answer",
                "formatted_body": "my <b>answer</b>",
                "m.mentions": {}
            })
        );
    }

    #[test]
    fn keeps_a_body_that_is_not_a_reply() {
        let content = map(json!({ "msgtype": "m.text", "body": "> a quote\n\nnot a reply" }));
        assert_eq!(
            forwarded_content(&content)["body"],
            "> a quote\n\nnot a reply"
        );
    }
}
