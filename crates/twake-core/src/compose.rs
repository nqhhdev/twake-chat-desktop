//! What the composer sends: text and emotes with mentions (web `mentionContent.ts`) and
//! bot commands (MSC4332, web `botCommands.ts`).

use matrix_sdk::ruma::OwnedUserId;
use matrix_sdk::ruma::events::Mentions;
use matrix_sdk::ruma::events::room::message::{
    EmoteMessageEventContent, FormattedBody, MessageType, TextMessageEventContent,
};
use twake_domain::composer::BotCommand;
use twake_domain::mention::{Mention, is_everyone_mention, is_room_mention};
use twake_domain::search::is_letter_or_number;

/// State types announcing the commands of a bot (unstable, then stable name).
pub const BOT_COMMANDS_TYPES: [&str; 2] = ["org.matrix.msc4332.commands", "m.bot.commands"];
/// Key of a message content naming the command it runs.
pub const BOT_COMMAND_KEY: &str = "org.matrix.msc4332.command";

/// `parseSyntax`: `gif {search} {words...}` is `gif`, `search`, `words` (variadic).
fn parse_syntax(syntax: &str) -> (String, Vec<String>, bool) {
    let mut words = twake_domain::js::trim(syntax).split(twake_domain::js::is_js_space);
    let name = words.next().unwrap_or("").to_string();
    let names: Vec<&str> = words
        .filter_map(|w| w.strip_prefix('{')?.strip_suffix('}'))
        .filter(|w| !w.is_empty())
        .collect();
    let variadic = names.last().is_some_and(|w| w.ends_with("..."));
    let names = names
        .iter()
        .map(|w| w.strip_suffix("...").unwrap_or(w).to_string())
        .collect();
    (name, names, variadic)
}

/// `readBotCommands` of one state event: `bot` is its state key.
pub fn read_bot_commands(bot: &str, content: &serde_json::Value) -> Vec<BotCommand> {
    let Some(entries) = content.get("commands").and_then(|c| c.as_array()) else {
        return Vec::new();
    };
    if bot.is_empty() {
        return Vec::new();
    }
    entries
        .iter()
        .filter_map(|entry| {
            let syntax = entry.get("syntax")?.as_str()?;
            let (name, argument_names, is_last_argument_variadic) = parse_syntax(syntax);
            if name.is_empty() {
                return None;
            }
            let description = entry
                .pointer("/description/m.text/0/body")
                .and_then(|b| b.as_str())
                .unwrap_or("")
                .to_string();
            Some(BotCommand {
                bot_user_id: bot.to_string(),
                name,
                syntax: syntax.to_string(),
                description,
                argument_names,
                is_last_argument_variadic,
            })
        })
        .collect()
}

/// `buildCommandContent`: the arguments after the command name, the last taking the rest.
pub fn command_content(text: &str, command: &BotCommand) -> serde_json::Value {
    let after_name = text
        .strip_prefix(['!', '/'])
        .map(|rest| {
            let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
            rest[end..].trim_start()
        })
        .unwrap_or(text);
    let words: Vec<&str> = after_name.split_whitespace().collect();
    let mut args = serde_json::Map::new();
    let count = command.argument_names.len();
    for (index, name) in command.argument_names.iter().enumerate() {
        let is_last = index + 1 == count;
        let value: &[&str] = if is_last {
            words.get(index..).unwrap_or(&[])
        } else {
            words.get(index..index + 1).unwrap_or(&[])
        };
        if value.is_empty() {
            continue;
        }
        if command.is_last_argument_variadic && is_last {
            args.insert(format!("{name}..."), serde_json::json!(value));
        } else {
            args.insert(name.clone(), serde_json::json!(value.join(" ")));
        }
    }
    serde_json::json!({ "syntax": command.syntax, "arguments": args })
}

