//! The Rust port of `src/domain/`.
//!
//! Replace each `unimplemented!` with the port of the source function. Return `Err` where
//! the source throws or raises: the caller gets an error there too. After changing a
//! signature, run `rust-anything refresh` to regenerate the bindings. `rust-anything diff` checks
//! behavior against what the source recorded.

#![forbid(unsafe_code)]

/// Port of `buildActivity` (`src/domain/activity.ts:57`).
pub fn build_activity(
    mentions: Vec<serde_json::Value>,
    threads: Vec<serde_json::Value>,
    reactions: serde_json::Value,
    missed_calls: serde_json::Value,
) -> Result<serde_json::Value, String> {
    use twake_domain::activity::{ActivityMention, ActivityReaction, ActivityThread};
    // A default parameter (`= []`) takes the place of `undefined` only
    let list = |v: serde_json::Value| -> Result<Vec<serde_json::Value>, String> {
        match v {
            serde_json::Value::Array(items) => Ok(items),
            v if crate::js::is_undefined(&v) => Ok(Vec::new()),
            _ => Err("SKIP: input not of the typed shape (not an array)".into()),
        }
    };
    let reactions = list(reactions)?;
    let missed_calls = list(missed_calls)?;
    // Typed checks: the shapes the port reads (the objects themselves are kept)
    let _: Vec<ActivityMention> = typed(serde_json::Value::Array(mentions.clone()))?;
    let _: Vec<ActivityThread> = typed(serde_json::Value::Array(threads.clone()))?;
    let _: Vec<ActivityReaction> = typed(serde_json::Value::Array(reactions.clone()))?;
    let num = |v: Option<&serde_json::Value>| v.and_then(|n| n.as_f64());
    let followed: Vec<serde_json::Value> = threads
        .into_iter()
        .filter(|t| {
            twake_domain::activity::is_followed(
                t.pointer("/thread/isParticipating")
                    .and_then(|b| b.as_bool())
                    .unwrap_or(false),
                num(t.pointer("/thread/lastReply/sentAt")),
                num(t.pointer("/thread/lastOwnReplyAt")),
            )
        })
        .collect();
    Ok(serde_json::json!({
        "mentions": twake_domain::activity::newest_first(&mentions, |m: &serde_json::Value| num(m.pointer("/message/sentAt")).unwrap_or(f64::NAN)),
        "threads": twake_domain::activity::newest_first(&followed, |t: &serde_json::Value| num(t.pointer("/thread/lastReply/sentAt")).unwrap_or(0.0)),
        "reactions": twake_domain::activity::newest_first(&reactions, |r: &serde_json::Value| num(r.get("reactedAt")).unwrap_or(f64::NAN)),
        "missedCalls": twake_domain::activity::newest_first(&missed_calls, |c: &serde_json::Value| num(c.get("calledAt")).unwrap_or(f64::NAN)),
    }))
}

/// Port of `buildFeed` (`src/domain/activityFeed.ts:47`).
pub fn build_feed(
    activity: Option<serde_json::Value>,
    unread: Vec<serde_json::Value>,
    saved: Vec<serde_json::Value>,
) -> Result<Vec<serde_json::Value>, String> {
    use twake_domain::activity::{Activity, UnreadRoom};
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Input {
        mentions: Vec<twake_domain::activity::ActivityMention>,
        threads: Vec<twake_domain::activity::ActivityThread>,
        reactions: Vec<twake_domain::activity::ActivityReaction>,
        missed_calls: Vec<serde_json::Value>,
    }
    use serde::Deserialize;
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SavedInput {
        room_id: String,
        room_name: String,
        event_id: String,
        saved_at: f64,
        message: Option<SavedMessageInput>,
    }
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SavedMessageInput {
        sender_name: String,
        #[serde(deserialize_with = "Option::deserialize")]
        sender_avatar_url: Option<String>,
        text: String,
        attachment: Option<SavedAttachmentInput>,
    }
    #[derive(serde::Deserialize)]
    struct SavedAttachmentInput {
        name: String,
    }
    let saved: Vec<SavedInput> = typed(serde_json::Value::Array(saved))?;
    let saved: Vec<twake_domain::saved::SavedMessage> = saved
        .into_iter()
        .map(|s| twake_domain::saved::SavedMessage {
            room_id: s.room_id,
            room_name: s.room_name,
            event_id: s.event_id,
            saved_at: s.saved_at,
            message: s.message.map(|m| twake_domain::saved::SavedQuote {
                sender_name: m.sender_name,
                sender_avatar_url: m.sender_avatar_url,
                // `message.text || (message.attachment?.name ?? '')`
                text: if m.text.is_empty() {
                    m.attachment.map(|a| a.name).unwrap_or_default()
                } else {
                    m.text
                },
            }),
        })
        .collect();
    let activity = match activity {
        Some(value) if !crate::js::is_undefined(&value) => {
            let input: Input = typed(value)?;
            if !input.missed_calls.is_empty() {
                return Err(
                    "SKIP: input not of the typed shape (missed calls come with M5)".into(),
                );
            }
            Activity {
                mentions: input.mentions,
                threads: input.threads,
                reactions: input.reactions,
            }
        }
        _ => Activity::default(),
    };
    let unread: Vec<UnreadRoom> = typed(serde_json::Value::Array(unread))?;
    twake_domain::activity::build_feed(&activity, &unread, &saved)
        .iter()
        .map(|item| serde_json::to_value(item).map_err(|e| e.to_string()))
        .collect()
}

/// Port of `filterFeed` (`src/domain/activityFeed.ts:175`).
pub fn filter_feed(
    items: Vec<serde_json::Value>,
    filter: String,
) -> Result<Vec<serde_json::Value>, String> {
    let filter: twake_domain::activity::FeedFilter = typed(serde_json::json!(filter))?;
    let typed_items: Vec<twake_domain::activity::FeedItem> =
        typed(serde_json::Value::Array(items.clone()))?;
    let kinds = twake_domain::activity::kinds_of(filter);
    Ok(items
        .into_iter()
        .zip(typed_items)
        .filter(|(_, t)| kinds.contains(&t.kind))
        .map(|(v, _)| v)
        .collect())
}

/// Port of `searchFeed` (`src/domain/activityFeed.ts:183`).
pub fn search_feed(
    items: Vec<serde_json::Value>,
    query: String,
) -> Result<Vec<serde_json::Value>, String> {
    let typed_items: Vec<twake_domain::activity::FeedItem> =
        typed(serde_json::Value::Array(items.clone()))?;
    Ok(items
        .into_iter()
        .zip(typed_items)
        .filter(|(_, t)| {
            !twake_domain::activity::search_feed(std::slice::from_ref(t), &query).is_empty()
        })
        .map(|(v, _)| v)
        .collect())
}

/// Port of `groupByDay` (`src/domain/activityFeed.ts:204`).
pub fn group_by_day(items: serde_json::Value, now: f64) -> Result<Vec<serde_json::Value>, String> {
    use twake_domain::activity::Day;
    let serde_json::Value::Array(items) = items else {
        return Err("SKIP: input not of the typed shape (items is not an array)".into());
    };
    let typed_items: Vec<twake_domain::activity::FeedItem> =
        typed(serde_json::Value::Array(items.clone()))?;
    let groups = twake_domain::activity::group_by_day(&typed_items, now);
    let mut next = items.into_iter();
    Ok(groups
        .into_iter()
        .map(|(day, list)| {
            let day = match day {
                Day::Today => serde_json::json!("today"),
                Day::Yesterday => serde_json::json!("yesterday"),
                Day::Start(ms) => crate::js::number(ms),
            };
            let originals: Vec<serde_json::Value> = next.by_ref().take(list.len()).collect();
            serde_json::json!({"day": day, "items": originals})
        })
        .collect())
}

/// Port of `assistantConsentStatus` (`src/domain/assistantConsent.ts:41`).
pub fn assistant_consent_status(
    consent: serde_json::Value,
    assistant_id: String,
    member_ids: Vec<String>,
) -> Result<serde_json::Value, String> {
    let _ = (&consent, &assistant_id, &member_ids);
    unimplemented!("port `assistantConsentStatus` from src/domain/assistantConsent.ts")
}

/// Port of `requestsWaitingFor` (`src/domain/assistantConsent.ts:68`).
pub fn requests_waiting_for(
    consent: serde_json::Value,
    user_id: String,
    member_ids: Vec<String>,
) -> Result<Vec<serde_json::Value>, String> {
    let _ = (&consent, &user_id, &member_ids);
    unimplemented!("port `requestsWaitingFor` from src/domain/assistantConsent.ts")
}

/// Port of `peopleOf` (`src/domain/assistantConsent.ts:87`).
pub fn people_of(
    members: Vec<serde_json::Value>,
    consent: serde_json::Value,
) -> Result<Vec<String>, String> {
    let _ = (&members, &consent);
    unimplemented!("port `peopleOf` from src/domain/assistantConsent.ts")
}

/// Port of `presentRequest` (`src/domain/assistantConsent.ts:98`).
pub fn present_request(
    consent: serde_json::Value,
    members: Vec<serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    let _ = (&consent, &members);
    unimplemented!("port `presentRequest` from src/domain/assistantConsent.ts")
}

/// Port of `findOpenQuestion` (`src/domain/assistantQuestion.ts:56`).
pub fn find_open_question(
    messages: Vec<serde_json::Value>,
    threads: Vec<serde_json::Value>,
    asker_ids: Vec<String>,
    now: f64,
) -> Result<Option<serde_json::Value>, String> {
    let _ = (&messages, &threads, &asker_ids, &now);
    unimplemented!("port `findOpenQuestion` from src/domain/assistantQuestion.ts")
}

/// Port of `computeWaveform` (`src/domain/attachment.ts:41`).
pub fn compute_waveform(
    samples: serde_json::Value,
    points: Option<serde_json::Value>,
) -> Result<Vec<f64>, String> {
    let _ = (&samples, &points);
    unimplemented!("port `computeWaveform` from src/domain/attachment.ts")
}

/// Port of `isPlayableAudio` (`src/domain/attachment.ts:67`).
pub fn is_playable_audio(attachment: serde_json::Value) -> Result<bool, String> {
    // The recorded inputs are any JSON: read them with JavaScript's rules (`size <= MAX`
    // converts the size to a number), the typed rule is `twake_domain`'s
    let Some(fields) = attachment.as_object() else {
        // `null.kind` throws; any other primitive or an array has no `kind`
        return if attachment.is_null() {
            Err("TypeError: Cannot read properties of null (reading 'kind')".into())
        } else {
            Ok(false)
        };
    };
    let is_undefined = |v: Option<&serde_json::Value>| v.is_none_or(|v| v == "$undefined");
    let size_fits = match fields.get("size") {
        Some(serde_json::Value::Null) => true,
        Some(value) => js_to_number(value)
            .is_some_and(|n| n <= twake_domain::attachment::MAX_PREVIEW_BYTES as f64),
        None => false,
    };
    Ok(fields.get("kind").is_some_and(|k| k == "audio")
        && is_undefined(fields.get("voice"))
        && size_fits)
}

/// `Number(value)` of a primitive (None for NaN and objects).
fn js_to_number(value: &serde_json::Value) -> Option<f64> {
    match value {
        serde_json::Value::Bool(b) => Some(f64::from(u8::from(*b))),
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::String(s) => {
            let t = twake_domain::js::trim(s);
            if t.is_empty() {
                Some(0.0)
            } else {
                t.parse().ok().filter(|n: &f64| !n.is_nan())
            }
        }
        serde_json::Value::Array(a) if a.is_empty() => Some(0.0),
        // Numbers JSON cannot hold, as the recorder writes them
        serde_json::Value::Object(o) => match o.get("$num").and_then(|n| n.as_str()) {
            Some("Infinity") => Some(f64::INFINITY),
            Some("-Infinity") => Some(f64::NEG_INFINITY),
            Some("-0") => Some(-0.0),
            _ => None,
        },
        _ => None,
    }
}

/// Port of `isViewablePdf` (`src/domain/attachment.ts:79`).
pub fn is_viewable_pdf(attachment: serde_json::Value, can_show_pdf: bool) -> Result<bool, String> {
    // JavaScript's rules on any JSON (the typed rule is `twake_domain`'s): `&&` stops at
    // the first false, reading a field of null throws, `split` and `toLowerCase` need strings
    if !can_show_pdf {
        return Ok(false);
    }
    let Some(fields) = attachment.as_object() else {
        return if attachment.is_null() {
            Err("TypeError: Cannot read properties of null (reading 'kind')".into())
        } else {
            Ok(false)
        };
    };
    if fields.get("kind").is_none_or(|k| k != "file") {
        return Ok(false);
    }
    let read = |field: Option<&serde_json::Value>, method: &str, call: &str| match field {
        Some(serde_json::Value::String(s)) => Ok(s.clone()),
        None => Err(format!(
            "TypeError: Cannot read properties of undefined (reading '{method}')"
        )),
        Some(serde_json::Value::Null) => Err(format!(
            "TypeError: Cannot read properties of null (reading '{method}')"
        )),
        Some(_) => Err(format!("TypeError: {call} is not a function")),
    };
    let mime = read(fields.get("mimeType"), "split", "mimeType.split")?;
    if twake_domain::attachment::base_type(&mime) == "application/pdf" {
        return Ok(true);
    }
    let name = read(
        fields.get("name"),
        "toLowerCase",
        "attachment.name.toLowerCase",
    )?;
    Ok(name.to_lowercase().ends_with(".pdf"))
}