fn escape_html(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

fn permalink(id: &str) -> String {
    format!("https://matrix.to/#/{}", id.replace('#', "%23"))
}

/// `linkNames`: each name of the formatted body as a matrix.to link, longest names
/// first, never glued to a word, never inside a link or code.
fn link_names(formatted: &str, linked: &[&Mention]) -> String {
    let mut names: Vec<(String, &str)> = linked
        .iter()
        .map(|m| (escape_html(&m.name), m.user_id.as_str()))
        .filter(|(name, _)| !name.is_empty())
        .collect();
    names.sort_by_key(|(name, _)| std::cmp::Reverse(name.len()));
    let mut out = String::with_capacity(formatted.len() * 2);
    let mut inside = 0i32;
    let mut i = 0;
    while i < formatted.len() {
        let rest = &formatted[i..];
        if rest.starts_with('<')
            && let Some(end) = rest.find('>')
        {
            let tag = &rest[..=end];
            let lower = tag.to_ascii_lowercase();
            if ["<a ", "<a>", "<code", "<pre"]
                .iter()
                .any(|t| lower.starts_with(t))
            {
                inside += 1;
            } else if ["</a>", "</code>", "</pre>"]
                .iter()
                .any(|t| lower.starts_with(t))
            {
                inside -= 1;
            }
            out.push_str(tag);
            i += tag.len();
            continue;
        }
        if rest.starts_with('&')
            && let Some(end) = rest.find(';')
            && end > 1
            && rest[1..end]
                .chars()
                .all(|c| c == '#' || c.is_alphanumeric() || c == '_')
        {
            out.push_str(&rest[..=end]);
            i += end + 1;
            continue;
        }
        let before = formatted[..i].chars().next_back();
        let found = (inside == 0)
            .then(|| {
                names.iter().find(|(name, _)| {
                    rest.starts_with(name.as_str()) && {
                        let first = name.chars().next().is_some_and(is_letter_or_number);
                        let last = name.chars().next_back().is_some_and(is_letter_or_number);
                        let after = rest[name.len()..].chars().next();
                        !(first && before.is_some_and(is_letter_or_number))
                            && !(last && after.is_some_and(is_letter_or_number))
                    }
                })
            })
            .flatten();
        if let Some((name, id)) = found {
            out.push_str(&format!("<a href=\"{}\">{name}</a>", permalink(id)));
            i += name.len();
            continue;
        }
        let c = rest.chars().next().unwrap_or(' ');
        out.push(c);
        i += c.len_utf8();
    }
    out
}

/// `buildTextContent`: the message type with its formatted body and the mentions to
/// set (`m.mentions` always, so servers never guess them from the body).
pub fn text_content(text: &str, mentions: &[Mention], is_emote: bool) -> (MessageType, Mentions) {
    let for_everyone = mentions.iter().any(is_everyone_mention);
    let linked: Vec<&Mention> = mentions
        .iter()
        .filter(|m| !is_everyone_mention(m))
        .collect();
    let people: Vec<OwnedUserId> = linked
        .iter()
        .filter(|m| !is_room_mention(m))
        .filter_map(|m| OwnedUserId::try_from(m.user_id.as_str()).ok())
        .collect();
    let mut set = Mentions::with_user_ids(people);
    set.room = for_everyone;
    let link = |formatted: Option<FormattedBody>| -> Option<FormattedBody> {
        if linked.is_empty() {
            return formatted;
        }
        let html = formatted
            .map(|f| f.body)
            .unwrap_or_else(|| escape_html(text).replace('\n', "<br>"));
        Some(FormattedBody::html(link_names(&html, &linked)))
    };
    let message = if is_emote {
        let mut content = EmoteMessageEventContent::markdown(text);
        content.formatted = link(content.formatted.take());
        MessageType::Emote(content)
    } else {
        let mut content = TextMessageEventContent::markdown(text);
        content.formatted = link(content.formatted.take());
        MessageType::Text(content)
    };
    (message, set)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mention(id: &str, name: &str) -> Mention {
        Mention {
            user_id: id.into(),
            name: name.into(),
        }
    }

    #[test]
    fn links_whole_names_outside_code() {
        let jean = mention("@jean:x", "Jean");
        let full = mention("@jl:x", "Jean Lepierre");
        let html = link_names(
            "Jean Lepierre, Jeanne and <code>Jean</code> Jean",
            &[&jean, &full],
        );
        assert_eq!(
            html,
            "<a href=\"https://matrix.to/#/@jl:x\">Jean Lepierre</a>, Jeanne and <code>Jean</code> \
             <a href=\"https://matrix.to/#/@jean:x\">Jean</a>"
        );
    }

    #[test]
    fn parses_commands() {
        let content = serde_json::json!({"commands": [
            {"syntax": "gif {search} {words...}", "description": {"m.text": [{"body": "A GIF"}]}},
            {"syntax": "  "}
        ]});
        let commands = read_bot_commands("@bot_a:x", &content);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name, "gif");
        assert_eq!(commands[0].argument_names, vec!["search", "words"]);
        assert!(commands[0].is_last_argument_variadic);
        assert_eq!(
            command_content("!gif cats very cute", &commands[0]),
            serde_json::json!({"syntax": "gif {search} {words...}",
                "arguments": {"search": "cats", "words...": ["very", "cute"]}})
        );
    }
}