/// Port of `isPreviewable` (`src/domain/attachment.ts:92`).
pub fn is_previewable(attachment: serde_json::Value) -> Result<bool, String> {
    let _ = &attachment;
    unimplemented!("port `isPreviewable` from src/domain/attachment.ts")
}

/// Port of `isGif` (`src/domain/attachment.ts:131`).
pub fn is_gif(attachment: serde_json::Value) -> Result<bool, String> {
    match &attachment {
        serde_json::Value::Null => {
            Err("TypeError: Cannot read properties of null (reading 'mimeType')".into())
        }
        v if crate::js::is_undefined(v) => {
            Err("TypeError: Cannot read properties of undefined (reading 'mimeType')".into())
        }
        v => Ok(v.get("mimeType").and_then(|m| m.as_str()) == Some("image/gif")),
    }
}

/// Port of `computeAttachmentKind` (`src/domain/attachment.ts:135`).
pub fn compute_attachment_kind(mime_type: String) -> Result<String, String> {
    Ok(
        twake_domain::attachment::compute_attachment_kind(&mime_type)
            .as_str()
            .to_string(),
    )
}

/// Port of `inferMimeType` (`src/domain/attachment.ts:183`).
pub fn infer_mime_type(name: String, mime_type: String) -> Result<String, String> {
    Ok(twake_domain::attachment::infer_mime_type(&name, &mime_type))
}

/// Port of `isSvg` (`src/domain/attachment.ts:195`).
pub fn is_svg(mime_type: String) -> Result<bool, String> {
    let _ = &mime_type;
    unimplemented!("port `isSvg` from src/domain/attachment.ts")
}

/// Port of `blobSafeMimeType` (`src/domain/attachment.ts:228`).
pub fn blob_safe_mime_type(mime_type: String) -> Result<String, String> {
    let _ = &mime_type;
    unimplemented!("port `blobSafeMimeType` from src/domain/attachment.ts")
}

/// Port of `svgCanvasSize` (`src/domain/attachment.ts:240`).
pub fn svg_canvas_size(width: f64, height: f64) -> Result<serde_json::Value, String> {
    let _ = (&width, &height);
    unimplemented!("port `svgCanvasSize` from src/domain/attachment.ts")
}

/// Port of `thumbnailSize` (`src/domain/attachment.ts:261`).
pub fn thumbnail_size(
    media: serde_json::Value,
    dimensions: serde_json::Value,
) -> Result<Option<serde_json::Value>, String> {
    let _ = (&media, &dimensions);
    unimplemented!("port `thumbnailSize` from src/domain/attachment.ts")
}

/// Port of `botOf` (`src/domain/bot.ts:22`).
pub fn bot_of(answer: serde_json::Value) -> Result<Option<serde_json::Value>, String> {
    let _ = &answer;
    unimplemented!("port `botOf` from src/domain/bot.ts")
}

/// Port of `botIdOf` (`src/domain/bot.ts:27`).
pub fn bot_id_of(user_id: String) -> Result<String, String> {
    let _ = &user_id;
    unimplemented!("port `botIdOf` from src/domain/bot.ts")
}

/// Port of `ownsBot` (`src/domain/bot.ts:37`).
pub fn owns_bot(
    own_user_id: String,
    bot_user_id: String,
    my_bot_user_id: Option<String>,
) -> Result<bool, String> {
    let _ = (&own_user_id, &bot_user_id, &my_bot_user_id);
    unimplemented!("port `ownsBot` from src/domain/bot.ts")
}

/// Port of `offersCommandsOf` (`src/domain/bot.ts:51`).
pub fn offers_commands_of(
    own_user_id: String,
    bot_user_id: String,
    my_bot_user_id: Option<String>,
    creator_id: Option<String>,
) -> Result<bool, String> {
    let _ = (&own_user_id, &bot_user_id, &my_bot_user_id, &creator_id);
    unimplemented!("port `offersCommandsOf` from src/domain/bot.ts")
}

/// Port of `findCommandQuery` (`src/domain/botCommand.ts:32`).
pub fn find_command_query(text: String, caret: f64) -> Result<Option<serde_json::Value>, String> {
    let before = js_slice_to(&text, caret);
    Ok(twake_domain::composer::find_command_query(&before)
        .map(|query| serde_json::json!({"start": 0, "query": query})))
}

/// Port of `matchCommands` (`src/domain/botCommand.ts:43`).
pub fn match_commands(
    commands: Vec<serde_json::Value>,
    query: String,
) -> Result<Vec<serde_json::Value>, String> {
    let typed_commands: Vec<twake_domain::composer::BotCommand> =
        typed(serde_json::Value::Array(commands.clone()))?;
    let found = twake_domain::composer::match_commands(&typed_commands, &query);
    Ok(found
        .iter()
        .filter_map(|c| typed_commands.iter().position(|t| std::ptr::eq(t, *c)))
        .map(|i| commands[i].clone())
        .collect())
}

/// Port of `findLocalCommand` (`src/domain/botCommand.ts:73`).
pub fn find_local_command(text: String) -> Result<Option<serde_json::Value>, String> {
    Ok(twake_domain::composer::find_local_command(&text)
        .map(|_| serde_json::json!("discardsession")))
}

/// Port of `prepareCommandText` (`src/domain/botCommand.ts:88`).
pub fn prepare_command_text(
    text: String,
    commands: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let (commands, typed_commands) = commands_of(&text, commands)?;
    let p = twake_domain::composer::prepare_command_text(&text, &typed_commands);
    let command = p
        .command
        .as_ref()
        .and_then(|c| typed_commands.iter().position(|t| t == c))
        .map(|i| commands[i].clone())
        .unwrap_or(serde_json::Value::Null);
    Ok(
        serde_json::json!({"text": p.text, "command": command, "isEmote": p.is_emote, "unknownCommand": p.unknown_command}),
    )
}

/// Port of `prepareOutgoing` (`src/domain/botCommand.ts:129`).
pub fn prepare_outgoing(
    text: String,
    mentions: Vec<serde_json::Value>,
    commands: serde_json::Value,
) -> Result<serde_json::Value, String> {
    use twake_domain::composer::OutgoingError;
    let (commands, typed_commands) = commands_of(twake_domain::js::trim(&text), commands)?;
    let typed_mentions: Vec<twake_domain::mention::Mention> =
        typed(serde_json::Value::Array(mentions))?;
    let o = twake_domain::composer::prepare_outgoing(&text, &typed_mentions, &typed_commands);
    let command = o
        .command
        .as_ref()
        .and_then(|c| typed_commands.iter().position(|t| t == c))
        .map(|i| commands[i].clone())
        .unwrap_or(serde_json::Value::Null);
    let error = match o.error {
        Some(OutgoingError::EmptyMessage) => serde_json::json!("empty_message"),
        Some(OutgoingError::UnknownCommand) => serde_json::json!("unknown_command"),
        None => serde_json::Value::Null,
    };
    Ok(
        serde_json::json!({"error": error, "text": o.text, "mentions": o.mentions, "command": command, "isEmote": o.is_emote}),
    )
}

/// Port of `shouldMirror` (`src/domain/call.ts:70`).
pub fn should_mirror(tile: serde_json::Value) -> Result<serde_json::Value, String> {
    use serde_json::Value;
    // JavaScript's `&&` gives back `isLocal` itself when it is falsy; a primitive has no
    // fields (undefined); the typed rule is `twake_domain::call::should_mirror`
    let Some(fields) = tile.as_object() else {
        return if tile.is_null() {
            Err("TypeError: Cannot read properties of null (reading 'isLocal')".into())
        } else {
            Ok(crate::js::undefined())
        };
    };
    let field = |name: &str| fields.get(name).filter(|v| !crate::js::is_undefined(v));
    let truthy = |v: Option<&Value>| match v {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => !s.is_empty(),
        Some(v) => crate::js::as_number(v).is_none_or(|n| n != 0.0 && !n.is_nan()),
    };
    let is_local = field("isLocal");
    if !truthy(is_local) {
        return Ok(is_local.cloned().unwrap_or_else(crate::js::undefined));
    }
    Ok(Value::Bool(twake_domain::call::should_mirror(
        true,
        truthy(field("isScreen")),
        field("facingMode").is_some_and(|f| f == "environment"),
    )))
}

/// Port of `recordingModeOf` (`src/domain/call.ts:96`).
pub fn recording_mode_of(
    is_recording: bool,
    metadata: Option<String>,
) -> Result<Option<String>, String> {
    let _ = (&is_recording, &metadata);
    unimplemented!("port `recordingModeOf` from src/domain/call.ts")
}

/// Port of `currentDeviceId` (`src/domain/call.ts:143`).
/// A JSON value: `null` without a device, as the web (`?? null`), where `Option` would
/// come back as `undefined` from an absent `activeId`.
pub fn current_device_id(
    devices: Vec<serde_json::Value>,
    active_id: Option<String>,
) -> Result<serde_json::Value, String> {
    let devices: Vec<twake_domain::call::MediaDevice> = typed(serde_json::Value::Array(devices))?;
    Ok(
        twake_domain::call::current_device_id(&devices, active_id.as_deref())
            .map_or(serde_json::Value::Null, serde_json::Value::String),
    )
}

/// Port of `countCallPeople` (`src/domain/call.ts:156`).
pub fn count_call_people(members: serde_json::Value) -> Result<f64, String> {
    use serde_json::Value;
    let members = match members {
        Value::Array(members) => members,
        Value::Null => {
            return Err("TypeError: Cannot read properties of null (reading 'map')".into());
        }
        _ => return Err("TypeError: members.map is not a function".into()),
    };
    // A `Set` of the user ids: equal as JavaScript's SameValueZero (NaN is NaN, -0 is 0,
    // each object its own), the typed rule is `twake_domain::call::count_call_people`
    let mut keys = Vec::with_capacity(members.len());
    for (index, member) in members.iter().enumerate() {
        if member.is_null() || crate::js::is_undefined(member) {
            let what = if member.is_null() {
                "null"
            } else {
                "undefined"
            };
            return Err(format!(
                "TypeError: Cannot read properties of {what} (reading 'userId')"
            ));
        }
        let user = member.as_object().and_then(|o| o.get("userId"));
        keys.push(match user {
            None => "u".to_string(),
            Some(v) if crate::js::is_undefined(v) => "u".to_string(),
            Some(Value::Null) => "n".to_string(),
            Some(Value::Bool(b)) => format!("b{b}"),
            Some(Value::String(s)) => format!("s{s}"),
            Some(v) => match crate::js::as_number(v) {
                Some(n) if n.is_nan() => "dNaN".to_string(),
                Some(n) => format!("d{}", if n == 0.0 { 0.0 } else { n }),
                None => format!("o{index}"),
            },
        });
    }
    Ok(twake_domain::call::count_call_people(keys.iter().map(String::as_str)) as f64)
}

/// Port of `ringAsks` (`src/domain/call.ts:177`).
pub fn ring_asks(
    ring: Option<serde_json::Value>,
    members: serde_json::Value,
    own_user_id: String,
    is_direct: bool,
    now: f64,
) -> Result<bool, String> {
    // JavaScript's rules on any JSON; the typed rule is `twake_domain::call::ring_asks`
    if let Some(ring) = ring.filter(js_truthy) {
        let declined = ring.as_object().and_then(|o| o.get("isDeclined"));
        return Ok(!declined.is_some_and(js_truthy));
    }
    if !is_direct {
        return Ok(false);
    }
    let ring_ms = twake_domain::call::RING_MS as f64;
    js_some(&members, |m| {
        Ok(
            js_field(m, "userId")?.is_none_or(|u| u != own_user_id.as_str())
                && match js_field(m, "joinedAt")? {
                    None | Some(serde_json::Value::Null) => now + ring_ms > now,
                    Some(v) => js_plus_number(v, ring_ms) > now,
                },
        )
    })
}

/// JavaScript's truthiness of a JSON value.
fn js_truthy(value: &serde_json::Value) -> bool {
    use serde_json::Value;
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::String(s) => !s.is_empty(),
        v if crate::js::is_undefined(v) => false,
        v => crate::js::as_number(v).is_none_or(|n| n != 0.0 && !n.is_nan()),
    }
}

/// `value.field` (None: undefined); reading a field of null or undefined throws.
fn js_field<'a>(
    value: &'a serde_json::Value,
    field: &str,
) -> Result<Option<&'a serde_json::Value>, String> {
    if value.is_null() || crate::js::is_undefined(value) {
        let what = if value.is_null() { "null" } else { "undefined" };
        return Err(format!(
            "TypeError: Cannot read properties of {what} (reading '{field}')"
        ));
    }
    Ok(value
        .as_object()
        .and_then(|o| o.get(field))
        .filter(|v| !crate::js::is_undefined(v)))
}

/// `members.some(test)` with JavaScript's errors.
fn js_some(
    members: &serde_json::Value,
    mut test: impl FnMut(&serde_json::Value) -> Result<bool, String>,
) -> Result<bool, String> {
    match members {
        serde_json::Value::Array(items) => {
            for item in items {
                if test(item)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        serde_json::Value::Null => {
            Err("TypeError: Cannot read properties of null (reading 'some')".into())
        }
        v if crate::js::is_undefined(v) => {
            Err("TypeError: Cannot read properties of undefined (reading 'some')".into())
        }
        _ => Err("TypeError: members.some is not a function".into()),
    }
}

/// `value` as JavaScript compares it with a number (NaN when it is not one).
fn js_compare_number(value: &serde_json::Value) -> f64 {
    use serde_json::Value;
    match value {
        Value::Bool(b) => f64::from(u8::from(*b)),
        Value::Null => 0.0,
        Value::String(s) => js_string_number(s),
        v => crate::js::as_number(v).unwrap_or(f64::NAN),
    }
}

fn js_string_number(s: &str) -> f64 {
    let t = s.trim();
    if t.is_empty() {
        0.0
    } else {
        t.parse::<f64>()
            .ok()
            .filter(|_| !t.contains(['i', 'n', 'N', 'I']) || t.ends_with("Infinity"))
            .unwrap_or(f64::NAN)
    }
}

/// `value + n` compared as a number: a string gives a string (`"5" + 60000` is
/// `"560000"`), a number a sum.
fn js_plus_number(value: &serde_json::Value, n: f64) -> f64 {
    match value {
        serde_json::Value::String(s) => js_string_number(&format!("{s}{n}")),
        serde_json::Value::Bool(_) | serde_json::Value::Null => js_compare_number(value) + n,
        v if crate::js::as_number(v).is_some() => js_compare_number(v) + n,
        // An object or an array is joined to a string first: never a number here
        _ => f64::NAN,
    }
}

/// Port of `isRinging` (`src/domain/call.ts:201`).
pub fn is_ringing(
    members: serde_json::Value,
    own_user_id: String,
    is_direct: bool,
    is_joined: bool,
    left_at: Option<f64>,
) -> Result<bool, String> {
    // JavaScript's rules on any JSON; the typed rule is `twake_domain::call::is_ringing`
    if !is_direct || is_joined {
        return Ok(false);
    }
    let own = own_user_id.as_str();
    let other = js_some(&members, |m| {
        Ok(js_field(m, "userId")?.is_none_or(|u| u != own)
            && left_at.is_none_or(|left| {
                js_field(m, "joinedAt")
                    .ok()
                    .flatten()
                    .filter(|v| !v.is_null())
                    .map_or(f64::INFINITY, js_compare_number)
                    > left
            }))
    })?;
    Ok(other
        && !js_some(&members, |m| {
            Ok(js_field(m, "userId")?.is_some_and(|u| u == own))
        })?)
}

/// Port of `findCallThread` (`src/domain/call.ts:237`); the typed rule is
/// `twake_domain::call::find_call_thread`.
pub fn find_call_thread(
    messages: serde_json::Value,
    now: f64,
) -> Result<serde_json::Value, String> {
    use serde_json::Value;
    const MAX_AGE_MS: f64 = 12.0 * 60.0 * 60.0 * 1000.0;
    const RACE_MS: f64 = 30_000.0;
    let items = match &messages {
        Value::Array(items) => items,
        v if v.is_null() || crate::js::is_undefined(v) => {
            js_field(v, "filter")?;
            return Err("TypeError: messages.filter is not a function".into());
        }
        _ => return Err("TypeError: messages.filter is not a function".into()),
    };
    // `now - message.sentAt`: a missing time is NaN
    let sent = |m: &Value| -> Result<f64, String> {
        Ok(js_field(m, "sentAt")?.map_or(f64::NAN, js_compare_number))
    };
    let mut roots = Vec::new();
    for message in items {
        let is_root = js_field(message, "isCallThreadRoot")? == Some(&Value::Bool(true));
        if is_root && now - sent(message)? < MAX_AGE_MS {
            roots.push(message);
        }
    }
    let Some(newest) = roots.last() else {
        return Ok(Value::Null);
    };
    let newest_at = sent(newest)?;
    let mut first = None;
    for root in &roots {
        if newest_at - sent(root)? <= RACE_MS {
            first = Some(*root);
            break;
        }
    }
    // `first?.id ?? newest.id`
    let id = first
        .map(|r| js_field(r, "id"))
        .transpose()?
        .flatten()
        .filter(|v| !v.is_null());
    Ok(match id {
        Some(id) => id.clone(),
        None => js_field(newest, "id")?
            .cloned()
            .unwrap_or_else(crate::js::undefined),
    })
}

/// Port of `toCallLine` (`src/domain/callLine.ts:20`).
pub fn to_call_line(
    content: serde_json::Value,
    is_own: bool,
) -> Result<Option<serde_json::Value>, String> {
    // JavaScript's `typeof … === 'number'` takes NaN and the infinities too; the typed rule
    // is `twake_domain::call::to_call_line`
    let fields = match &content {
        serde_json::Value::Object(o) if !crate::js::is_undefined(&content) => o.clone(),
        serde_json::Value::Array(_) => serde_json::Map::new(),
        _ => return Ok(None),
    };
    let date = |name: &str| {
        fields
            .get(name)
            .filter(|v| !crate::js::is_undefined(v))
            .map(|v| crate::js::as_number(v).ok_or(()))
    };
    let is_video = match fields.get("intent").and_then(|i| i.as_str()) {
        Some("video") => true,
        Some("audio") => false,
        _ => return Ok(None),
    };
    let Some(Ok(ended)) = date("ended_ts") else {
        return Ok(None);
    };
    let direction = if is_own { "outgoing" } else { "incoming" };
    match date("answered_ts") {
        None => Ok(Some(serde_json::json!({
            "direction": direction,
            "isVideo": is_video,
            "outcome": if is_own { "canceled" } else { "missed" },
        }))),
        Some(Ok(answered)) if !(answered > ended) => Ok(Some(serde_json::json!({
            "direction": direction,
            "isVideo": is_video,
            "outcome": "answered",
            "durationMs": crate::js::number(ended - answered),
        }))),
        Some(_) => Ok(None),
    }
}

use twake_domain::call_window as cw;

/// A recorded object of numbers, typed: a missing or non-number field is another shape.
fn numbers<const N: usize>(value: &serde_json::Value, keys: [&str; N]) -> Result<[f64; N], String> {
    let mut out = [0.0; N];
    for (slot, key) in out.iter_mut().zip(keys) {
        *slot = value
            .get(key)
            .and_then(crate::js::as_number)
            .ok_or_else(|| format!("SKIP: input not of the typed shape ({key} is not a number)"))?;
    }
    Ok(out)
}

fn window_rect(value: &serde_json::Value) -> Result<cw::WindowRect, String> {
    let [x, y, width, height] = numbers(value, ["x", "y", "width", "height"])?;
    Ok(cw::WindowRect {
        x,
        y,
        width,
        height,
    })
}

fn viewport(value: &serde_json::Value) -> Result<cw::WindowViewport, String> {
    let [width, height] = numbers(value, ["width", "height"])?;
    Ok(cw::WindowViewport { width, height })
}

/// NaN, ±Infinity and -0 marked as `index.cjs` reads them.
fn window_json(rect: cw::WindowRect) -> serde_json::Value {
    use crate::js::number;
    serde_json::json!({
        "x": number(rect.x),
        "y": number(rect.y),
        "width": number(rect.width),
        "height": number(rect.height),
    })
}

/// A room id of `now`: a string or null; absent or of another type, another shape.
fn room_id_of(now: &serde_json::Value, key: &str) -> Result<Option<String>, String> {
    match now.get(key) {
        Some(serde_json::Value::String(id)) => Ok(Some(id.clone())),
        Some(serde_json::Value::Null) => Ok(None),
        _ => Err(format!("SKIP: input not of the typed shape ({key})")),
    }
}

/// Port of `fitWindow` (`src/domain/callWindow.ts:33`).
pub fn fit_window(
    rect: serde_json::Value,
    screen: serde_json::Value,
) -> Result<serde_json::Value, String> {
    Ok(window_json(cw::fit_window(
        window_rect(&rect)?,
        viewport(&screen)?,
    )))
}

/// Port of `defaultCallWindow` (`src/domain/callWindow.ts:47`).
pub fn default_call_window(screen: serde_json::Value) -> Result<serde_json::Value, String> {
    Ok(window_json(cw::default_call_window(viewport(&screen)?)))
}

/// Port of `moveWindow` (`src/domain/callWindow.ts:60`).
pub fn move_window(
    rect: serde_json::Value,
    dx: f64,
    dy: f64,
    screen: serde_json::Value,
) -> Result<serde_json::Value, String> {
    Ok(window_json(cw::move_window(
        window_rect(&rect)?,
        dx,
        dy,
        viewport(&screen)?,
    )))
}

/// Port of `placeInCorner` (`src/domain/callWindow.ts:69`).
pub fn place_in_corner(
    rect: serde_json::Value,
    corner: String,
    screen: serde_json::Value,
) -> Result<serde_json::Value, String> {
    Ok(window_json(cw::place_in_corner(
        window_rect(&rect)?,
        &corner,
        viewport(&screen)?,
    )))
}

/// Port of `resizeWindow` (`src/domain/callWindow.ts:105`).
pub fn resize_window(
    rect: serde_json::Value,
    edge: String,
    dx: f64,
    dy: f64,
    screen: serde_json::Value,
) -> Result<serde_json::Value, String> {
    Ok(window_json(cw::resize_window(
        window_rect(&rect)?,
        &edge,
        dx,
        dy,
        viewport(&screen)?,
    )))
}

/// Port of `parseCallWindow` (`src/domain/callWindow.ts:139`).
pub fn parse_call_window(stored: Option<String>) -> Result<Option<serde_json::Value>, String> {
    Ok(cw::parse_call_window(stored.as_deref()).map(window_json))
}

/// Port of `placeCall` (`src/domain/callWindow.ts:172`).
pub fn place_call(
    state: serde_json::Value,
    now: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let state: cw::CallPlacement = typed(state)?;
    let shown = room_id_of(&now, "shownRoomId")?;
    let call = room_id_of(&now, "callRoomId")?;
    serde_json::to_value(cw::place_call(&state, shown.as_deref(), call.as_deref()))
        .map_err(|e| e.to_string())
}

/// Port of `isCallFloating` (`src/domain/callWindow.ts:190`).
pub fn is_call_floating(
    state: serde_json::Value,
    call_room_id: serde_json::Value,
) -> Result<bool, String> {
    let state: cw::CallPlacement = typed(state)?;
    // `string | null`: an undefined room is another shape (`undefined !== null` in JS)
    let call_room_id = match call_room_id {
        serde_json::Value::String(id) => Some(id),
        serde_json::Value::Null => None,
        _ => return Err("SKIP: input not of the typed shape (callRoomId)".into()),
    };
    Ok(cw::is_call_floating(&state, call_room_id.as_deref()))
}

/// Port of `parseClientConfig` (`src/domain/clientConfig.ts:34`).
pub fn parse_client_config(raw: serde_json::Value) -> Result<serde_json::Value, String> {
    let _ = &raw;
    unimplemented!("port `parseClientConfig` from src/domain/clientConfig.ts")
}

/// Port of `readComposerStyle` (`src/domain/composerStyle.ts:14`).
pub fn read_composer_style(content: serde_json::Value) -> Result<serde_json::Value, String> {
    let _ = &content;
    unimplemented!("port `readComposerStyle` from src/domain/composerStyle.ts")
}

/// Port of `parseInvitationTarget` (`src/domain/contact.ts:35`).
pub fn parse_invitation_target(input: String) -> Result<Option<serde_json::Value>, String> {
    let _ = &input;
    unimplemented!("port `parseInvitationTarget` from src/domain/contact.ts")
}

/// Port of `groupByInitial` (`src/domain/contact.ts:51`).
pub fn group_by_initial(people: Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>, String> {
    let _ = &people;
    unimplemented!("port `groupByInitial` from src/domain/contact.ts")
}

/// Port of `parseContactsVisibility` (`src/domain/contactsVisibility.ts:10`).
pub fn parse_contacts_visibility(value: serde_json::Value) -> Result<Option<String>, String> {
    let _ = &value;
    unimplemented!("port `parseContactsVisibility` from src/domain/contactsVisibility.ts")
}

/// Port of `emptyCardOf` (`src/domain/conversationParts.ts:40`).
pub fn empty_card_of(facts: serde_json::Value) -> Result<Option<serde_json::Value>, String> {
    let _ = &facts;
    unimplemented!("port `emptyCardOf` from src/domain/conversationParts.ts")
}

/// Port of `footerOf` (`src/domain/conversationParts.ts:92`).
pub fn footer_of(facts: serde_json::Value) -> Result<Option<serde_json::Value>, String> {
    let _ = &facts;
    unimplemented!("port `footerOf` from src/domain/conversationParts.ts")
}

/// Port of `timelineHeadOf` (`src/domain/conversationParts.ts:142`).
pub fn timeline_head_of(facts: serde_json::Value) -> Result<serde_json::Value, String> {
    let _ = &facts;
    unimplemented!("port `timelineHeadOf` from src/domain/conversationParts.ts")
}

/// Port of `headOverWindow` (`src/domain/conversationParts.ts:171`).
pub fn head_over_window(
    head: serde_json::Value,
    hidden_above: f64,
) -> Result<serde_json::Value, String> {
    let _ = (&head, &hidden_above);
    unimplemented!("port `headOverWindow` from src/domain/conversationParts.ts")
}

/// Port of `isLandingRoom` (`src/domain/conversationView.ts:16`).
pub fn is_landing_room(room: Option<serde_json::Value>, is_embedded: bool) -> Result<bool, String> {
    let _ = (&room, &is_embedded);
    unimplemented!("port `isLandingRoom` from src/domain/conversationView.ts")
}

/// Port of `mayPostIn` (`src/domain/conversationView.ts:33`).
pub fn may_post_in(facts: serde_json::Value) -> Result<bool, String> {
    let _ = &facts;
    unimplemented!("port `mayPostIn` from src/domain/conversationView.ts")
}

/// Port of `isDirectRoom` (`src/domain/conversationView.ts:44`).
pub fn is_direct_room(
    room: Option<serde_json::Value>,
    direct_user_id: Option<String>,
) -> Result<bool, String> {
    let _ = (&room, &direct_user_id);
    unimplemented!("port `isDirectRoom` from src/domain/conversationView.ts")
}

/// Port of `peerStateOf` (`src/domain/conversationView.ts:73`).
pub fn peer_state_of(facts: serde_json::Value) -> Result<serde_json::Value, String> {
    let _ = &facts;
    unimplemented!("port `peerStateOf` from src/domain/conversationView.ts")
}

/// Port of `splitsCallChat` (`src/domain/conversationView.ts:97`): JavaScript's `&&`
/// gives the first falsy fact, or the last; the typed rule is
/// `twake_domain::call::splits_call_chat`.
pub fn splits_call_chat(facts: serde_json::Value) -> Result<serde_json::Value, String> {
    use serde_json::Value;
    let fact = |name| -> Result<Value, String> {
        Ok(js_field(&facts, name)?
            .cloned()
            .unwrap_or_else(crate::js::undefined))
    };
    let is_in_call = fact("isInCall")?;
    if !js_truthy(&is_in_call) {
        return Ok(is_in_call);
    }
    if js_truthy(&fact("isDirect")?) || js_truthy(&fact("isVoice")?) {
        return Ok(Value::Bool(false));
    }
    fact("mayPost")
}

/// Port of `isEncryptedConversation` (`src/domain/conversationView.ts:107`).
pub fn is_encrypted_conversation(room: Option<serde_json::Value>) -> Result<bool, String> {
    let _ = &room;
    unimplemented!("port `isEncryptedConversation` from src/domain/conversationView.ts")
}

/// Port of `isAwaitingRoom` (`src/domain/conversationView.ts:127`).
pub fn is_awaiting_room(facts: serde_json::Value) -> Result<bool, String> {
    let _ = &facts;
    unimplemented!("port `isAwaitingRoom` from src/domain/conversationView.ts")
}

/// Port of `isEmptyConversation` (`src/domain/conversationView.ts:146`).
pub fn is_empty_conversation(message_count: f64, has_outgoing: bool) -> Result<bool, String> {
    let _ = (&message_count, &has_outgoing);
    unimplemented!("port `isEmptyConversation` from src/domain/conversationView.ts")
}

/// Port of `isCardInlineIn` (`src/domain/conversationView.ts:159`).
pub fn is_card_inline_in(line_count: f64, has_predecessor: bool) -> Result<bool, String> {
    let _ = (&line_count, &has_predecessor);
    unimplemented!("port `isCardInlineIn` from src/domain/conversationView.ts")
}

/// Port of `roomNameOf` (`src/domain/conversationView.ts:170`).
pub fn room_name_of(sources: Vec<Option<serde_json::Value>>) -> Result<Option<String>, String> {
    let _ = &sources;
    unimplemented!("port `roomNameOf` from src/domain/conversationView.ts")
}

/// Port of `assistantOfferIn` (`src/domain/conversationView.ts:190`).
pub fn assistant_offer_in(facts: serde_json::Value) -> Result<Option<String>, String> {
    let _ = &facts;
    unimplemented!("port `assistantOfferIn` from src/domain/conversationView.ts")
}

/// Port of `hasAssistantIn` (`src/domain/conversationView.ts:214`).
pub fn has_assistant_in(
    room: Option<serde_json::Value>,
    consent: serde_json::Value,
    members: Vec<serde_json::Value>,
) -> Result<bool, String> {
    let _ = (&room, &consent, &members);
    unimplemented!("port `hasAssistantIn` from src/domain/conversationView.ts")
}

/// Port of `offersAddContact` (`src/domain/conversationView.ts:237`).
pub fn offers_add_contact(facts: serde_json::Value) -> Result<bool, String> {
    let _ = &facts;
    unimplemented!("port `offersAddContact` from src/domain/conversationView.ts")
}

/// Port of `matchPlaceOf` (`src/domain/conversationView.ts:260`).
pub fn match_place_of(
    message_ids: Vec<String>,
    focus_event_id: Option<String>,
) -> Result<serde_json::Value, String> {
    let _ = (&message_ids, &focus_event_id);
    unimplemented!("port `matchPlaceOf` from src/domain/conversationView.ts")
}

/// A recorded `DeletedChats`: an object of `{ before_ts: number }`; else another shape.
fn deleted_chats(
    value: &serde_json::Value,
) -> Result<twake_domain::deleted_chats::DeletedChats, String> {
    let skip = || "SKIP: input not of the typed shape (DeletedChats)".to_string();
    let entries = value
        .as_object()
        .filter(|_| crate::js::as_number(value).is_none())
        .ok_or_else(skip)?;
    entries
        .iter()
        .map(|(room_id, entry)| {
            let before = entry
                .get("before_ts")
                .and_then(crate::js::as_number)
                .ok_or_else(skip)?;
            Ok((room_id.clone(), before))
        })
        .collect()
}

/// Port of `readDeletedChats` (`src/domain/deletedChats.ts:13`).
pub fn read_deleted_chats(content: serde_json::Value) -> Result<serde_json::Value, String> {
    use serde_json::Value;
    // `typeof content === 'object'`: a marked number is a number
    let entries: Vec<(String, &Value)> = match &content {
        Value::Object(o) if crate::js::as_number(&content).is_none() => {
            o.iter().map(|(k, v)| (k.clone(), v)).collect()
        }
        Value::Array(a) => a
            .iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), v))
            .collect(),
        _ => Vec::new(),
    };
    let plain = twake_domain::deleted_chats::read_deleted_chats(&content);
    // NaN and ±Infinity are numbers too, which JSON cannot carry to the domain
    let marked = |entry: &Value| {
        entry
            .get("before_ts")
            .is_some_and(|b| b.is_object() && crate::js::as_number(b).is_some())
    };
    // The entries as they were given, as `Object.fromEntries` keeps them
    let kept: serde_json::Map<String, Value> = entries
        .into_iter()
        .filter(|(key, entry)| {
            crate::js::as_number(entry).is_none() && (plain.contains_key(key) || marked(entry))
        })
        .map(|(k, v)| (k, v.clone()))
        .collect();
    Ok(Value::Object(kept))
}

/// Port of `isChatHidden` (`src/domain/deletedChats.ts:26`).
pub fn is_chat_hidden(
    deleted: serde_json::Value,
    room_id: String,
    last_activity_at: f64,
) -> Result<bool, String> {
    let deleted = deleted_chats(&deleted)?;
    Ok(twake_domain::deleted_chats::is_chat_hidden(
        &deleted,
        &room_id,
        last_activity_at,
    ))
}

/// Port of `withoutDeleted` (`src/domain/deletedChats.ts:36`).
pub fn without_deleted(
    deleted: serde_json::Value,
    room_id: String,
    messages: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let deleted = deleted_chats(&deleted)?;
    // No entry: the very messages given back, whatever they are
    if !deleted.contains_key(&room_id) {
        return Ok(messages);
    }
    let serde_json::Value::Array(items) = &messages else {
        return Err("TypeError: messages.filter is not a function".into());
    };
    let typed_messages: Vec<twake_domain::Message> = typed(messages.clone())?;
    let kept = twake_domain::deleted_chats::without_deleted(&deleted, &room_id, &typed_messages);
    // The messages as they were given
    Ok(serde_json::Value::Array(
        kept.into_iter()
            .map(|m| {
                let index = typed_messages
                    .iter()
                    .position(|t| std::ptr::eq(t, m))
                    .expect("a message of the list");
                items[index].clone()
            })
            .collect(),
    ))
}

/// Port of `isAfterDeletion` (`src/domain/deletedChats.ts:48`).
pub fn is_after_deletion(
    deleted: serde_json::Value,
    room_id: String,
    at: f64,
) -> Result<bool, String> {
    let deleted = deleted_chats(&deleted)?;
    Ok(twake_domain::deleted_chats::is_after_deletion(
        &deleted, &room_id, at,
    ))
}

/// Port of `readDismissedQuestions` (`src/domain/dismissedQuestions.ts:12`).
pub fn read_dismissed_questions(content: serde_json::Value) -> Result<serde_json::Value, String> {
    let _ = &content;
    unimplemented!("port `readDismissedQuestions` from src/domain/dismissedQuestions.ts")
}

/// Port of `withDismissedQuestion` (`src/domain/dismissedQuestions.ts:22`).
pub fn with_dismissed_question(
    dismissed: serde_json::Value,
    id: String,
    closes_at: f64,
    now: f64,
) -> Result<serde_json::Value, String> {
    let _ = (&dismissed, &id, &closes_at, &now);
    unimplemented!("port `withDismissedQuestion` from src/domain/dismissedQuestions.ts")
}

/// Port of `findEmojiQuery` (`src/domain/emojiShortcode.ts:65`).
pub fn find_emoji_query(text: String, caret: f64) -> Result<Option<serde_json::Value>, String> {
    let before = js_slice_to(&text, caret);
    Ok(
        twake_domain::composer::find_emoji_query(&before).map(|query| {
            let start = caret - twake_domain::js::len_utf16(query) as f64 - 1.0;
            serde_json::json!({"start": crate::js::number(start), "query": query})
        }),
    )
}

/// Port of `isEmojiDrawable` (`src/domain/emojiSupport.ts:31`).
pub fn is_emoji_drawable(
    emoji: String,
    max: Option<serde_json::Value>,
    flags: Option<bool>,
) -> Result<bool, String> {
    let _ = (&emoji, &max, &flags);
    unimplemented!("port `isEmojiDrawable` from src/domain/emojiSupport.ts")
}

/// Port of `hasInk` (`src/domain/emojiSupport.ts:57`).
pub fn has_ink(pixels: serde_json::Value) -> Result<bool, String> {
    let _ = &pixels;
    unimplemented!("port `hasInk` from src/domain/emojiSupport.ts")
}

/// Port of `hasColour` (`src/domain/emojiSupport.ts:69`).
pub fn has_colour(pixels: serde_json::Value) -> Result<bool, String> {
    let _ = &pixels;
    unimplemented!("port `hasColour` from src/domain/emojiSupport.ts")
}

/// Port of `isDrawn` (`src/domain/emojiSupport.ts:88`).
pub fn is_drawn(sample: serde_json::Value, blank: serde_json::Value) -> Result<bool, String> {
    let _ = (&sample, &blank);
    unimplemented!("port `isDrawn` from src/domain/emojiSupport.ts")
}

/// Port of `undecryptableHelpKey` (`src/domain/encryption.ts:24`).
pub fn undecryptable_help_key(reason: String, is_device_verified: bool) -> Result<String, String> {
    let _ = (&reason, &is_device_verified);
    unimplemented!("port `undecryptableHelpKey` from src/domain/encryption.ts")
}

/// Port of `normalizeRecoveryKey` (`src/domain/encryption.ts:62`).
pub fn normalize_recovery_key(input: String) -> Result<Option<String>, String> {
    let _ = &input;
    unimplemented!("port `normalizeRecoveryKey` from src/domain/encryption.ts")
}

/// Port of `isNotificationService` (`src/domain/encryption.ts:142`).
pub fn is_notification_service(device: serde_json::Value) -> Result<bool, String> {
    let _ = &device;
    unimplemented!("port `isNotificationService` from src/domain/encryption.ts")
}

/// Port of `readFolders` (`src/domain/folders.ts:68`).
pub fn read_folders(content: serde_json::Value) -> Result<Vec<serde_json::Value>, String> {
    let _ = &content;
    unimplemented!("port `readFolders` from src/domain/folders.ts")
}

/// Port of `writeFolders` (`src/domain/folders.ts:106`).
pub fn write_folders(folders: Vec<serde_json::Value>) -> Result<serde_json::Value, String> {
    let _ = &folders;
    unimplemented!("port `writeFolders` from src/domain/folders.ts")
}

/// Port of `withHiddenBuiltins` (`src/domain/folders.ts:136`).
pub fn with_hidden_builtins(
    folders: Vec<serde_json::Value>,
) -> Result<Vec<serde_json::Value>, String> {
    let _ = &folders;
    unimplemented!("port `withHiddenBuiltins` from src/domain/folders.ts")
}

/// Port of `listFolders` (`src/domain/folders.ts:158`).
pub fn list_folders(
    saved: Vec<serde_json::Value>,
    entries: Vec<serde_json::Value>,
) -> Result<Vec<serde_json::Value>, String> {
    let _ = (&saved, &entries);
    unimplemented!("port `listFolders` from src/domain/folders.ts")
}

/// Port of `folderChatTypeOf` (`src/domain/folders.ts:179`).
pub fn folder_chat_type_of(room: serde_json::Value) -> Result<Option<String>, String> {
    let _ = &room;
    unimplemented!("port `folderChatTypeOf` from src/domain/folders.ts")
}

/// Port of `isInFolder` (`src/domain/folders.ts:196`).
pub fn is_in_folder(entry: serde_json::Value, folder: serde_json::Value) -> Result<bool, String> {
    let _ = (&entry, &folder);
    unimplemented!("port `isInFolder` from src/domain/folders.ts")
}

/// Port of `isRecommendationTaken` (`src/domain/folders.ts:208`).
pub fn is_recommendation_taken(
    r#type: String,
    folders: Vec<serde_json::Value>,
) -> Result<bool, String> {
    let _ = (&r#type, &folders);
    unimplemented!("port `isRecommendationTaken` from src/domain/folders.ts")
}

/// Port of `formatDuration` (`src/domain/format.ts:4`).
pub fn format_duration(milliseconds: f64) -> Result<String, String> {
    Ok(twake_domain::format::format_duration(milliseconds))
}

/// Port of `fitPreview` (`src/domain/format.ts:21`).
pub fn fit_preview(
    dimensions: serde_json::Value,
    max_side: f64,
    empty_ratio: Option<f64>,
) -> Result<serde_json::Value, String> {
    let dims = if coerce::is_falsy(&dimensions) {
        None
    } else {
        let side =
            |key: &str| coerce::to_number(dimensions.get(key).unwrap_or(&crate::js::undefined()));
        Some((side("width"), side("height")))
    };
    let (width, height) =
        twake_domain::format::fit_preview(dims, max_side, empty_ratio.unwrap_or(1.0));
    Ok(
        serde_json::json!({ "width": crate::js::number(width), "height": crate::js::number(height) }),
    )
}

/// Port of `isRecord` (`src/domain/json.ts:2`).
pub fn is_record(value: serde_json::Value) -> Result<bool, String> {
    let _ = &value;
    unimplemented!("port `isRecord` from src/domain/json.ts")
}

/// Port of `readText` (`src/domain/json.ts:7`).
pub fn read_text(source: serde_json::Value, key: String) -> Result<Option<String>, String> {
    let _ = (&source, &key);
    unimplemented!("port `readText` from src/domain/json.ts")
}

/// Port of `readTexts` (`src/domain/json.ts:16`).
pub fn read_texts(source: serde_json::Value, key: String) -> Result<Vec<String>, String> {
    let _ = (&source, &key);
    unimplemented!("port `readTexts` from src/domain/json.ts")
}

/// Port of `readBoolean` (`src/domain/json.ts:28`).
pub fn read_boolean(
    source: serde_json::Value,
    key: String,
    fallback: bool,
) -> Result<bool, String> {
    let _ = (&source, &key, &fallback);
    unimplemented!("port `readBoolean` from src/domain/json.ts")
}

/// Port of `isJumboEmoji` (`src/domain/jumboEmoji.ts:10`).
pub fn is_jumbo_emoji(text: String) -> Result<bool, String> {
    Ok(twake_domain::composer::is_jumbo_emoji(&text))
}

/// Port of `isLikelyIn` (`src/domain/languageGuess.ts:69`).
pub fn is_likely_in(text: String, prompt: String) -> Result<Option<bool>, String> {
    let _ = (&text, &prompt);
    unimplemented!("port `isLikelyIn` from src/domain/languageGuess.ts")
}

/// Port of `autoTranslationOf` (`src/domain/languageGuess.ts:81`).
pub fn auto_translation_of(
    message: serde_json::Value,
    prompt: Option<String>,
) -> Result<Option<String>, String> {
    let _ = (&message, &prompt);
    unimplemented!("port `autoTranslationOf` from src/domain/languageGuess.ts")
}

/// Port of `parseGeoUri` (`src/domain/location.ts:11`).
pub fn parse_geo_uri(uri: serde_json::Value) -> Result<Option<serde_json::Value>, String> {
    let _ = &uri;
    unimplemented!("port `parseGeoUri` from src/domain/location.ts")
}

/// Port of `openStreetMapUrl` (`src/domain/location.ts:20`).
pub fn open_street_map_url(arg0: serde_json::Value) -> Result<String, String> {
    let _ = &arg0;
    unimplemented!("port `openStreetMapUrl` from src/domain/location.ts")
}

/// Port of `avatarThumbnailSize` (`src/domain/mediaUrl.ts:10`).
pub fn avatar_thumbnail_size(size: f64) -> Result<f64, String> {
    let _ = &size;
    unimplemented!("port `avatarThumbnailSize` from src/domain/mediaUrl.ts")
}

/// Port of `thumbnailMediaUrl` (`src/domain/mediaUrl.ts:19`).
pub fn thumbnail_media_url(
    homeserver_url: String,
    mxc_url: String,
    size: f64,
    method: String,
) -> Result<Option<String>, String> {
    let _ = (&homeserver_url, &mxc_url, &size, &method);
    unimplemented!("port `thumbnailMediaUrl` from src/domain/mediaUrl.ts")
}

/// Port of `pickAttendees` (`src/domain/meeting.ts:16`).
pub fn pick_attendees(
    members: Vec<serde_json::Value>,
    address_book: Vec<serde_json::Value>,
    own_user_id: String,
) -> Result<serde_json::Value, String> {
    let _ = (&members, &address_book, &own_user_id);
    unimplemented!("port `pickAttendees` from src/domain/meeting.ts")
}

/// Port of `newEventUrl` (`src/domain/meeting.ts:34`).
pub fn new_event_url(calendar_url: String, emails: Vec<String>) -> Result<String, String> {
    let _ = (&calendar_url, &emails);
    unimplemented!("port `newEventUrl` from src/domain/meeting.ts")
}

/// Port of `readCreatedMeeting` (`src/domain/meeting.ts:64`).
pub fn read_created_meeting(value: serde_json::Value) -> Result<Option<serde_json::Value>, String> {
    let _ = &value;
    unimplemented!("port `readCreatedMeeting` from src/domain/meeting.ts")
}

/// Port of `meetingMessageText` (`src/domain/meeting.ts:89`).
pub fn meeting_message_text(
    meeting: serde_json::Value,
    lang: String,
    sender_zone: String,
) -> Result<String, String> {
    let _ = (&meeting, &lang, &sender_zone);
    unimplemented!("port `meetingMessageText` from src/domain/meeting.ts")
}

/// Port of `describeMeetingTime` (`src/domain/meetingTime.ts:10`).
pub fn describe_meeting_time(
    start_ts: f64,
    end_ts: f64,
    now: f64,
    language: String,
) -> Result<String, String> {
    let _ = (&start_ts, &end_ts, &now, &language);
    unimplemented!("port `describeMeetingTime` from src/domain/meetingTime.ts")
}

/// Port of `describePeople` (`src/domain/meetingTime.ts:39`).
pub fn describe_people(
    names: Vec<String>,
    total: Option<serde_json::Value>,
) -> Result<String, String> {
    let _ = (&names, &total);
    unimplemented!("port `describePeople` from src/domain/meetingTime.ts")
}

/// Port of `matchMembers` (`src/domain/mention.ts:19`).
pub fn match_members(
    members: Vec<serde_json::Value>,
    query: String,
    own_user_id: String,
    limit: Option<f64>,
) -> Result<Vec<serde_json::Value>, String> {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Member {
        user_id: String,
        name: String,
        #[serde(default)]
        matrix_name: Option<String>,
    }
    impl twake_domain::mention::Mentionable for Member {
        fn user_id(&self) -> &str {
            &self.user_id
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn matrix_name(&self) -> Option<&str> {
            self.matrix_name.as_deref()
        }
    }
    let typed_members: Vec<Member> = typed(serde_json::Value::Array(members.clone()))?;
    let found =
        twake_domain::mention::match_members(&typed_members, &query, &own_user_id, usize::MAX);
    let found = &found[..js_limit(limit, found.len())];
    Ok(found
        .iter()
        .filter_map(|m| typed_members.iter().position(|t| std::ptr::eq(t, *m)))
        .map(|i| members[i].clone())
        .collect())
}

/// Port of `mentionText` (`src/domain/mention.ts:56`).
pub fn mention_text(member: serde_json::Value) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Member {
        name: String,
        #[serde(default)]
        matrix_name: Option<String>,
    }
    let m: Member = typed(member)?;
    Ok(twake_domain::mention::mention_text(&m.name, m.matrix_name.as_deref()).to_string())
}

/// Port of `keepPresentMentions` (`src/domain/mention.ts:65`).
pub fn keep_present_mentions(
    text: String,
    mentions: Vec<serde_json::Value>,
) -> Result<Vec<serde_json::Value>, String> {
    let typed_mentions: Vec<twake_domain::mention::Mention> =
        typed(serde_json::Value::Array(mentions.clone()))?;
    let kept = twake_domain::mention::keep_present_mentions(&text, &typed_mentions);
    // The web keeps the objects it was given, first of each person
    Ok(kept
        .iter()
        .filter_map(|k| typed_mentions.iter().position(|t| t == k))
        .map(|i| mentions[i].clone())
        .collect())
}

/// Port of `namePattern` (`src/domain/mention.ts:89`).
pub fn name_pattern(name: String) -> Result<String, String> {
    let _ = &name;
    unimplemented!("port `namePattern` from src/domain/mention.ts")
}

/// Port of `matchRooms` (`src/domain/mention.ts:125`).
pub fn match_rooms(
    rooms: Vec<serde_json::Value>,
    query: String,
    limit: Option<f64>,
) -> Result<Vec<serde_json::Value>, String> {
    let typed_rooms: Vec<twake_domain::mention::LinkableRoom> =
        typed(serde_json::Value::Array(rooms.clone()))?;
    let found = twake_domain::mention::match_rooms(&typed_rooms, &query, usize::MAX);
    let found = &found[..js_limit(limit, found.len())];
    Ok(found
        .iter()
        .filter_map(|r| typed_rooms.iter().position(|t| std::ptr::eq(t, *r)))
        .map(|i| rooms[i].clone())
        .collect())
}

/// Port of `roomMentionRef` (`src/domain/mention.ts:152`).
pub fn room_mention_ref(room: serde_json::Value) -> Result<String, String> {
    let room: twake_domain::mention::LinkableRoom = typed(room)?;
    Ok(twake_domain::mention::room_mention_ref(&room).to_string())
}

/// Port of `roomMentionText` (`src/domain/mention.ts:157`).
pub fn room_mention_text(room: serde_json::Value) -> Result<String, String> {
    let room: twake_domain::mention::LinkableRoom = typed(room)?;
    Ok(twake_domain::mention::room_mention_text(&room))
}

/// Port of `isRoomMention` (`src/domain/mention.ts:162`).
pub fn is_room_mention(mention: serde_json::Value) -> Result<bool, String> {
    let _ = &mention;
    unimplemented!("port `isRoomMention` from src/domain/mention.ts")
}

/// Port of `isEveryoneMention` (`src/domain/mention.ts:170`).
pub fn is_everyone_mention(mention: serde_json::Value) -> Result<bool, String> {
    let _ = &mention;
    unimplemented!("port `isEveryoneMention` from src/domain/mention.ts")
}

/// Port of `offersEveryone` (`src/domain/mention.ts:179`).
pub fn offers_everyone(query: String) -> Result<bool, String> {
    Ok(twake_domain::mention::offers_everyone(&query))
}

/// Port of `describeEmote` (`src/domain/message.ts:76`).
pub fn describe_emote(sender_name: String, text: String) -> Result<String, String> {
    Ok(twake_domain::format::describe_emote(&sender_name, &text))
}

/// Port of `bareEmoji` (`src/domain/message.ts:109`).
pub fn bare_emoji(emoji: String) -> Result<String, String> {
    let _ = &emoji;
    unimplemented!("port `bareEmoji` from src/domain/message.ts")
}

/// Port of `countsAsUnread` (`src/domain/message.ts:158`).
pub fn counts_as_unread(message: serde_json::Value) -> Result<bool, String> {
    let _ = &message;
    unimplemented!("port `countsAsUnread` from src/domain/message.ts")
}

/// Port of `findFirstUnreadId` (`src/domain/message.ts:169`).
pub fn find_first_unread_id(
    messages: Vec<serde_json::Value>,
    unread_count: f64,
) -> Result<Option<String>, String> {
    let _ = (&messages, &unread_count);
    unimplemented!("port `findFirstUnreadId` from src/domain/message.ts")
}

/// Port of `isSameDay` (`src/domain/message.ts:202`).
pub fn is_same_day(a: f64, b: f64) -> Result<bool, String> {
    let _ = (&a, &b);
    unimplemented!("port `isSameDay` from src/domain/message.ts")
}

/// Port of `isSelectable` (`src/domain/message.ts:207`).
pub fn is_selectable(message: serde_json::Value) -> Result<bool, String> {
    use twake_domain::{MessageNotice, SendStatus};
    // The three fields the rule reads, typed; the app's `Message` is a subset of the web's,
    // so the rest of the recorded message is not read
    let skip = |field: &str| format!("SKIP: input not of the typed shape ({field})");
    let send_status = match message.get("sendStatus").and_then(serde_json::Value::as_str) {
        Some("sent") => SendStatus::Sent,
        Some("sending") => SendStatus::Sending,
        Some("failed") => SendStatus::Failed,
        _ => return Err(skip("sendStatus")),
    };
    // Any notice but « deleted » (unsupported, a missed or legacy call) reads the same here
    let notice = match message.get("notice") {
        Some(serde_json::Value::Null) => None,
        Some(serde_json::Value::String(n)) if n == "deleted" => Some(MessageNotice::Deleted),
        Some(serde_json::Value::String(_)) => Some(MessageNotice::Unsupported),
        _ => return Err(skip("notice")),
    };
    let is_call = message.get("call").is_some_and(|c| !coerce::is_falsy(c));
    let typed_message = twake_domain::Message {
        send_status: Some(send_status),
        notice,
        ..twake_domain::Message::default()
    };
    Ok(twake_domain::model::is_selectable(&typed_message, is_call))
}

/// Port of `startsGroup` (`src/domain/message.ts:218`).
pub fn starts_group(messages: Vec<serde_json::Value>, index: f64) -> Result<bool, String> {
    let _ = (&messages, &index);
    unimplemented!("port `startsGroup` from src/domain/message.ts")
}

/// Port of `groupAlbums` (`src/domain/message.ts:260`).
pub fn group_albums(
    messages: Vec<serde_json::Value>,
    break_before: Option<String>,
    hides_gifs: Option<bool>,
) -> Result<Vec<Vec<serde_json::Value>>, String> {
    let _ = (&messages, &break_before, &hides_gifs);
    unimplemented!("port `groupAlbums` from src/domain/message.ts")
}

/// Port of `startsDay` (`src/domain/message.ts:290`).
pub fn starts_day(messages: Vec<serde_json::Value>, index: f64) -> Result<bool, String> {
    let _ = (&messages, &index);
    unimplemented!("port `startsDay` from src/domain/message.ts")
}

/// Port of `isTranslatable` (`src/domain/message.ts:303`).
pub fn is_translatable(message: serde_json::Value) -> Result<bool, String> {
    let _ = &message;
    unimplemented!("port `isTranslatable` from src/domain/message.ts")
}

/// Port of `isEditable` (`src/domain/message.ts:313`).
pub fn is_editable(message: serde_json::Value) -> Result<bool, String> {
    let _ = &message;
    unimplemented!("port `isEditable` from src/domain/message.ts")
}

/// Port of `shouldNotify` (`src/domain/notifications.ts:45`).
pub fn should_notify(
    message: serde_json::Value,
    level: String,
    settings: serde_json::Value,
) -> Result<bool, String> {
    let _ = (&message, &level, &settings);
    unimplemented!("port `shouldNotify` from src/domain/notifications.ts")
}

/// Port of `messageLink` (`src/domain/permalink.ts:7`).
pub fn message_link(origin: String, room_id: String, event_id: String) -> Result<String, String> {
    let _ = (&origin, &room_id, &event_id);
    unimplemented!("port `messageLink` from src/domain/permalink.ts")
}

/// Port of `roomLink` (`src/domain/permalink.ts:16`).
pub fn room_link(origin: String, room_id: String) -> Result<String, String> {
    let _ = (&origin, &room_id);
    unimplemented!("port `roomLink` from src/domain/permalink.ts")
}

/// Port of `userPermalink` (`src/domain/permalink.ts:21`).
pub fn user_permalink(user_id: String) -> Result<String, String> {
    Ok(twake_domain::permalink::user_permalink(&user_id))
}

/// Port of `roomPermalink` (`src/domain/permalink.ts:26`).
pub fn room_permalink(alias_or_id: String) -> Result<String, String> {
    Ok(twake_domain::permalink::room_permalink(&alias_or_id))
}

/// Port of `parsePermalink` (`src/domain/permalink.ts:49`).
pub fn parse_permalink(
    href: String,
    app_origin: String,
) -> Result<Option<serde_json::Value>, String> {
    let _ = (&href, &app_origin);
    unimplemented!("port `parsePermalink` from src/domain/permalink.ts")
}

/// Port of `parseRoomAddress` (`src/domain/permalink.ts:107`).
pub fn parse_room_address(
    text: String,
    own_server: String,
) -> Result<Option<serde_json::Value>, String> {
    let _ = (&text, &own_server);
    unimplemented!("port `parseRoomAddress` from src/domain/permalink.ts")
}

/// Port of `readSeenSteps` (`src/domain/personalizedSteps.ts:27`).
pub fn read_seen_steps(content: serde_json::Value) -> Result<Vec<String>, String> {
    let _ = &content;
    unimplemented!("port `readSeenSteps` from src/domain/personalizedSteps.ts")
}

/// Port of `parseSeenSteps` (`src/domain/personalizedSteps.ts:36`).
pub fn parse_seen_steps(stored: String) -> Result<Vec<String>, String> {
    let _ = &stored;
    unimplemented!("port `parseSeenSteps` from src/domain/personalizedSteps.ts")
}

/// Port of `unionSteps` (`src/domain/personalizedSteps.ts:46`).
pub fn union_steps(lists: Option<Vec<Vec<String>>>) -> Result<Vec<String>, String> {
    let _ = &lists;
    unimplemented!("port `unionSteps` from src/domain/personalizedSteps.ts")
}

/// Port of `tallyPoll` (`src/domain/poll.ts:54`).
pub fn tally_poll(
    poll: serde_json::Value,
    responses: Vec<serde_json::Value>,
    ended_at: Option<f64>,
    user_id: String,
) -> Result<serde_json::Value, String> {
    let _ = (&poll, &responses, &ended_at, &user_id);
    unimplemented!("port `tallyPoll` from src/domain/poll.ts")
}

/// Port of `pressAnswer` (`src/domain/poll.ts:90`).
pub fn press_answer(
    poll: serde_json::Value,
    answer_id: String,
) -> Result<Option<Vec<String>>, String> {
    if poll.is_null() || crate::js::is_undefined(&poll) {
        return Err("TypeError: Cannot read properties of null (reading 'ownAnswerIds')".into());
    }
    let Some(own) = poll.get("ownAnswerIds").and_then(|v| v.as_array()) else {
        return Err("TypeError: Cannot read properties of undefined (reading 'includes')".into());
    };
    let own: Vec<String> = own
        .iter()
        .map(|v| v.as_str().map(str::to_string))
        .collect::<Option<_>>()
        .ok_or("SKIP: input not of the typed shape (answer ids not strings)")?;
    let max = coerce::to_number(poll.get("maxSelections").unwrap_or(&crate::js::undefined()));
    Ok(twake_domain::poll::press_answer(max, &own, &answer_id))
}

/// Port of `readNewPoll` (`src/domain/poll.ts:106`).
pub fn read_new_poll(
    question: String,
    answers: Vec<String>,
) -> Result<Option<serde_json::Value>, String> {
    let _ = (&question, &answers);
    unimplemented!("port `readNewPoll` from src/domain/poll.ts")
}

/// Port of `computeRoomUnread` (`src/domain/readState.ts:74`).
pub fn compute_room_unread(
    state: serde_json::Value,
    flags: Option<serde_json::Value>,
) -> Result<serde_json::Value, String> {
    let _ = (&state, &flags);
    unimplemented!("port `computeRoomUnread` from src/domain/readState.ts")
}

/// Port of `computeSilentUnread` (`src/domain/readState.ts:93`).
pub fn compute_silent_unread(
    state: serde_json::Value,
    flags: Option<serde_json::Value>,
) -> Result<f64, String> {
    let _ = (&state, &flags);
    unimplemented!("port `computeSilentUnread` from src/domain/readState.ts")
}

/// Port of `isUnread` (`src/domain/readState.ts:113`).
pub fn is_unread(count: serde_json::Value) -> Result<bool, String> {
    let _ = &count;
    unimplemented!("port `isUnread` from src/domain/readState.ts")
}

/// Port of `computeTotalUnread` (`src/domain/readState.ts:125`).
pub fn compute_total_unread(counts: Vec<serde_json::Value>) -> Result<serde_json::Value, String> {
    let _ = &counts;
    unimplemented!("port `computeTotalUnread` from src/domain/readState.ts")
}

/// Port of `computeReceiptTargets` (`src/domain/readState.ts:141`).
pub fn compute_receipt_targets(state: serde_json::Value) -> Result<Vec<serde_json::Value>, String> {
    let _ = &state;
    unimplemented!("port `computeReceiptTargets` from src/domain/readState.ts")
}

/// Port of `isRoomUnread` (`src/domain/readState.ts:154`).
pub fn is_room_unread(
    unread: serde_json::Value,
    silent_count: Option<f64>,
) -> Result<bool, String> {
    let _ = (&unread, &silent_count);
    unimplemented!("port `isRoomUnread` from src/domain/readState.ts")
}

/// Port of `shownUnreadCount` (`src/domain/readState.ts:166`).
pub fn shown_unread_count(entry: Option<serde_json::Value>) -> Result<f64, String> {
    let _ = &entry;
    unimplemented!("port `shownUnreadCount` from src/domain/readState.ts")
}

/// Port of `chooseReceiptEvent` (`src/domain/readState.ts:178`).
pub fn choose_receipt_event(
    ordered_ids: Vec<String>,
    unread_count: f64,
    shown_id: Option<String>,
) -> Result<Option<String>, String> {
    let _ = (&ordered_ids, &unread_count, &shown_id);
    unimplemented!("port `chooseReceiptEvent` from src/domain/readState.ts")
}

/// Port of `emptyReadState` (`src/domain/readState.ts:190`).
pub fn empty_read_state(room_id: String) -> Result<serde_json::Value, String> {
    let _ = &room_id;
    unimplemented!("port `emptyReadState` from src/domain/readState.ts")
}

/// Port of `makeReadRoomState` (`src/domain/readState.ts:198`).
pub fn make_read_room_state(state: serde_json::Value) -> Result<serde_json::Value, String> {
    let _ = &state;
    unimplemented!("port `makeReadRoomState` from src/domain/readState.ts")
}

/// Port of `hasGifsOff` (`src/domain/room.ts:92`).
pub fn has_gifs_off(room: serde_json::Value) -> Result<bool, String> {
    let _ = &room;
    unimplemented!("port `hasGifsOff` from src/domain/room.ts")
}

/// Port of `offersScribe` (`src/domain/room.ts:101`).
pub fn offers_scribe(room: serde_json::Value) -> Result<bool, String> {
    let _ = &room;
    unimplemented!("port `offersScribe` from src/domain/room.ts")
}

/// Port of `spaceOfRoom` (`src/domain/room.ts:116`).
pub fn space_of_room(room: serde_json::Value) -> Result<Option<String>, String> {
    let _ = &room;
    unimplemented!("port `spaceOfRoom` from src/domain/room.ts")
}

/// Port of `canBeListenedTo` (`src/domain/room.ts:122`).
pub fn can_be_listened_to(room: serde_json::Value) -> Result<bool, String> {
    let _ = &room;
    unimplemented!("port `canBeListenedTo` from src/domain/room.ts")
}

/// Port of `computeRoomKind` (`src/domain/room.ts:132`).
pub fn compute_room_kind(inputs: serde_json::Value) -> Result<String, String> {
    let _ = &inputs;
    unimplemented!("port `computeRoomKind` from src/domain/room.ts")
}

/// Port of `canPost` (`src/domain/room.ts:139`).
pub fn can_post(room: serde_json::Value, user_power_level: f64) -> Result<bool, String> {
    let _ = (&room, &user_power_level);
    unimplemented!("port `canPost` from src/domain/room.ts")
}

/// Port of `findSpaceGeneral` (`src/domain/room.ts:149`).
pub fn find_space_general(
    rooms: Vec<serde_json::Value>,
    space_id: String,
) -> Result<Option<serde_json::Value>, String> {
    let _ = (&rooms, &space_id);
    unimplemented!("port `findSpaceGeneral` from src/domain/room.ts")
}

/// Port of `readArchive` (`src/domain/roomArchive.ts:17`).
pub fn read_archive(content: serde_json::Value) -> Result<Vec<serde_json::Value>, String> {
    let _ = &content;
    unimplemented!("port `readArchive` from src/domain/roomArchive.ts")
}

/// Port of `withArchived` (`src/domain/roomArchive.ts:32`).
pub fn with_archived(
    rooms: Vec<serde_json::Value>,
    room_id: String,
    room: Option<serde_json::Value>,
) -> Result<Vec<serde_json::Value>, String> {
    let _ = (&rooms, &room_id, &room);
    unimplemented!("port `withArchived` from src/domain/roomArchive.ts")
}

/// Port of `toRoomLines` (`src/domain/roomLine.ts:52`).
pub fn to_room_lines(
    events: Vec<serde_json::Value>,
    message_times: Vec<f64>,
    listener_id: Option<String>,
    deleted: serde_json::Value,
    room_id: String,
) -> Result<Vec<serde_json::Value>, String> {
    let _ = (&events, &message_times, &listener_id, &deleted, &room_id);
    unimplemented!("port `toRoomLines` from src/domain/roomLine.ts")
}

/// Port of `folderView` (`src/domain/roomListView.ts:15`).
pub fn folder_view(folder: serde_json::Value) -> Result<serde_json::Value, String> {
    let _ = &folder;
    unimplemented!("port `folderView` from src/domain/roomListView.ts")
}

/// Port of `isInView` (`src/domain/roomListView.ts:26`).
pub fn is_in_view(
    entry: serde_json::Value,
    view: serde_json::Value,
    all: Vec<serde_json::Value>,
    folders: Option<Vec<serde_json::Value>>,
) -> Result<bool, String> {
    let _ = (&entry, &view, &all, &folders);
    unimplemented!("port `isInView` from src/domain/roomListView.ts")
}

/// Port of `isOnOtherServer` (`src/domain/roomManagement.ts:58`).
pub fn is_on_other_server(user_id: Option<String>, own_user_id: String) -> Result<bool, String> {
    let _ = (&user_id, &own_user_id);
    unimplemented!("port `isOnOtherServer` from src/domain/roomManagement.ts")
}

/// Port of `levelOfRole` (`src/domain/roomManagement.ts:103`).
pub fn level_of_role(role: String) -> Result<f64, String> {
    let role = serde_json::from_value(serde_json::Value::String(role))
        .unwrap_or(twake_domain::room_management::MemberRole::Member);
    Ok(twake_domain::room_management::level_of_role(role) as f64)
}

/// Port of `computeRole` (`src/domain/roomManagement.ts:111`).
pub fn compute_role(power_level: f64) -> Result<String, String> {
    Ok(twake_domain::room_management::compute_role(power_level)
        .as_str()
        .to_string())
}

/// Port of `isRoomOwner` (`src/domain/roomManagement.ts:174`).
pub fn is_room_owner(permissions: serde_json::Value) -> Result<bool, String> {
    let permissions = typed::<twake_domain::room_management::RoomPermissions>(permissions)?;
    Ok(twake_domain::room_management::is_room_owner(&permissions))
}

/// Port of `canManageMember` (`src/domain/roomManagement.ts:179`).
pub fn can_manage_member(
    member: serde_json::Value,
    permissions: serde_json::Value,
    is_self: bool,
) -> Result<bool, String> {
    let (member, permissions) = (
        typed::<twake_domain::room_management::RoomMemberDetail>(member)?,
        typed::<twake_domain::room_management::RoomPermissions>(permissions)?,
    );
    Ok(twake_domain::room_management::can_manage_member(
        &member,
        &permissions,
        is_self,
    ))
}

/// Port of `canChangeRole` (`src/domain/roomManagement.ts:196`).
pub fn can_change_role(
    member: serde_json::Value,
    permissions: serde_json::Value,
    is_self: bool,
) -> Result<bool, String> {
    let (member, permissions) = (
        typed::<twake_domain::room_management::RoomMemberDetail>(member)?,
        typed::<twake_domain::room_management::RoomPermissions>(permissions)?,
    );
    Ok(twake_domain::room_management::can_change_role(
        &member,
        &permissions,
        is_self,
    ))
}

/// Port of `assignableRoles` (`src/domain/roomManagement.ts:211`).
pub fn assignable_roles(
    member: serde_json::Value,
    permissions: serde_json::Value,
    is_self: bool,
) -> Result<Vec<String>, String> {
    let (member, permissions) = (
        typed::<twake_domain::room_management::RoomMemberDetail>(member)?,
        typed::<twake_domain::room_management::RoomPermissions>(permissions)?,
    );
    Ok(
        twake_domain::room_management::assignable_roles(&member, &permissions, is_self)
            .into_iter()
            .map(|r| r.as_str().to_string())
            .collect(),
    )
}

/// Port of `canBanMember` (`src/domain/roomManagement.ts:224`).
pub fn can_ban_member(
    member: serde_json::Value,
    permissions: serde_json::Value,
    is_self: bool,
) -> Result<bool, String> {
    let (member, permissions) = (
        typed::<twake_domain::room_management::RoomMemberDetail>(member)?,
        typed::<twake_domain::room_management::RoomPermissions>(permissions)?,
    );
    Ok(twake_domain::room_management::can_ban_member(
        &member,
        &permissions,
        is_self,
    ))
}

/// Port of `canSetReadOnly` (`src/domain/roomManagement.ts:241`).
pub fn can_set_read_only(
    member: serde_json::Value,
    permissions: serde_json::Value,
    is_self: bool,
) -> Result<bool, String> {
    let _ = (&member, &permissions, &is_self);
    unimplemented!("port `canSetReadOnly` from src/domain/roomManagement.ts")
}

/// Port of `ownershipAction` (`src/domain/roomManagement.ts:259`).
pub fn ownership_action(permissions: serde_json::Value) -> Result<Option<String>, String> {
    let _ = &permissions;
    unimplemented!("port `ownershipAction` from src/domain/roomManagement.ts")
}

/// Port of `canTakeOwnership` (`src/domain/roomManagement.ts:267`).
pub fn can_take_ownership(
    member: serde_json::Value,
    permissions: serde_json::Value,
    is_self: bool,
) -> Result<bool, String> {
    let _ = (&member, &permissions, &is_self);
    unimplemented!("port `canTakeOwnership` from src/domain/roomManagement.ts")
}

/// Port of `canTransferTo` (`src/domain/roomManagement.ts:279`).
pub fn can_transfer_to(
    member: serde_json::Value,
    permissions: serde_json::Value,
    is_self: bool,
) -> Result<bool, String> {
    let _ = (&member, &permissions, &is_self);
    unimplemented!("port `canTransferTo` from src/domain/roomManagement.ts")
}

/// Port of `hasStaffRole` (`src/domain/roomManagement.ts:295`).
pub fn has_staff_role(member: serde_json::Value) -> Result<bool, String> {
    let _ = &member;
    unimplemented!("port `hasStaffRole` from src/domain/roomManagement.ts")
}

/// Port of `permissionRoleOf` (`src/domain/roomManagement.ts:299`).
pub fn permission_role_of(
    member: serde_json::Value,
    permissions: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let _ = (&member, &permissions);
    unimplemented!("port `permissionRoleOf` from src/domain/roomManagement.ts")
}

/// Port of `givenRoles` (`src/domain/roomManagement.ts:314`).
pub fn given_roles(permissions: serde_json::Value) -> Result<Vec<String>, String> {
    let _ = &permissions;
    unimplemented!("port `givenRoles` from src/domain/roomManagement.ts")
}

/// Port of `isValidSlug` (`src/domain/roomManagement.ts:452`).
pub fn is_valid_slug(slug: String) -> Result<bool, String> {
    Ok(twake_domain::room_management::is_valid_slug(&slug))
}

/// Port of `canHavePublicPage` (`src/domain/roomManagement.ts:461`).
pub fn can_have_public_page(room: serde_json::Value) -> Result<bool, String> {
    let _ = &room;
    unimplemented!("port `canHavePublicPage` from src/domain/roomManagement.ts")
}

/// Port of `publicPagePath` (`src/domain/roomManagement.ts:476`).
pub fn public_page_path(room: serde_json::Value) -> Result<Option<String>, String> {
    let _ = &room;
    unimplemented!("port `publicPagePath` from src/domain/roomManagement.ts")
}

/// Port of `suggestSlug` (`src/domain/roomManagement.ts:483`).
pub fn suggest_slug(name: String) -> Result<String, String> {
    Ok(twake_domain::room_management::suggest_slug(&name))
}

/// Port of `isValidColor` (`src/domain/roomManagement.ts:493`).
pub fn is_valid_color(color: String) -> Result<bool, String> {
    let _ = &color;
    unimplemented!("port `isValidColor` from src/domain/roomManagement.ts")
}

/// Port of `parseMatrixUserId` (`src/domain/roomManagement.ts:502`).
pub fn parse_matrix_user_id(text: String) -> Result<Option<String>, String> {
    let _ = &text;
    unimplemented!("port `parseMatrixUserId` from src/domain/roomManagement.ts")
}

/// Port of `toPreviewText` (`src/domain/roomPreview.ts:52`).
pub fn to_preview_text(text: String) -> Result<String, String> {
    Ok(twake_domain::room_preview::to_preview_text(&text))
}

/// Port of `roomIdOfRoute` (`src/domain/roomRoute.ts:5`).
pub fn room_id_of_route(route_id: String, rooms: Vec<serde_json::Value>) -> Result<String, String> {
    let _ = (&route_id, &rooms);
    unimplemented!("port `roomIdOfRoute` from src/domain/roomRoute.ts")
}

/// Port of `readSavedRefs` (`src/domain/savedMessages.ts:21`).
pub fn read_saved_refs(content: serde_json::Value) -> Result<Vec<serde_json::Value>, String> {
    // The web keeps the entries as written (extra fields too): filter the recorded values
    // with its checks, the typed reading is `twake_domain::saved::read_saved_refs`
    let is_js_number =
        |v: &serde_json::Value| v.is_number() || v.get("$num").is_some_and(|n| n.is_string());
    let Some(list) = content.get("messages").and_then(|m| m.as_array()) else {
        return Ok(Vec::new());
    };
    Ok(list
        .iter()
        .filter(|e| {
            e.get("roomId").is_some_and(|v| v.is_string())
                && e.get("eventId").is_some_and(|v| v.is_string())
                && e.get("savedAt").is_some_and(is_js_number)
        })
        .cloned()
        .collect())
}

/// Port of `withSaved` (`src/domain/savedMessages.ts:35`).
pub fn with_saved(
    refs: Vec<serde_json::Value>,
    room_id: String,
    event_id: String,
    is_saved: bool,
    now: serde_json::Value,
) -> Result<Vec<serde_json::Value>, String> {
    // The rule of `twake_domain::saved::with_saved` on the recorded values, kept as written
    // (`now` too: the web stores whatever it is given)
    let mut out = Vec::new();
    if is_saved {
        out.push(serde_json::json!({ "roomId": room_id, "eventId": event_id, "savedAt": now }));
    }
    for r in refs {
        if r.is_null() || crate::js::is_undefined(&r) {
            return Err("TypeError: Cannot read properties of null (reading 'roomId')".into());
        }
        let same = r.get("roomId").and_then(|v| v.as_str()) == Some(room_id.as_str())
            && r.get("eventId").and_then(|v| v.as_str()) == Some(event_id.as_str());
        if !same {
            out.push(r);
        }
    }
    out.truncate(twake_domain::saved::MAX_SAVED);
    Ok(out)
}

/// Port of `readAutoTranslate` (`src/domain/scribe.ts:39`).
pub fn read_auto_translate(content: serde_json::Value) -> Result<Option<String>, String> {
    let _ = &content;
    unimplemented!("port `readAutoTranslate` from src/domain/scribe.ts")
}

/// Port of `normalizeForSearch` (`src/domain/search.ts:4`).
pub fn normalize_for_search(text: String) -> Result<String, String> {
    Ok(twake_domain::search::normalize_for_search(&text))
}

/// Port of `isClosedQuery` (`src/domain/search.ts:12`).
pub fn is_closed_query(query: String) -> Result<bool, String> {
    Ok(twake_domain::search::is_closed_query(&query))
}

/// Port of `holdsWord` (`src/domain/search.ts:18`).
pub fn holds_word(haystack: String, word: String, whole: bool) -> Result<bool, String> {
    Ok(twake_domain::search::holds_word(&haystack, &word, whole))
}

/// Port of `toIndexedMessage` (`src/domain/search.ts:52`).
pub fn to_indexed_message(
    room_id: String,
    message: serde_json::Value,
) -> Result<Option<serde_json::Value>, String> {
    let _ = (&room_id, &message);
    unimplemented!("port `toIndexedMessage` from src/domain/search.ts")
}

/// Port of `isLocalEchoId` (`src/domain/search.ts:73`).
pub fn is_local_echo_id(message_id: String) -> Result<bool, String> {
    Ok(twake_domain::search::is_local_echo_id(&message_id))
}

/// Port of `isEmptyFilter` (`src/domain/search.ts:78`).
pub fn is_empty_filter(filter: serde_json::Value) -> Result<bool, String> {
    let filter: twake_domain::search::SearchFilter = typed(filter)?;
    Ok(twake_domain::search::is_empty_filter(&filter))
}

/// Port of `matchesFilter` (`src/domain/search.ts:87`).
pub fn matches_filter(entry: serde_json::Value, filter: serde_json::Value) -> Result<bool, String> {
    let entry: twake_domain::search::IndexedMessage = typed(entry)?;
    let filter: twake_domain::search::SearchFilter = typed(filter)?;
    Ok(twake_domain::search::matches_filter(&entry, &filter))
}

/// Port of `isServiceUrl` (`src/domain/serviceUrl.ts:7`).
pub fn is_service_url(value: String) -> Result<bool, String> {
    let _ = &value;
    unimplemented!("port `isServiceUrl` from src/domain/serviceUrl.ts")
}

/// Port of `extractLinks` (`src/domain/sharedContent.ts:6`).
pub fn extract_links(text: String) -> Result<Vec<String>, String> {
    Ok(twake_domain::web_links::extract_links(&text))
}

/// Port of `computeSharedContent` (`src/domain/sharedContent.ts:36`).
pub fn compute_shared_content(
    messages: Vec<serde_json::Value>,
) -> Result<serde_json::Value, String> {
    use serde_json::{Value, json};
    let typed_messages: Vec<twake_domain::Message> = typed(Value::Array(messages.clone()))?;
    let content = twake_domain::shared_content::compute_shared_content(&typed_messages);
    // The web returns the fields as it was given them: read back from the raw message
    let raw = |message: &twake_domain::Message| -> &Value {
        let index = typed_messages
            .iter()
            .position(|m| std::ptr::eq(m, message))
            .expect("a message of the list");
        &messages[index]
    };
    let field = |m: &Value, key: &str| m.get(key).cloned().unwrap_or_else(crate::js::undefined);
    let item = |message: &twake_domain::Message| {
        let m = raw(message);
        json!({
            "messageId": field(m, "id"),
            "attachment": field(m, "attachment"),
            "senderName": field(m, "senderName"),
            "sentAt": field(m, "sentAt"),
        })
    };
    let links: Vec<Value> = content
        .links
        .iter()
        .map(|link| {
            let m = raw(link.message);
            json!({
                "messageId": field(m, "id"),
                "url": link.url,
                "senderName": field(m, "senderName"),
                "sentAt": field(m, "sentAt"),
            })
        })
        .collect();
    Ok(json!({
        "media": content.media.iter().map(|m| item(m)).collect::<Vec<_>>(),
        "links": links,
        "files": content.files.iter().map(|m| item(m)).collect::<Vec<_>>(),
    }))
}

/// Port of `roomBadgeCount` (`src/domain/spaceBadges.ts:16`).
pub fn room_badge_count(unread: serde_json::Value) -> Result<f64, String> {
    let _ = &unread;
    unimplemented!("port `roomBadgeCount` from src/domain/spaceBadges.ts")
}

/// Port of `computeSpaceBadges` (`src/domain/spaceBadges.ts:26`).
pub fn compute_space_badges(
    entries: Vec<serde_json::Value>,
) -> Result<Vec<serde_json::Value>, String> {
    let _ = &entries;
    unimplemented!("port `computeSpaceBadges` from src/domain/spaceBadges.ts")
}

/// Port of `canMoveToSpace` (`src/domain/spaceChoices.ts:59`).
pub fn can_move_to_space(
    room: serde_json::Value,
    permissions: serde_json::Value,
) -> Result<bool, String> {
    // The four fields the rule reads, of their web types; anything else is another shape
    let skip = || "SKIP: input not of the typed shape".to_string();
    let kind = room.get("kind").and_then(serde_json::Value::as_str).ok_or_else(skip)?;
    let flag = |value: &serde_json::Value, key: &str| value.get(key).and_then(serde_json::Value::as_bool).ok_or_else(skip);
    Ok(twake_domain::space_choices::can_move_to_space(
        kind == "direct",
        flag(&room, "isSpace")?,
        flag(&permissions, "isManagedBySpace")?,
        flag(&permissions, "canEditSettings")?,
    ))
}

/// Port of `arrangeRooms` (`src/domain/spaceTree.ts:23`).
pub fn arrange_rooms(
    items: Vec<serde_json::Value>,
    pinned_first: Option<bool>,
) -> Result<serde_json::Value, String> {
    let _ = (&items, &pinned_first);
    unimplemented!("port `arrangeRooms` from src/domain/spaceTree.ts")
}

/// Port of `staysInTree` (`src/domain/spaceTree.ts:79`).
pub fn stays_in_tree(item: serde_json::Value) -> Result<bool, String> {
    let _ = &item;
    unimplemented!("port `staysInTree` from src/domain/spaceTree.ts")
}

/// Port of `parentSpaceOf` (`src/domain/spaceTree.ts:114`).
pub fn parent_space_of(
    item: serde_json::Value,
    items: Vec<serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    let _ = (&item, &items);
    unimplemented!("port `parentSpaceOf` from src/domain/spaceTree.ts")
}

/// Port of `nonBlank` (`src/domain/text.ts:2`).
pub fn non_blank(text: String) -> Result<Option<String>, String> {
    let _ = &text;
    unimplemented!("port `nonBlank` from src/domain/text.ts")
}

/// Port of `stripBidi` (`src/domain/text.ts:11`).
pub fn strip_bidi(text: String) -> Result<String, String> {
    let _ = &text;
    unimplemented!("port `stripBidi` from src/domain/text.ts")
}

/// Port of `normalizeStatus` (`src/domain/text.ts:23`).
pub fn normalize_status(text: Option<String>) -> Result<Option<String>, String> {
    let _ = &text;
    unimplemented!("port `normalizeStatus` from src/domain/text.ts")
}

/// Port of `isNameTooLong` (`src/domain/text.ts:31`).
pub fn is_name_too_long(name: String) -> Result<bool, String> {
    Ok(twake_domain::room_management::is_name_too_long(&name))
}

/// Port of `presenceOfPreset` (`src/domain/text.ts:46`).
pub fn presence_of_preset(preset: serde_json::Value) -> Result<String, String> {
    let _ = &preset;
    unimplemented!("port `presenceOfPreset` from src/domain/text.ts")
}

/// Port of `computeThreadTitle` (`src/domain/thread.ts:40`).
pub fn compute_thread_title(root_text: String) -> Result<String, String> {
    let _ = &root_text;
    unimplemented!("port `computeThreadTitle` from src/domain/thread.ts")
}

/// Port of `sortThreadsByActivity` (`src/domain/thread.ts:48`).
pub fn sort_threads_by_activity(
    threads: Vec<serde_json::Value>,
) -> Result<Vec<serde_json::Value>, String> {
    let _ = &threads;
    unimplemented!("port `sortThreadsByActivity` from src/domain/thread.ts")
}

/// Port of `readThreadPrefs` (`src/domain/threadFollow.ts:22`).
pub fn read_thread_prefs(content: serde_json::Value) -> Result<Vec<serde_json::Value>, String> {
    let _ = &content;
    unimplemented!("port `readThreadPrefs` from src/domain/threadFollow.ts")
}

/// Port of `withThreadPref` (`src/domain/threadFollow.ts:36`).
pub fn with_thread_pref(
    entries: Vec<serde_json::Value>,
    room_id: String,
    thread_id: String,
    pref: String,
) -> Result<Vec<serde_json::Value>, String> {
    let _ = (&entries, &room_id, &thread_id, &pref);
    unimplemented!("port `withThreadPref` from src/domain/threadFollow.ts")
}

/// Port of `resolveThreadFollow` (`src/domain/threadFollow.ts:49`).
pub fn resolve_thread_follow(
    is_participating: bool,
    pref: Option<String>,
) -> Result<serde_json::Value, String> {
    let _ = (&is_participating, &pref);
    unimplemented!("port `resolveThreadFollow` from src/domain/threadFollow.ts")
}

/// Port of `timelineWindowOf` (`src/domain/timelineWindow.ts:45`).
pub fn timeline_window_of(
    messages: Vec<serde_json::Value>,
    lines: Option<Vec<serde_json::Value>>,
    start: serde_json::Value,
    reach: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let _ = (&messages, &lines, &start, &reach);
    unimplemented!("port `timelineWindowOf` from src/domain/timelineWindow.ts")
}

/// Port of `olderStartOf` (`src/domain/timelineWindow.ts:73`).
pub fn older_start_of(
    messages: Vec<serde_json::Value>,
    hidden_above: f64,
) -> Result<serde_json::Value, String> {
    let _ = (&messages, &hidden_above);
    unimplemented!("port `olderStartOf` from src/domain/timelineWindow.ts")
}

/// Port of `commonSettingsUrlFor` (`src/domain/twakeServices.ts:72`).
pub fn common_settings_url_for(
    settings: serde_json::Value,
    user_id: String,
) -> Result<Option<String>, String> {
    let _ = (&settings, &user_id);
    unimplemented!("port `commonSettingsUrlFor` from src/domain/twakeServices.ts")
}

/// Port of `findWebLinks` (`src/domain/webLinks.ts:24`).
pub fn find_web_links(text: String) -> Result<Vec<serde_json::Value>, String> {
    // JavaScript counts the index in UTF-16 units
    Ok(twake_domain::web_links::find_web_links(&text)
        .into_iter()
        .map(|l| {
            let index = twake_domain::js::utf16_of_byte(&text, l.index);
            serde_json::json!({ "index": index, "url": l.url })
        })
        .collect())
}

/// Port of `toHref` (`src/domain/webLinks.ts:46`).
pub fn to_href(url: String) -> Result<String, String> {
    Ok(twake_domain::web_links::to_href(&url))
}

/// A recorded input read as the typed Rust value. Inputs of another shape (generated `"x"`,
/// `true` in place of an object) have no typed equivalent: they are reported as skipped by
/// `rust/tools/check-ported.mjs`, never counted as passing.
fn typed<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> Result<T, String> {
    serde_json::from_value(value).map_err(|e| format!("SKIP: input not of the typed shape ({e})"))
}

/// JavaScript coercions for the recorded inputs, which may be of any type. Kept here, in the
/// harness: the typed functions of `twake-domain` take real types.
mod coerce {
    use serde_json::Value;

    /// `!value`.
    pub fn is_falsy(value: &Value) -> bool {
        match value {
            Value::Null => true,
            Value::Bool(b) => !b,
            Value::String(s) => s.is_empty(),
            Value::Number(n) => n.as_f64() == Some(0.0),
            v if crate::js::is_undefined(v) => true,
            v => crate::js::as_number(v).is_some_and(|n| n == 0.0 || n.is_nan()),
        }
    }

    /// `Number(value)` (ToNumber), for the shapes the recordings use.
    pub fn to_number(value: &Value) -> f64 {
        if let Some(n) = crate::js::as_number(value) {
            return n;
        }
        match value {
            Value::Null => 0.0,
            Value::Bool(b) => f64::from(u8::from(*b)),
            Value::String(s) => string_to_number(s),
            Value::Array(items) => match items.as_slice() {
                [] => 0.0,
                [one] => match one {
                    Value::Array(_) | Value::Object(_) => f64::NAN,
                    Value::Null => 0.0,
                    Value::String(s) => string_to_number(s),
                    other => to_number(other),
                },
                _ => f64::NAN,
            },
            _ => f64::NAN,
        }
    }

    fn string_to_number(s: &str) -> f64 {
        let t = twake_domain::js::trim(s);
        if t.is_empty() {
            return 0.0;
        }
        match t {
            "Infinity" | "+Infinity" => f64::INFINITY,
            "-Infinity" => f64::NEG_INFINITY,
            _ => {
                if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
                    return u64::from_str_radix(hex, 16).map_or(f64::NAN, |n| n as f64);
                }
                if t.chars()
                    .any(|c| c.is_ascii_alphabetic() && !matches!(c, 'e' | 'E'))
                {
                    return f64::NAN;
                }
                t.parse::<f64>().unwrap_or(f64::NAN)
            }
        }
    }
}

/// `text.slice(0, caret)` with JavaScript's handling of the index.
fn js_slice_to(text: &str, caret: f64) -> String {
    let units = crate::utf16::units(text);
    let len = units.len() as f64;
    let end = if caret.is_nan() {
        0.0
    } else if caret < 0.0 {
        (len + caret.trunc()).max(0.0)
    } else {
        caret.trunc().min(len)
    };
    crate::utf16::slice(&units, 0, end as usize)
}

/// The end of `slice(0, limit)` over `len` items, `limit` defaulting to 25.
fn js_limit(limit: Option<f64>, len: usize) -> usize {
    let len_f = len as f64;
    let end = match limit {
        None => 25.0,
        Some(l) if l.is_nan() => 0.0,
        Some(l) if l < 0.0 => (len_f + l.trunc()).max(0.0),
        Some(l) => l.trunc(),
    };
    end.min(len_f) as usize
}

/// The commands argument: an array, or anything when the text never reaches
/// `commands.find` (not a « /word »); otherwise JavaScript throws, which is not replayed.
fn commands_of(
    text: &str,
    commands: serde_json::Value,
) -> Result<
    (
        Vec<serde_json::Value>,
        Vec<twake_domain::composer::BotCommand>,
    ),
    String,
> {
    match commands {
        serde_json::Value::Array(items) => {
            let typed_commands = typed(serde_json::Value::Array(items.clone()))?;
            Ok((items, typed_commands))
        }
        _ if !reaches_find(text) => Ok((Vec::new(), Vec::new())),
        _ => Err("SKIP: input not of the typed shape (commands is not an array)".into()),
    }
}

/// Whether `prepareCommandText` calls `commands.find` for this text: a « /word » that
/// is not « // ».
fn reaches_find(text: &str) -> bool {
    let p = twake_domain::composer::prepare_command_text(text, &[]);
    !text.starts_with("//") && (p.unknown_command.is_some() || p.is_emote || p.text != text)
}
