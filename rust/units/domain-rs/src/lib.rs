//! JS bindings for `src/domain/`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use napi_derive::napi;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unknown panic".into())
}

#[allow(dead_code)]
fn run<T>(f: impl FnOnce() -> Result<T, String>) -> napi::Result<T> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(result) => result.map_err(napi::Error::from_reason),
        Err(payload) => Err(napi::Error::from_reason(format!(
            "Rust panic: {}",
            panic_message(&*payload)
        ))),
    }
}

/// `buildActivity`.
#[napi(js_name = "buildActivity")]
pub fn build_activity(
    mentions: String,
    threads: String,
    reactions: String,
    missed_calls: String,
) -> napi::Result<String> {
    run(move || {
        let mentions = crate::js::from_json(&mentions)?;
        let threads = crate::js::from_json(&threads)?;
        let reactions = crate::js::from_json(&reactions)?;
        let missed_calls = crate::js::from_json(&missed_calls)?;
        let out = port::build_activity(mentions, threads, reactions, missed_calls)?;
        crate::js::to_json(&out)
    })
}

/// `buildFeed`.
#[napi(js_name = "buildFeed")]
pub fn build_feed(activity: String, unread: String, saved: String) -> napi::Result<String> {
    run(move || {
        let activity = crate::js::from_json(&activity)?;
        let unread = crate::js::from_json(&unread)?;
        let saved = crate::js::from_json(&saved)?;
        let out = port::build_feed(activity, unread, saved)?;
        crate::js::to_json(&out)
    })
}

/// `filterFeed`.
#[napi(js_name = "filterFeed")]
pub fn filter_feed(items: String, filter: String) -> napi::Result<String> {
    run(move || {
        let items = crate::js::from_json(&items)?;
        let out = port::filter_feed(items, filter)?;
        crate::js::to_json(&out)
    })
}

/// `searchFeed`.
#[napi(js_name = "searchFeed")]
pub fn search_feed(items: String, query: String) -> napi::Result<String> {
    run(move || {
        let items = crate::js::from_json(&items)?;
        let out = port::search_feed(items, query)?;
        crate::js::to_json(&out)
    })
}

/// `groupByDay`.
#[napi(js_name = "groupByDay")]
pub fn group_by_day(items: String, now: f64) -> napi::Result<String> {
    run(move || {
        let items = crate::js::from_json(&items)?;
        let out = port::group_by_day(items, now)?;
        crate::js::to_json(&out)
    })
}

/// `assistantConsentStatus`.
#[napi(js_name = "assistantConsentStatus")]
pub fn assistant_consent_status(
    consent: String,
    assistant_id: String,
    member_ids: Vec<String>,
) -> napi::Result<String> {
    run(move || {
        let consent = crate::js::from_json(&consent)?;
        let out = port::assistant_consent_status(consent, assistant_id, member_ids)?;
        crate::js::to_json(&out)
    })
}

/// `requestsWaitingFor`.
#[napi(js_name = "requestsWaitingFor")]
pub fn requests_waiting_for(
    consent: String,
    user_id: String,
    member_ids: Vec<String>,
) -> napi::Result<String> {
    run(move || {
        let consent = crate::js::from_json(&consent)?;
        let out = port::requests_waiting_for(consent, user_id, member_ids)?;
        crate::js::to_json(&out)
    })
}

/// `peopleOf`.
#[napi(js_name = "peopleOf")]
pub fn people_of(members: String, consent: String) -> napi::Result<Vec<String>> {
    run(move || {
        let members = crate::js::from_json(&members)?;
        let consent = crate::js::from_json(&consent)?;
        let out = port::people_of(members, consent)?;
        Ok(out)
    })
}

/// `presentRequest`.
#[napi(js_name = "presentRequest")]
pub fn present_request(consent: String, members: String) -> napi::Result<String> {
    run(move || {
        let consent = crate::js::from_json(&consent)?;
        let members = crate::js::from_json(&members)?;
        let out = port::present_request(consent, members)?;
        crate::js::to_json(&out)
    })
}

/// `findOpenQuestion`.
#[napi(js_name = "findOpenQuestion")]
pub fn find_open_question(
    messages: String,
    threads: String,
    asker_ids: Vec<String>,
    now: f64,
) -> napi::Result<String> {
    run(move || {
        let messages = crate::js::from_json(&messages)?;
        let threads = crate::js::from_json(&threads)?;
        let out = port::find_open_question(messages, threads, asker_ids, now)?;
        crate::js::to_json(&out)
    })
}

/// `computeWaveform`.
#[napi(js_name = "computeWaveform")]
pub fn compute_waveform(samples: String, points: String) -> napi::Result<Vec<f64>> {
    run(move || {
        let samples = crate::js::from_json(&samples)?;
        let points = crate::js::from_json(&points)?;
        let out = port::compute_waveform(samples, points)?;
        Ok(out)
    })
}

/// `isPlayableAudio`.
#[napi(js_name = "isPlayableAudio")]
pub fn is_playable_audio(attachment: String) -> napi::Result<bool> {
    run(move || {
        let attachment = crate::js::from_json(&attachment)?;
        let out = port::is_playable_audio(attachment)?;
        Ok(out)
    })
}

/// `isViewablePdf`.
#[napi(js_name = "isViewablePdf")]
pub fn is_viewable_pdf(attachment: String, can_show_pdf: bool) -> napi::Result<bool> {
    run(move || {
        let attachment = crate::js::from_json(&attachment)?;
        let out = port::is_viewable_pdf(attachment, can_show_pdf)?;
        Ok(out)
    })
}

/// `isPreviewable`.
#[napi(js_name = "isPreviewable")]
pub fn is_previewable(attachment: String) -> napi::Result<bool> {
    run(move || {
        let attachment = crate::js::from_json(&attachment)?;
        let out = port::is_previewable(attachment)?;
        Ok(out)
    })
}

/// `isGif`.
#[napi(js_name = "isGif")]
pub fn is_gif(attachment: String) -> napi::Result<bool> {
    run(move || {
        let attachment = crate::js::from_json(&attachment)?;
        let out = port::is_gif(attachment)?;
        Ok(out)
    })
}

/// `computeAttachmentKind`.
#[napi(js_name = "computeAttachmentKind")]
pub fn compute_attachment_kind(mime_type: String) -> napi::Result<String> {
    run(move || {
        let out = port::compute_attachment_kind(mime_type)?;
        Ok(out)
    })
}

/// `inferMimeType`.
#[napi(js_name = "inferMimeType")]
pub fn infer_mime_type(name: String, mime_type: String) -> napi::Result<String> {
    run(move || {
        let out = port::infer_mime_type(name, mime_type)?;
        Ok(out)
    })
}

/// `isSvg`.
#[napi(js_name = "isSvg")]
pub fn is_svg(mime_type: String) -> napi::Result<bool> {
    run(move || {
        let out = port::is_svg(mime_type)?;
        Ok(out)
    })
}

/// `blobSafeMimeType`.
#[napi(js_name = "blobSafeMimeType")]
pub fn blob_safe_mime_type(mime_type: String) -> napi::Result<String> {
    run(move || {
        let out = port::blob_safe_mime_type(mime_type)?;
        Ok(out)
    })
}

/// `svgCanvasSize`.
#[napi(js_name = "svgCanvasSize")]
pub fn svg_canvas_size(width: f64, height: f64) -> napi::Result<String> {
    run(move || {
        let out = port::svg_canvas_size(width, height)?;
        crate::js::to_json(&out)
    })
}

/// `thumbnailSize`.
#[napi(js_name = "thumbnailSize")]
pub fn thumbnail_size(media: String, dimensions: String) -> napi::Result<String> {
    run(move || {
        let media = crate::js::from_json(&media)?;
        let dimensions = crate::js::from_json(&dimensions)?;
        let out = port::thumbnail_size(media, dimensions)?;
        crate::js::to_json(&out)
    })
}

/// `botOf`.
#[napi(js_name = "botOf")]
pub fn bot_of(answer: String) -> napi::Result<String> {
    run(move || {
        let answer = crate::js::from_json(&answer)?;
        let out = port::bot_of(answer)?;
        crate::js::to_json(&out)
    })
}

/// `botIdOf`.
#[napi(js_name = "botIdOf")]
pub fn bot_id_of(user_id: String) -> napi::Result<String> {
    run(move || {
        let out = port::bot_id_of(user_id)?;
        Ok(out)
    })
}

/// `ownsBot`.
#[napi(js_name = "ownsBot")]
pub fn owns_bot(
    own_user_id: String,
    bot_user_id: String,
    my_bot_user_id: Option<String>,
) -> napi::Result<bool> {
    run(move || {
        let out = port::owns_bot(own_user_id, bot_user_id, my_bot_user_id)?;
        Ok(out)
    })
}

/// `offersCommandsOf`.
#[napi(js_name = "offersCommandsOf")]
pub fn offers_commands_of(
    own_user_id: String,
    bot_user_id: String,
    my_bot_user_id: Option<String>,
    creator_id: Option<String>,
) -> napi::Result<bool> {
    run(move || {
        let out = port::offers_commands_of(own_user_id, bot_user_id, my_bot_user_id, creator_id)?;
        Ok(out)
    })
}

/// `findCommandQuery`.
#[napi(js_name = "findCommandQuery")]
pub fn find_command_query(text: String, caret: f64) -> napi::Result<String> {
    run(move || {
        let out = port::find_command_query(text, caret)?;
        crate::js::to_json(&out)
    })
}

/// `matchCommands`.
#[napi(js_name = "matchCommands")]
pub fn match_commands(commands: String, query: String) -> napi::Result<String> {
    run(move || {
        let commands = crate::js::from_json(&commands)?;
        let out = port::match_commands(commands, query)?;
        crate::js::to_json(&out)
    })
}

/// `findLocalCommand`.
#[napi(js_name = "findLocalCommand")]
pub fn find_local_command(text: String) -> napi::Result<String> {
    run(move || {
        let out = port::find_local_command(text)?;
        crate::js::to_json(&out)
    })
}

/// `prepareCommandText`.
#[napi(js_name = "prepareCommandText")]
pub fn prepare_command_text(text: String, commands: String) -> napi::Result<String> {
    run(move || {
        let commands = crate::js::from_json(&commands)?;
        let out = port::prepare_command_text(text, commands)?;
        crate::js::to_json(&out)
    })
}

/// `prepareOutgoing`.
#[napi(js_name = "prepareOutgoing")]
pub fn prepare_outgoing(text: String, mentions: String, commands: String) -> napi::Result<String> {
    run(move || {
        let mentions = crate::js::from_json(&mentions)?;
        let commands = crate::js::from_json(&commands)?;
        let out = port::prepare_outgoing(text, mentions, commands)?;
        crate::js::to_json(&out)
    })
}

/// `shouldMirror`.
#[napi(js_name = "shouldMirror")]
pub fn should_mirror(tile: String) -> napi::Result<String> {
    run(move || {
        let tile = crate::js::from_json(&tile)?;
        let out = port::should_mirror(tile)?;
        crate::js::to_json(&out)
    })
}

/// `recordingModeOf`.
#[napi(js_name = "recordingModeOf")]
pub fn recording_mode_of(
    is_recording: bool,
    metadata: Option<String>,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let out = port::recording_mode_of(is_recording, metadata)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `currentDeviceId`.
#[napi(js_name = "currentDeviceId")]
pub fn current_device_id(
    devices: String,
    active_id: Option<String>,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let devices = crate::js::from_json(&devices)?;
        let out = port::current_device_id(devices, active_id)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `countCallPeople`.
#[napi(js_name = "countCallPeople")]
pub fn count_call_people(members: String) -> napi::Result<f64> {
    run(move || {
        let members = crate::js::from_json(&members)?;
        let out = port::count_call_people(members)?;
        Ok(out)
    })
}

/// `ringAsks`.
#[napi(js_name = "ringAsks")]
pub fn ring_asks(
    ring: String,
    members: String,
    own_user_id: String,
    is_direct: bool,
    now: f64,
) -> napi::Result<bool> {
    run(move || {
        let ring = crate::js::from_json(&ring)?;
        let members = crate::js::from_json(&members)?;
        let out = port::ring_asks(ring, members, own_user_id, is_direct, now)?;
        Ok(out)
    })
}

/// `isRinging`.
#[napi(js_name = "isRinging")]
pub fn is_ringing(
    members: String,
    own_user_id: String,
    is_direct: bool,
    is_joined: bool,
    left_at: Option<f64>,
) -> napi::Result<bool> {
    run(move || {
        let members = crate::js::from_json(&members)?;
        let out = port::is_ringing(members, own_user_id, is_direct, is_joined, left_at)?;
        Ok(out)
    })
}

/// `findCallThread`.
#[napi(js_name = "findCallThread")]
pub fn find_call_thread(messages: String, now: f64) -> napi::Result<String> {
    run(move || {
        let messages = crate::js::from_json(&messages)?;
        let out = port::find_call_thread(messages, now)?;
        crate::js::to_json(&out)
    })
}

/// `toCallLine`.
#[napi(js_name = "toCallLine")]
pub fn to_call_line(content: String, is_own: bool) -> napi::Result<String> {
    run(move || {
        let content = crate::js::from_json(&content)?;
        let out = port::to_call_line(content, is_own)?;
        crate::js::to_json(&out)
    })
}

/// `fitWindow`.
#[napi(js_name = "fitWindow")]
pub fn fit_window(rect: String, screen: String) -> napi::Result<String> {
    run(move || {
        let rect = crate::js::from_json(&rect)?;
        let screen = crate::js::from_json(&screen)?;
        let out = port::fit_window(rect, screen)?;
        crate::js::to_json(&out)
    })
}

/// `defaultCallWindow`.
#[napi(js_name = "defaultCallWindow")]
pub fn default_call_window(screen: String) -> napi::Result<String> {
    run(move || {
        let screen = crate::js::from_json(&screen)?;
        let out = port::default_call_window(screen)?;
        crate::js::to_json(&out)
    })
}

/// `moveWindow`.
#[napi(js_name = "moveWindow")]
pub fn move_window(rect: String, dx: f64, dy: f64, screen: String) -> napi::Result<String> {
    run(move || {
        let rect = crate::js::from_json(&rect)?;
        let screen = crate::js::from_json(&screen)?;
        let out = port::move_window(rect, dx, dy, screen)?;
        crate::js::to_json(&out)
    })
}

/// `placeInCorner`.
#[napi(js_name = "placeInCorner")]
pub fn place_in_corner(rect: String, corner: String, screen: String) -> napi::Result<String> {
    run(move || {
        let rect = crate::js::from_json(&rect)?;
        let screen = crate::js::from_json(&screen)?;
        let out = port::place_in_corner(rect, corner, screen)?;
        crate::js::to_json(&out)
    })
}

/// `resizeWindow`.
#[napi(js_name = "resizeWindow")]
pub fn resize_window(
    rect: String,
    edge: String,
    dx: f64,
    dy: f64,
    screen: String,
) -> napi::Result<String> {
    run(move || {
        let rect = crate::js::from_json(&rect)?;
        let screen = crate::js::from_json(&screen)?;
        let out = port::resize_window(rect, edge, dx, dy, screen)?;
        crate::js::to_json(&out)
    })
}

/// `parseCallWindow`.
#[napi(js_name = "parseCallWindow")]
pub fn parse_call_window(stored: Option<String>) -> napi::Result<String> {
    run(move || {
        let out = port::parse_call_window(stored)?;
        crate::js::to_json(&out)
    })
}

/// `placeCall`.
#[napi(js_name = "placeCall")]
pub fn place_call(state: String, now: String) -> napi::Result<String> {
    run(move || {
        let state = crate::js::from_json(&state)?;
        let now = crate::js::from_json(&now)?;
        let out = port::place_call(state, now)?;
        crate::js::to_json(&out)
    })
}

/// `isCallFloating`.
#[napi(js_name = "isCallFloating")]
pub fn is_call_floating(state: String, call_room_id: Option<String>) -> napi::Result<bool> {
    run(move || {
        let state = crate::js::from_json(&state)?;
        let out = port::is_call_floating(state, call_room_id)?;
        Ok(out)
    })
}

/// `parseClientConfig`.
#[napi(js_name = "parseClientConfig")]
pub fn parse_client_config(raw: String) -> napi::Result<String> {
    run(move || {
        let raw = crate::js::from_json(&raw)?;
        let out = port::parse_client_config(raw)?;
        crate::js::to_json(&out)
    })
}

/// `readComposerStyle`.
#[napi(js_name = "readComposerStyle")]
pub fn read_composer_style(content: String) -> napi::Result<String> {
    run(move || {
        let content = crate::js::from_json(&content)?;
        let out = port::read_composer_style(content)?;
        crate::js::to_json(&out)
    })
}

/// `parseInvitationTarget`.
#[napi(js_name = "parseInvitationTarget")]
pub fn parse_invitation_target(input: String) -> napi::Result<String> {
    run(move || {
        let out = port::parse_invitation_target(input)?;
        crate::js::to_json(&out)
    })
}

/// `groupByInitial`.
#[napi(js_name = "groupByInitial")]
pub fn group_by_initial(people: String) -> napi::Result<String> {
    run(move || {
        let people = crate::js::from_json(&people)?;
        let out = port::group_by_initial(people)?;
        crate::js::to_json(&out)
    })
}

/// `parseContactsVisibility`.
#[napi(js_name = "parseContactsVisibility")]
pub fn parse_contacts_visibility(
    value: String,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let value = crate::js::from_json(&value)?;
        let out = port::parse_contacts_visibility(value)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `emptyCardOf`.
#[napi(js_name = "emptyCardOf")]
pub fn empty_card_of(facts: String) -> napi::Result<String> {
    run(move || {
        let facts = crate::js::from_json(&facts)?;
        let out = port::empty_card_of(facts)?;
        crate::js::to_json(&out)
    })
}

/// `footerOf`.
#[napi(js_name = "footerOf")]
pub fn footer_of(facts: String) -> napi::Result<String> {
    run(move || {
        let facts = crate::js::from_json(&facts)?;
        let out = port::footer_of(facts)?;
        crate::js::to_json(&out)
    })
}

/// `timelineHeadOf`.
#[napi(js_name = "timelineHeadOf")]
pub fn timeline_head_of(facts: String) -> napi::Result<String> {
    run(move || {
        let facts = crate::js::from_json(&facts)?;
        let out = port::timeline_head_of(facts)?;
        crate::js::to_json(&out)
    })
}

/// `headOverWindow`.
#[napi(js_name = "headOverWindow")]
pub fn head_over_window(head: String, hidden_above: f64) -> napi::Result<String> {
    run(move || {
        let head = crate::js::from_json(&head)?;
        let out = port::head_over_window(head, hidden_above)?;
        crate::js::to_json(&out)
    })
}

/// `isLandingRoom`.
#[napi(js_name = "isLandingRoom")]
pub fn is_landing_room(room: String, is_embedded: bool) -> napi::Result<bool> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::is_landing_room(room, is_embedded)?;
        Ok(out)
    })
}

/// `mayPostIn`.
#[napi(js_name = "mayPostIn")]
pub fn may_post_in(facts: String) -> napi::Result<bool> {
    run(move || {
        let facts = crate::js::from_json(&facts)?;
        let out = port::may_post_in(facts)?;
        Ok(out)
    })
}

/// `isDirectRoom`.
#[napi(js_name = "isDirectRoom")]
pub fn is_direct_room(room: String, direct_user_id: Option<String>) -> napi::Result<bool> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::is_direct_room(room, direct_user_id)?;
        Ok(out)
    })
}

/// `peerStateOf`.
#[napi(js_name = "peerStateOf")]
pub fn peer_state_of(facts: String) -> napi::Result<String> {
    run(move || {
        let facts = crate::js::from_json(&facts)?;
        let out = port::peer_state_of(facts)?;
        crate::js::to_json(&out)
    })
}

/// `splitsCallChat`.
#[napi(js_name = "splitsCallChat")]
pub fn splits_call_chat(facts: String) -> napi::Result<String> {
    run(move || {
        let facts = crate::js::from_json(&facts)?;
        let out = port::splits_call_chat(facts)?;
        crate::js::to_json(&out)
    })
}

/// `isEncryptedConversation`.
#[napi(js_name = "isEncryptedConversation")]
pub fn is_encrypted_conversation(room: String) -> napi::Result<bool> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::is_encrypted_conversation(room)?;
        Ok(out)
    })
}

/// `isAwaitingRoom`.
#[napi(js_name = "isAwaitingRoom")]
pub fn is_awaiting_room(facts: String) -> napi::Result<bool> {
    run(move || {
        let facts = crate::js::from_json(&facts)?;
        let out = port::is_awaiting_room(facts)?;
        Ok(out)
    })
}

/// `isEmptyConversation`.
#[napi(js_name = "isEmptyConversation")]
pub fn is_empty_conversation(message_count: f64, has_outgoing: bool) -> napi::Result<bool> {
    run(move || {
        let out = port::is_empty_conversation(message_count, has_outgoing)?;
        Ok(out)
    })
}

/// `isCardInlineIn`.
#[napi(js_name = "isCardInlineIn")]
pub fn is_card_inline_in(line_count: f64, has_predecessor: bool) -> napi::Result<bool> {
    run(move || {
        let out = port::is_card_inline_in(line_count, has_predecessor)?;
        Ok(out)
    })
}

/// `roomNameOf`.
#[napi(js_name = "roomNameOf")]
pub fn room_name_of(sources: String) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let sources = crate::js::from_json(&sources)?;
        let out = port::room_name_of(sources)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `assistantOfferIn`.
#[napi(js_name = "assistantOfferIn")]
pub fn assistant_offer_in(
    facts: String,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let facts = crate::js::from_json(&facts)?;
        let out = port::assistant_offer_in(facts)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `hasAssistantIn`.
#[napi(js_name = "hasAssistantIn")]
pub fn has_assistant_in(room: String, consent: String, members: String) -> napi::Result<bool> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let consent = crate::js::from_json(&consent)?;
        let members = crate::js::from_json(&members)?;
        let out = port::has_assistant_in(room, consent, members)?;
        Ok(out)
    })
}

/// `offersAddContact`.
#[napi(js_name = "offersAddContact")]
pub fn offers_add_contact(facts: String) -> napi::Result<bool> {
    run(move || {
        let facts = crate::js::from_json(&facts)?;
        let out = port::offers_add_contact(facts)?;
        Ok(out)
    })
}

/// `matchPlaceOf`.
#[napi(js_name = "matchPlaceOf")]
pub fn match_place_of(
    message_ids: Vec<String>,
    focus_event_id: Option<String>,
) -> napi::Result<String> {
    run(move || {
        let out = port::match_place_of(message_ids, focus_event_id)?;
        crate::js::to_json(&out)
    })
}

/// `readDeletedChats`.
#[napi(js_name = "readDeletedChats")]
pub fn read_deleted_chats(content: String) -> napi::Result<String> {
    run(move || {
        let content = crate::js::from_json(&content)?;
        let out = port::read_deleted_chats(content)?;
        crate::js::to_json(&out)
    })
}

/// `isChatHidden`.
#[napi(js_name = "isChatHidden")]
pub fn is_chat_hidden(
    deleted: String,
    room_id: String,
    last_activity_at: f64,
) -> napi::Result<bool> {
    run(move || {
        let deleted = crate::js::from_json(&deleted)?;
        let out = port::is_chat_hidden(deleted, room_id, last_activity_at)?;
        Ok(out)
    })
}

/// `withoutDeleted`.
#[napi(js_name = "withoutDeleted")]
pub fn without_deleted(deleted: String, room_id: String, messages: String) -> napi::Result<String> {
    run(move || {
        let deleted = crate::js::from_json(&deleted)?;
        let messages = crate::js::from_json(&messages)?;
        let out = port::without_deleted(deleted, room_id, messages)?;
        crate::js::to_json(&out)
    })
}

/// `isAfterDeletion`.
#[napi(js_name = "isAfterDeletion")]
pub fn is_after_deletion(deleted: String, room_id: String, at: f64) -> napi::Result<bool> {
    run(move || {
        let deleted = crate::js::from_json(&deleted)?;
        let out = port::is_after_deletion(deleted, room_id, at)?;
        Ok(out)
    })
}

/// `readDismissedQuestions`.
#[napi(js_name = "readDismissedQuestions")]
pub fn read_dismissed_questions(content: String) -> napi::Result<String> {
    run(move || {
        let content = crate::js::from_json(&content)?;
        let out = port::read_dismissed_questions(content)?;
        crate::js::to_json(&out)
    })
}

/// `withDismissedQuestion`.
#[napi(js_name = "withDismissedQuestion")]
pub fn with_dismissed_question(
    dismissed: String,
    id: String,
    closes_at: f64,
    now: f64,
) -> napi::Result<String> {
    run(move || {
        let dismissed = crate::js::from_json(&dismissed)?;
        let out = port::with_dismissed_question(dismissed, id, closes_at, now)?;
        crate::js::to_json(&out)
    })
}

/// `findEmojiQuery`.
#[napi(js_name = "findEmojiQuery")]
pub fn find_emoji_query(text: String, caret: f64) -> napi::Result<String> {
    run(move || {
        let out = port::find_emoji_query(text, caret)?;
        crate::js::to_json(&out)
    })
}

/// `isEmojiDrawable`.
#[napi(js_name = "isEmojiDrawable")]
pub fn is_emoji_drawable(emoji: String, max: String, flags: Option<bool>) -> napi::Result<bool> {
    run(move || {
        let max = crate::js::from_json(&max)?;
        let out = port::is_emoji_drawable(emoji, max, flags)?;
        Ok(out)
    })
}

/// `hasInk`.
#[napi(js_name = "hasInk")]
pub fn has_ink(pixels: String) -> napi::Result<bool> {
    run(move || {
        let pixels = crate::js::from_json(&pixels)?;
        let out = port::has_ink(pixels)?;
        Ok(out)
    })
}

/// `hasColour`.
#[napi(js_name = "hasColour")]
pub fn has_colour(pixels: String) -> napi::Result<bool> {
    run(move || {
        let pixels = crate::js::from_json(&pixels)?;
        let out = port::has_colour(pixels)?;
        Ok(out)
    })
}

/// `isDrawn`.
#[napi(js_name = "isDrawn")]
pub fn is_drawn(sample: String, blank: String) -> napi::Result<bool> {
    run(move || {
        let sample = crate::js::from_json(&sample)?;
        let blank = crate::js::from_json(&blank)?;
        let out = port::is_drawn(sample, blank)?;
        Ok(out)
    })
}

/// `undecryptableHelpKey`.
#[napi(js_name = "undecryptableHelpKey")]
pub fn undecryptable_help_key(reason: String, is_device_verified: bool) -> napi::Result<String> {
    run(move || {
        let out = port::undecryptable_help_key(reason, is_device_verified)?;
        Ok(out)
    })
}

/// `normalizeRecoveryKey`.
#[napi(js_name = "normalizeRecoveryKey")]
pub fn normalize_recovery_key(
    input: String,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let out = port::normalize_recovery_key(input)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `isNotificationService`.
#[napi(js_name = "isNotificationService")]
pub fn is_notification_service(device: String) -> napi::Result<bool> {
    run(move || {
        let device = crate::js::from_json(&device)?;
        let out = port::is_notification_service(device)?;
        Ok(out)
    })
}

/// `readFolders`.
#[napi(js_name = "readFolders")]
pub fn read_folders(content: String) -> napi::Result<String> {
    run(move || {
        let content = crate::js::from_json(&content)?;
        let out = port::read_folders(content)?;
        crate::js::to_json(&out)
    })
}

/// `writeFolders`.
#[napi(js_name = "writeFolders")]
pub fn write_folders(folders: String) -> napi::Result<String> {
    run(move || {
        let folders = crate::js::from_json(&folders)?;
        let out = port::write_folders(folders)?;
        crate::js::to_json(&out)
    })
}

/// `withHiddenBuiltins`.
#[napi(js_name = "withHiddenBuiltins")]
pub fn with_hidden_builtins(folders: String) -> napi::Result<String> {
    run(move || {
        let folders = crate::js::from_json(&folders)?;
        let out = port::with_hidden_builtins(folders)?;
        crate::js::to_json(&out)
    })
}

/// `listFolders`.
#[napi(js_name = "listFolders")]
pub fn list_folders(saved: String, entries: String) -> napi::Result<String> {
    run(move || {
        let saved = crate::js::from_json(&saved)?;
        let entries = crate::js::from_json(&entries)?;
        let out = port::list_folders(saved, entries)?;
        crate::js::to_json(&out)
    })
}

/// `folderChatTypeOf`.
#[napi(js_name = "folderChatTypeOf")]
pub fn folder_chat_type_of(
    room: String,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::folder_chat_type_of(room)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `isInFolder`.
#[napi(js_name = "isInFolder")]
pub fn is_in_folder(entry: String, folder: String) -> napi::Result<bool> {
    run(move || {
        let entry = crate::js::from_json(&entry)?;
        let folder = crate::js::from_json(&folder)?;
        let out = port::is_in_folder(entry, folder)?;
        Ok(out)
    })
}

/// `isRecommendationTaken`.
#[napi(js_name = "isRecommendationTaken")]
pub fn is_recommendation_taken(r#type: String, folders: String) -> napi::Result<bool> {
    run(move || {
        let folders = crate::js::from_json(&folders)?;
        let out = port::is_recommendation_taken(r#type, folders)?;
        Ok(out)
    })
}

/// `formatDuration`.
#[napi(js_name = "formatDuration")]
pub fn format_duration(milliseconds: f64) -> napi::Result<String> {
    run(move || {
        let out = port::format_duration(milliseconds)?;
        Ok(out)
    })
}

/// `fitPreview`.
#[napi(js_name = "fitPreview")]
pub fn fit_preview(
    dimensions: String,
    max_side: f64,
    empty_ratio: Option<f64>,
) -> napi::Result<String> {
    run(move || {
        let dimensions = crate::js::from_json(&dimensions)?;
        let out = port::fit_preview(dimensions, max_side, empty_ratio)?;
        crate::js::to_json(&out)
    })
}

/// `isRecord`.
#[napi(js_name = "isRecord")]
pub fn is_record(value: String) -> napi::Result<bool> {
    run(move || {
        let value = crate::js::from_json(&value)?;
        let out = port::is_record(value)?;
        Ok(out)
    })
}

/// `readText`.
#[napi(js_name = "readText")]
pub fn read_text(
    source: String,
    key: String,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let source = crate::js::from_json(&source)?;
        let out = port::read_text(source, key)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `readTexts`.
#[napi(js_name = "readTexts")]
pub fn read_texts(source: String, key: String) -> napi::Result<Vec<String>> {
    run(move || {
        let source = crate::js::from_json(&source)?;
        let out = port::read_texts(source, key)?;
        Ok(out)
    })
}

/// `readBoolean`.
#[napi(js_name = "readBoolean")]
pub fn read_boolean(source: String, key: String, fallback: bool) -> napi::Result<bool> {
    run(move || {
        let source = crate::js::from_json(&source)?;
        let out = port::read_boolean(source, key, fallback)?;
        Ok(out)
    })
}

/// `isJumboEmoji`.
#[napi(js_name = "isJumboEmoji")]
pub fn is_jumbo_emoji(text: String) -> napi::Result<bool> {
    run(move || {
        let out = port::is_jumbo_emoji(text)?;
        Ok(out)
    })
}

/// `isLikelyIn`.
#[napi(js_name = "isLikelyIn")]
pub fn is_likely_in(
    text: String,
    prompt: String,
) -> napi::Result<napi::bindgen_prelude::Either<bool, ()>> {
    run(move || {
        let out = port::is_likely_in(text, prompt)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `autoTranslationOf`.
#[napi(js_name = "autoTranslationOf")]
pub fn auto_translation_of(
    message: String,
    prompt: Option<String>,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let message = crate::js::from_json(&message)?;
        let out = port::auto_translation_of(message, prompt)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `parseGeoUri`.
#[napi(js_name = "parseGeoUri")]
pub fn parse_geo_uri(uri: String) -> napi::Result<String> {
    run(move || {
        let uri = crate::js::from_json(&uri)?;
        let out = port::parse_geo_uri(uri)?;
        crate::js::to_json(&out)
    })
}

/// `openStreetMapUrl`.
#[napi(js_name = "openStreetMapUrl")]
pub fn open_street_map_url(arg0: String) -> napi::Result<String> {
    run(move || {
        let arg0 = crate::js::from_json(&arg0)?;
        let out = port::open_street_map_url(arg0)?;
        Ok(out)
    })
}

/// `avatarThumbnailSize`.
#[napi(js_name = "avatarThumbnailSize")]
pub fn avatar_thumbnail_size(size: f64) -> napi::Result<f64> {
    run(move || {
        let out = port::avatar_thumbnail_size(size)?;
        Ok(out)
    })
}

/// `thumbnailMediaUrl`.
#[napi(js_name = "thumbnailMediaUrl")]
pub fn thumbnail_media_url(
    homeserver_url: String,
    mxc_url: String,
    size: f64,
    method: String,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let out = port::thumbnail_media_url(homeserver_url, mxc_url, size, method)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `pickAttendees`.
#[napi(js_name = "pickAttendees")]
pub fn pick_attendees(
    members: String,
    address_book: String,
    own_user_id: String,
) -> napi::Result<String> {
    run(move || {
        let members = crate::js::from_json(&members)?;
        let address_book = crate::js::from_json(&address_book)?;
        let out = port::pick_attendees(members, address_book, own_user_id)?;
        crate::js::to_json(&out)
    })
}

/// `newEventUrl`.
#[napi(js_name = "newEventUrl")]
pub fn new_event_url(calendar_url: String, emails: Vec<String>) -> napi::Result<String> {
    run(move || {
        let out = port::new_event_url(calendar_url, emails)?;
        Ok(out)
    })
}

/// `readCreatedMeeting`.
#[napi(js_name = "readCreatedMeeting")]
pub fn read_created_meeting(value: String) -> napi::Result<String> {
    run(move || {
        let value = crate::js::from_json(&value)?;
        let out = port::read_created_meeting(value)?;
        crate::js::to_json(&out)
    })
}

/// `meetingMessageText`.
#[napi(js_name = "meetingMessageText")]
pub fn meeting_message_text(
    meeting: String,
    lang: String,
    sender_zone: String,
) -> napi::Result<String> {
    run(move || {
        let meeting = crate::js::from_json(&meeting)?;
        let out = port::meeting_message_text(meeting, lang, sender_zone)?;
        Ok(out)
    })
}

/// `describeMeetingTime`.
#[napi(js_name = "describeMeetingTime")]
pub fn describe_meeting_time(
    start_ts: f64,
    end_ts: f64,
    now: f64,
    language: String,
) -> napi::Result<String> {
    run(move || {
        let out = port::describe_meeting_time(start_ts, end_ts, now, language)?;
        Ok(out)
    })
}

/// `describePeople`.
#[napi(js_name = "describePeople")]
pub fn describe_people(names: Vec<String>, total: String) -> napi::Result<String> {
    run(move || {
        let total = crate::js::from_json(&total)?;
        let out = port::describe_people(names, total)?;
        Ok(out)
    })
}

/// `matchMembers`.
#[napi(js_name = "matchMembers")]
pub fn match_members(
    members: String,
    query: String,
    own_user_id: String,
    limit: Option<f64>,
) -> napi::Result<String> {
    run(move || {
        let members = crate::js::from_json(&members)?;
        let out = port::match_members(members, query, own_user_id, limit)?;
        crate::js::to_json(&out)
    })
}

/// `mentionText`.
#[napi(js_name = "mentionText")]
pub fn mention_text(member: String) -> napi::Result<String> {
    run(move || {
        let member = crate::js::from_json(&member)?;
        let out = port::mention_text(member)?;
        Ok(out)
    })
}

/// `keepPresentMentions`.
#[napi(js_name = "keepPresentMentions")]
pub fn keep_present_mentions(text: String, mentions: String) -> napi::Result<String> {
    run(move || {
        let mentions = crate::js::from_json(&mentions)?;
        let out = port::keep_present_mentions(text, mentions)?;
        crate::js::to_json(&out)
    })
}

/// `namePattern`.
#[napi(js_name = "namePattern")]
pub fn name_pattern(name: String) -> napi::Result<String> {
    run(move || {
        let out = port::name_pattern(name)?;
        Ok(out)
    })
}

/// `matchRooms`.
#[napi(js_name = "matchRooms")]
pub fn match_rooms(rooms: String, query: String, limit: Option<f64>) -> napi::Result<String> {
    run(move || {
        let rooms = crate::js::from_json(&rooms)?;
        let out = port::match_rooms(rooms, query, limit)?;
        crate::js::to_json(&out)
    })
}

/// `roomMentionRef`.
#[napi(js_name = "roomMentionRef")]
pub fn room_mention_ref(room: String) -> napi::Result<String> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::room_mention_ref(room)?;
        Ok(out)
    })
}

/// `roomMentionText`.
#[napi(js_name = "roomMentionText")]
pub fn room_mention_text(room: String) -> napi::Result<String> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::room_mention_text(room)?;
        Ok(out)
    })
}

/// `isRoomMention`.
#[napi(js_name = "isRoomMention")]
pub fn is_room_mention(mention: String) -> napi::Result<bool> {
    run(move || {
        let mention = crate::js::from_json(&mention)?;
        let out = port::is_room_mention(mention)?;
        Ok(out)
    })
}

/// `isEveryoneMention`.
#[napi(js_name = "isEveryoneMention")]
pub fn is_everyone_mention(mention: String) -> napi::Result<bool> {
    run(move || {
        let mention = crate::js::from_json(&mention)?;
        let out = port::is_everyone_mention(mention)?;
        Ok(out)
    })
}

/// `offersEveryone`.
#[napi(js_name = "offersEveryone")]
pub fn offers_everyone(query: String) -> napi::Result<bool> {
    run(move || {
        let out = port::offers_everyone(query)?;
        Ok(out)
    })
}

/// `describeEmote`.
#[napi(js_name = "describeEmote")]
pub fn describe_emote(sender_name: String, text: String) -> napi::Result<String> {
    run(move || {
        let out = port::describe_emote(sender_name, text)?;
        Ok(out)
    })
}

/// `bareEmoji`.
#[napi(js_name = "bareEmoji")]
pub fn bare_emoji(emoji: String) -> napi::Result<String> {
    run(move || {
        let out = port::bare_emoji(emoji)?;
        Ok(out)
    })
}

/// `countsAsUnread`.
#[napi(js_name = "countsAsUnread")]
pub fn counts_as_unread(message: String) -> napi::Result<bool> {
    run(move || {
        let message = crate::js::from_json(&message)?;
        let out = port::counts_as_unread(message)?;
        Ok(out)
    })
}

/// `findFirstUnreadId`.
#[napi(js_name = "findFirstUnreadId")]
pub fn find_first_unread_id(
    messages: String,
    unread_count: f64,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let messages = crate::js::from_json(&messages)?;
        let out = port::find_first_unread_id(messages, unread_count)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `isSameDay`.
#[napi(js_name = "isSameDay")]
pub fn is_same_day(a: f64, b: f64) -> napi::Result<bool> {
    run(move || {
        let out = port::is_same_day(a, b)?;
        Ok(out)
    })
}

/// `isSelectable`.
#[napi(js_name = "isSelectable")]
pub fn is_selectable(message: String) -> napi::Result<bool> {
    run(move || {
        let message = crate::js::from_json(&message)?;
        let out = port::is_selectable(message)?;
        Ok(out)
    })
}

/// `startsGroup`.
#[napi(js_name = "startsGroup")]
pub fn starts_group(messages: String, index: f64) -> napi::Result<bool> {
    run(move || {
        let messages = crate::js::from_json(&messages)?;
        let out = port::starts_group(messages, index)?;
        Ok(out)
    })
}

/// `groupAlbums`.
#[napi(js_name = "groupAlbums")]
pub fn group_albums(
    messages: String,
    break_before: Option<String>,
    hides_gifs: Option<bool>,
) -> napi::Result<String> {
    run(move || {
        let messages = crate::js::from_json(&messages)?;
        let out = port::group_albums(messages, break_before, hides_gifs)?;
        crate::js::to_json(&out)
    })
}

/// `startsDay`.
#[napi(js_name = "startsDay")]
pub fn starts_day(messages: String, index: f64) -> napi::Result<bool> {
    run(move || {
        let messages = crate::js::from_json(&messages)?;
        let out = port::starts_day(messages, index)?;
        Ok(out)
    })
}

/// `isTranslatable`.
#[napi(js_name = "isTranslatable")]
pub fn is_translatable(message: String) -> napi::Result<bool> {
    run(move || {
        let message = crate::js::from_json(&message)?;
        let out = port::is_translatable(message)?;
        Ok(out)
    })
}

/// `isEditable`.
#[napi(js_name = "isEditable")]
pub fn is_editable(message: String) -> napi::Result<bool> {
    run(move || {
        let message = crate::js::from_json(&message)?;
        let out = port::is_editable(message)?;
        Ok(out)
    })
}

/// `shouldNotify`.
#[napi(js_name = "shouldNotify")]
pub fn should_notify(message: String, level: String, settings: String) -> napi::Result<bool> {
    run(move || {
        let message = crate::js::from_json(&message)?;
        let settings = crate::js::from_json(&settings)?;
        let out = port::should_notify(message, level, settings)?;
        Ok(out)
    })
}

/// `messageLink`.
#[napi(js_name = "messageLink")]
pub fn message_link(origin: String, room_id: String, event_id: String) -> napi::Result<String> {
    run(move || {
        let out = port::message_link(origin, room_id, event_id)?;
        Ok(out)
    })
}

/// `roomLink`.
#[napi(js_name = "roomLink")]
pub fn room_link(origin: String, room_id: String) -> napi::Result<String> {
    run(move || {
        let out = port::room_link(origin, room_id)?;
        Ok(out)
    })
}

/// `userPermalink`.
#[napi(js_name = "userPermalink")]
pub fn user_permalink(user_id: String) -> napi::Result<String> {
    run(move || {
        let out = port::user_permalink(user_id)?;
        Ok(out)
    })
}

/// `roomPermalink`.
#[napi(js_name = "roomPermalink")]
pub fn room_permalink(alias_or_id: String) -> napi::Result<String> {
    run(move || {
        let out = port::room_permalink(alias_or_id)?;
        Ok(out)
    })
}

/// `parsePermalink`.
#[napi(js_name = "parsePermalink")]
pub fn parse_permalink(href: String, app_origin: String) -> napi::Result<String> {
    run(move || {
        let out = port::parse_permalink(href, app_origin)?;
        crate::js::to_json(&out)
    })
}

/// `parseRoomAddress`.
#[napi(js_name = "parseRoomAddress")]
pub fn parse_room_address(text: String, own_server: String) -> napi::Result<String> {
    run(move || {
        let out = port::parse_room_address(text, own_server)?;
        crate::js::to_json(&out)
    })
}

/// `readSeenSteps`.
#[napi(js_name = "readSeenSteps")]
pub fn read_seen_steps(content: String) -> napi::Result<Vec<String>> {
    run(move || {
        let content = crate::js::from_json(&content)?;
        let out = port::read_seen_steps(content)?;
        Ok(out)
    })
}

/// `parseSeenSteps`.
#[napi(js_name = "parseSeenSteps")]
pub fn parse_seen_steps(stored: String) -> napi::Result<Vec<String>> {
    run(move || {
        let out = port::parse_seen_steps(stored)?;
        Ok(out)
    })
}

/// `unionSteps`.
#[napi(js_name = "unionSteps")]
pub fn union_steps(lists: String) -> napi::Result<Vec<String>> {
    run(move || {
        let lists = crate::js::from_json(&lists)?;
        let out = port::union_steps(lists)?;
        Ok(out)
    })
}

/// `tallyPoll`.
#[napi(js_name = "tallyPoll")]
pub fn tally_poll(
    poll: String,
    responses: String,
    ended_at: Option<f64>,
    user_id: String,
) -> napi::Result<String> {
    run(move || {
        let poll = crate::js::from_json(&poll)?;
        let responses = crate::js::from_json(&responses)?;
        let out = port::tally_poll(poll, responses, ended_at, user_id)?;
        crate::js::to_json(&out)
    })
}

/// `pressAnswer`.
#[napi(js_name = "pressAnswer")]
pub fn press_answer(poll: String, answer_id: String) -> napi::Result<String> {
    run(move || {
        let poll = crate::js::from_json(&poll)?;
        let out = port::press_answer(poll, answer_id)?;
        crate::js::to_json(&out)
    })
}

/// `readNewPoll`.
#[napi(js_name = "readNewPoll")]
pub fn read_new_poll(question: String, answers: Vec<String>) -> napi::Result<String> {
    run(move || {
        let out = port::read_new_poll(question, answers)?;
        crate::js::to_json(&out)
    })
}

/// `computeRoomUnread`.
#[napi(js_name = "computeRoomUnread")]
pub fn compute_room_unread(state: String, flags: String) -> napi::Result<String> {
    run(move || {
        let state = crate::js::from_json(&state)?;
        let flags = crate::js::from_json(&flags)?;
        let out = port::compute_room_unread(state, flags)?;
        crate::js::to_json(&out)
    })
}

/// `computeSilentUnread`.
#[napi(js_name = "computeSilentUnread")]
pub fn compute_silent_unread(state: String, flags: String) -> napi::Result<f64> {
    run(move || {
        let state = crate::js::from_json(&state)?;
        let flags = crate::js::from_json(&flags)?;
        let out = port::compute_silent_unread(state, flags)?;
        Ok(out)
    })
}

/// `isUnread`.
#[napi(js_name = "isUnread")]
pub fn is_unread(count: String) -> napi::Result<bool> {
    run(move || {
        let count = crate::js::from_json(&count)?;
        let out = port::is_unread(count)?;
        Ok(out)
    })
}

/// `computeTotalUnread`.
#[napi(js_name = "computeTotalUnread")]
pub fn compute_total_unread(counts: String) -> napi::Result<String> {
    run(move || {
        let counts = crate::js::from_json(&counts)?;
        let out = port::compute_total_unread(counts)?;
        crate::js::to_json(&out)
    })
}

/// `computeReceiptTargets`.
#[napi(js_name = "computeReceiptTargets")]
pub fn compute_receipt_targets(state: String) -> napi::Result<String> {
    run(move || {
        let state = crate::js::from_json(&state)?;
        let out = port::compute_receipt_targets(state)?;
        crate::js::to_json(&out)
    })
}

/// `isRoomUnread`.
#[napi(js_name = "isRoomUnread")]
pub fn is_room_unread(unread: String, silent_count: Option<f64>) -> napi::Result<bool> {
    run(move || {
        let unread = crate::js::from_json(&unread)?;
        let out = port::is_room_unread(unread, silent_count)?;
        Ok(out)
    })
}

/// `shownUnreadCount`.
#[napi(js_name = "shownUnreadCount")]
pub fn shown_unread_count(entry: String) -> napi::Result<f64> {
    run(move || {
        let entry = crate::js::from_json(&entry)?;
        let out = port::shown_unread_count(entry)?;
        Ok(out)
    })
}

/// `chooseReceiptEvent`.
#[napi(js_name = "chooseReceiptEvent")]
pub fn choose_receipt_event(
    ordered_ids: Vec<String>,
    unread_count: f64,
    shown_id: Option<String>,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let out = port::choose_receipt_event(ordered_ids, unread_count, shown_id)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `emptyReadState`.
#[napi(js_name = "emptyReadState")]
pub fn empty_read_state(room_id: String) -> napi::Result<String> {
    run(move || {
        let out = port::empty_read_state(room_id)?;
        crate::js::to_json(&out)
    })
}

/// `makeReadRoomState`.
#[napi(js_name = "makeReadRoomState")]
pub fn make_read_room_state(state: String) -> napi::Result<String> {
    run(move || {
        let state = crate::js::from_json(&state)?;
        let out = port::make_read_room_state(state)?;
        crate::js::to_json(&out)
    })
}

/// `hasGifsOff`.
#[napi(js_name = "hasGifsOff")]
pub fn has_gifs_off(room: String) -> napi::Result<bool> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::has_gifs_off(room)?;
        Ok(out)
    })
}

/// `offersScribe`.
#[napi(js_name = "offersScribe")]
pub fn offers_scribe(room: String) -> napi::Result<bool> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::offers_scribe(room)?;
        Ok(out)
    })
}

/// `spaceOfRoom`.
#[napi(js_name = "spaceOfRoom")]
pub fn space_of_room(room: String) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::space_of_room(room)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `canBeListenedTo`.
#[napi(js_name = "canBeListenedTo")]
pub fn can_be_listened_to(room: String) -> napi::Result<bool> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::can_be_listened_to(room)?;
        Ok(out)
    })
}

/// `computeRoomKind`.
#[napi(js_name = "computeRoomKind")]
pub fn compute_room_kind(inputs: String) -> napi::Result<String> {
    run(move || {
        let inputs = crate::js::from_json(&inputs)?;
        let out = port::compute_room_kind(inputs)?;
        Ok(out)
    })
}

/// `canPost`.
#[napi(js_name = "canPost")]
pub fn can_post(room: String, user_power_level: f64) -> napi::Result<bool> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::can_post(room, user_power_level)?;
        Ok(out)
    })
}

/// `findSpaceGeneral`.
#[napi(js_name = "findSpaceGeneral")]
pub fn find_space_general(rooms: String, space_id: String) -> napi::Result<String> {
    run(move || {
        let rooms = crate::js::from_json(&rooms)?;
        let out = port::find_space_general(rooms, space_id)?;
        crate::js::to_json(&out)
    })
}

/// `readArchive`.
#[napi(js_name = "readArchive")]
pub fn read_archive(content: String) -> napi::Result<String> {
    run(move || {
        let content = crate::js::from_json(&content)?;
        let out = port::read_archive(content)?;
        crate::js::to_json(&out)
    })
}

/// `withArchived`.
#[napi(js_name = "withArchived")]
pub fn with_archived(rooms: String, room_id: String, room: String) -> napi::Result<String> {
    run(move || {
        let rooms = crate::js::from_json(&rooms)?;
        let room = crate::js::from_json(&room)?;
        let out = port::with_archived(rooms, room_id, room)?;
        crate::js::to_json(&out)
    })
}

/// `toRoomLines`.
#[napi(js_name = "toRoomLines")]
pub fn to_room_lines(
    events: String,
    message_times: Vec<f64>,
    listener_id: Option<String>,
    deleted: String,
    room_id: String,
) -> napi::Result<String> {
    run(move || {
        let events = crate::js::from_json(&events)?;
        let deleted = crate::js::from_json(&deleted)?;
        let out = port::to_room_lines(events, message_times, listener_id, deleted, room_id)?;
        crate::js::to_json(&out)
    })
}

/// `folderView`.
#[napi(js_name = "folderView")]
pub fn folder_view(folder: String) -> napi::Result<String> {
    run(move || {
        let folder = crate::js::from_json(&folder)?;
        let out = port::folder_view(folder)?;
        crate::js::to_json(&out)
    })
}

/// `isInView`.
#[napi(js_name = "isInView")]
pub fn is_in_view(entry: String, view: String, all: String, folders: String) -> napi::Result<bool> {
    run(move || {
        let entry = crate::js::from_json(&entry)?;
        let view = crate::js::from_json(&view)?;
        let all = crate::js::from_json(&all)?;
        let folders = crate::js::from_json(&folders)?;
        let out = port::is_in_view(entry, view, all, folders)?;
        Ok(out)
    })
}

/// `isOnOtherServer`.
#[napi(js_name = "isOnOtherServer")]
pub fn is_on_other_server(user_id: Option<String>, own_user_id: String) -> napi::Result<bool> {
    run(move || {
        let out = port::is_on_other_server(user_id, own_user_id)?;
        Ok(out)
    })
}

/// `levelOfRole`.
#[napi(js_name = "levelOfRole")]
pub fn level_of_role(role: String) -> napi::Result<f64> {
    run(move || {
        let out = port::level_of_role(role)?;
        Ok(out)
    })
}

/// `computeRole`.
#[napi(js_name = "computeRole")]
pub fn compute_role(power_level: f64) -> napi::Result<String> {
    run(move || {
        let out = port::compute_role(power_level)?;
        Ok(out)
    })
}

/// `isRoomOwner`.
#[napi(js_name = "isRoomOwner")]
pub fn is_room_owner(permissions: String) -> napi::Result<bool> {
    run(move || {
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::is_room_owner(permissions)?;
        Ok(out)
    })
}

/// `canManageMember`.
#[napi(js_name = "canManageMember")]
pub fn can_manage_member(member: String, permissions: String, is_self: bool) -> napi::Result<bool> {
    run(move || {
        let member = crate::js::from_json(&member)?;
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::can_manage_member(member, permissions, is_self)?;
        Ok(out)
    })
}

/// `canChangeRole`.
#[napi(js_name = "canChangeRole")]
pub fn can_change_role(member: String, permissions: String, is_self: bool) -> napi::Result<bool> {
    run(move || {
        let member = crate::js::from_json(&member)?;
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::can_change_role(member, permissions, is_self)?;
        Ok(out)
    })
}

/// `assignableRoles`.
#[napi(js_name = "assignableRoles")]
pub fn assignable_roles(
    member: String,
    permissions: String,
    is_self: bool,
) -> napi::Result<Vec<String>> {
    run(move || {
        let member = crate::js::from_json(&member)?;
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::assignable_roles(member, permissions, is_self)?;
        Ok(out)
    })
}

/// `canBanMember`.
#[napi(js_name = "canBanMember")]
pub fn can_ban_member(member: String, permissions: String, is_self: bool) -> napi::Result<bool> {
    run(move || {
        let member = crate::js::from_json(&member)?;
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::can_ban_member(member, permissions, is_self)?;
        Ok(out)
    })
}

/// `canSetReadOnly`.
#[napi(js_name = "canSetReadOnly")]
pub fn can_set_read_only(member: String, permissions: String, is_self: bool) -> napi::Result<bool> {
    run(move || {
        let member = crate::js::from_json(&member)?;
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::can_set_read_only(member, permissions, is_self)?;
        Ok(out)
    })
}

/// `ownershipAction`.
#[napi(js_name = "ownershipAction")]
pub fn ownership_action(
    permissions: String,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::ownership_action(permissions)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `canTakeOwnership`.
#[napi(js_name = "canTakeOwnership")]
pub fn can_take_ownership(
    member: String,
    permissions: String,
    is_self: bool,
) -> napi::Result<bool> {
    run(move || {
        let member = crate::js::from_json(&member)?;
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::can_take_ownership(member, permissions, is_self)?;
        Ok(out)
    })
}

/// `canTransferTo`.
#[napi(js_name = "canTransferTo")]
pub fn can_transfer_to(member: String, permissions: String, is_self: bool) -> napi::Result<bool> {
    run(move || {
        let member = crate::js::from_json(&member)?;
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::can_transfer_to(member, permissions, is_self)?;
        Ok(out)
    })
}

/// `hasStaffRole`.
#[napi(js_name = "hasStaffRole")]
pub fn has_staff_role(member: String) -> napi::Result<bool> {
    run(move || {
        let member = crate::js::from_json(&member)?;
        let out = port::has_staff_role(member)?;
        Ok(out)
    })
}

/// `permissionRoleOf`.
#[napi(js_name = "permissionRoleOf")]
pub fn permission_role_of(member: String, permissions: String) -> napi::Result<String> {
    run(move || {
        let member = crate::js::from_json(&member)?;
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::permission_role_of(member, permissions)?;
        crate::js::to_json(&out)
    })
}

/// `givenRoles`.
#[napi(js_name = "givenRoles")]
pub fn given_roles(permissions: String) -> napi::Result<Vec<String>> {
    run(move || {
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::given_roles(permissions)?;
        Ok(out)
    })
}

/// `isValidSlug`.
#[napi(js_name = "isValidSlug")]
pub fn is_valid_slug(slug: String) -> napi::Result<bool> {
    run(move || {
        let out = port::is_valid_slug(slug)?;
        Ok(out)
    })
}

/// `canHavePublicPage`.
#[napi(js_name = "canHavePublicPage")]
pub fn can_have_public_page(room: String) -> napi::Result<bool> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::can_have_public_page(room)?;
        Ok(out)
    })
}

/// `publicPagePath`.
#[napi(js_name = "publicPagePath")]
pub fn public_page_path(room: String) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let out = port::public_page_path(room)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `suggestSlug`.
#[napi(js_name = "suggestSlug")]
pub fn suggest_slug(name: String) -> napi::Result<String> {
    run(move || {
        let out = port::suggest_slug(name)?;
        Ok(out)
    })
}

/// `isValidColor`.
#[napi(js_name = "isValidColor")]
pub fn is_valid_color(color: String) -> napi::Result<bool> {
    run(move || {
        let out = port::is_valid_color(color)?;
        Ok(out)
    })
}

/// `parseMatrixUserId`.
#[napi(js_name = "parseMatrixUserId")]
pub fn parse_matrix_user_id(
    text: String,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let out = port::parse_matrix_user_id(text)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `toPreviewText`.
#[napi(js_name = "toPreviewText")]
pub fn to_preview_text(text: String) -> napi::Result<String> {
    run(move || {
        let out = port::to_preview_text(text)?;
        Ok(out)
    })
}

/// `roomIdOfRoute`.
#[napi(js_name = "roomIdOfRoute")]
pub fn room_id_of_route(route_id: String, rooms: String) -> napi::Result<String> {
    run(move || {
        let rooms = crate::js::from_json(&rooms)?;
        let out = port::room_id_of_route(route_id, rooms)?;
        Ok(out)
    })
}

/// `readSavedRefs`.
#[napi(js_name = "readSavedRefs")]
pub fn read_saved_refs(content: String) -> napi::Result<String> {
    run(move || {
        let content = crate::js::from_json(&content)?;
        let out = port::read_saved_refs(content)?;
        crate::js::to_json(&out)
    })
}

/// `withSaved`.
#[napi(js_name = "withSaved")]
pub fn with_saved(
    refs: String,
    room_id: String,
    event_id: String,
    is_saved: bool,
    now: String,
) -> napi::Result<String> {
    run(move || {
        let refs = crate::js::from_json(&refs)?;
        let now = crate::js::from_json(&now)?;
        let out = port::with_saved(refs, room_id, event_id, is_saved, now)?;
        crate::js::to_json(&out)
    })
}

/// `readAutoTranslate`.
#[napi(js_name = "readAutoTranslate")]
pub fn read_auto_translate(
    content: String,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let content = crate::js::from_json(&content)?;
        let out = port::read_auto_translate(content)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `normalizeForSearch`.
#[napi(js_name = "normalizeForSearch")]
pub fn normalize_for_search(text: String) -> napi::Result<String> {
    run(move || {
        let out = port::normalize_for_search(text)?;
        Ok(out)
    })
}

/// `isClosedQuery`.
#[napi(js_name = "isClosedQuery")]
pub fn is_closed_query(query: String) -> napi::Result<bool> {
    run(move || {
        let out = port::is_closed_query(query)?;
        Ok(out)
    })
}

/// `holdsWord`.
#[napi(js_name = "holdsWord")]
pub fn holds_word(haystack: String, word: String, whole: bool) -> napi::Result<bool> {
    run(move || {
        let out = port::holds_word(haystack, word, whole)?;
        Ok(out)
    })
}

/// `toIndexedMessage`.
#[napi(js_name = "toIndexedMessage")]
pub fn to_indexed_message(room_id: String, message: String) -> napi::Result<String> {
    run(move || {
        let message = crate::js::from_json(&message)?;
        let out = port::to_indexed_message(room_id, message)?;
        crate::js::to_json(&out)
    })
}

/// `isLocalEchoId`.
#[napi(js_name = "isLocalEchoId")]
pub fn is_local_echo_id(message_id: String) -> napi::Result<bool> {
    run(move || {
        let out = port::is_local_echo_id(message_id)?;
        Ok(out)
    })
}

/// `isEmptyFilter`.
#[napi(js_name = "isEmptyFilter")]
pub fn is_empty_filter(filter: String) -> napi::Result<bool> {
    run(move || {
        let filter = crate::js::from_json(&filter)?;
        let out = port::is_empty_filter(filter)?;
        Ok(out)
    })
}

/// `matchesFilter`.
#[napi(js_name = "matchesFilter")]
pub fn matches_filter(entry: String, filter: String) -> napi::Result<bool> {
    run(move || {
        let entry = crate::js::from_json(&entry)?;
        let filter = crate::js::from_json(&filter)?;
        let out = port::matches_filter(entry, filter)?;
        Ok(out)
    })
}

/// `isServiceUrl`.
#[napi(js_name = "isServiceUrl")]
pub fn is_service_url(value: String) -> napi::Result<bool> {
    run(move || {
        let out = port::is_service_url(value)?;
        Ok(out)
    })
}

/// `extractLinks`.
#[napi(js_name = "extractLinks")]
pub fn extract_links(text: String) -> napi::Result<Vec<String>> {
    run(move || {
        let out = port::extract_links(text)?;
        Ok(out)
    })
}

/// `computeSharedContent`.
#[napi(js_name = "computeSharedContent")]
pub fn compute_shared_content(messages: String) -> napi::Result<String> {
    run(move || {
        let messages = crate::js::from_json(&messages)?;
        let out = port::compute_shared_content(messages)?;
        crate::js::to_json(&out)
    })
}

/// `roomBadgeCount`.
#[napi(js_name = "roomBadgeCount")]
pub fn room_badge_count(unread: String) -> napi::Result<f64> {
    run(move || {
        let unread = crate::js::from_json(&unread)?;
        let out = port::room_badge_count(unread)?;
        Ok(out)
    })
}

/// `computeSpaceBadges`.
#[napi(js_name = "computeSpaceBadges")]
pub fn compute_space_badges(entries: String) -> napi::Result<String> {
    run(move || {
        let entries = crate::js::from_json(&entries)?;
        let out = port::compute_space_badges(entries)?;
        crate::js::to_json(&out)
    })
}

/// `canMoveToSpace`.
#[napi(js_name = "canMoveToSpace")]
pub fn can_move_to_space(room: String, permissions: String) -> napi::Result<bool> {
    run(move || {
        let room = crate::js::from_json(&room)?;
        let permissions = crate::js::from_json(&permissions)?;
        let out = port::can_move_to_space(room, permissions)?;
        Ok(out)
    })
}

/// `arrangeRooms`.
#[napi(js_name = "arrangeRooms")]
pub fn arrange_rooms(items: String, pinned_first: Option<bool>) -> napi::Result<String> {
    run(move || {
        let items = crate::js::from_json(&items)?;
        let out = port::arrange_rooms(items, pinned_first)?;
        crate::js::to_json(&out)
    })
}

/// `staysInTree`.
#[napi(js_name = "staysInTree")]
pub fn stays_in_tree(item: String) -> napi::Result<bool> {
    run(move || {
        let item = crate::js::from_json(&item)?;
        let out = port::stays_in_tree(item)?;
        Ok(out)
    })
}

/// `parentSpaceOf`.
#[napi(js_name = "parentSpaceOf")]
pub fn parent_space_of(item: String, items: String) -> napi::Result<String> {
    run(move || {
        let item = crate::js::from_json(&item)?;
        let items = crate::js::from_json(&items)?;
        let out = port::parent_space_of(item, items)?;
        crate::js::to_json(&out)
    })
}

/// `nonBlank`.
#[napi(js_name = "nonBlank")]
pub fn non_blank(text: String) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let out = port::non_blank(text)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `stripBidi`.
#[napi(js_name = "stripBidi")]
pub fn strip_bidi(text: String) -> napi::Result<String> {
    run(move || {
        let out = port::strip_bidi(text)?;
        Ok(out)
    })
}

/// `normalizeStatus`.
#[napi(js_name = "normalizeStatus")]
pub fn normalize_status(
    text: Option<String>,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let out = port::normalize_status(text)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `isNameTooLong`.
#[napi(js_name = "isNameTooLong")]
pub fn is_name_too_long(name: String) -> napi::Result<bool> {
    run(move || {
        let out = port::is_name_too_long(name)?;
        Ok(out)
    })
}

/// `presenceOfPreset`.
#[napi(js_name = "presenceOfPreset")]
pub fn presence_of_preset(preset: String) -> napi::Result<String> {
    run(move || {
        let preset = crate::js::from_json(&preset)?;
        let out = port::presence_of_preset(preset)?;
        Ok(out)
    })
}

/// `computeThreadTitle`.
#[napi(js_name = "computeThreadTitle")]
pub fn compute_thread_title(root_text: String) -> napi::Result<String> {
    run(move || {
        let out = port::compute_thread_title(root_text)?;
        Ok(out)
    })
}

/// `sortThreadsByActivity`.
#[napi(js_name = "sortThreadsByActivity")]
pub fn sort_threads_by_activity(threads: String) -> napi::Result<String> {
    run(move || {
        let threads = crate::js::from_json(&threads)?;
        let out = port::sort_threads_by_activity(threads)?;
        crate::js::to_json(&out)
    })
}

/// `readThreadPrefs`.
#[napi(js_name = "readThreadPrefs")]
pub fn read_thread_prefs(content: String) -> napi::Result<String> {
    run(move || {
        let content = crate::js::from_json(&content)?;
        let out = port::read_thread_prefs(content)?;
        crate::js::to_json(&out)
    })
}

/// `withThreadPref`.
#[napi(js_name = "withThreadPref")]
pub fn with_thread_pref(
    entries: String,
    room_id: String,
    thread_id: String,
    pref: String,
) -> napi::Result<String> {
    run(move || {
        let entries = crate::js::from_json(&entries)?;
        let out = port::with_thread_pref(entries, room_id, thread_id, pref)?;
        crate::js::to_json(&out)
    })
}

/// `resolveThreadFollow`.
#[napi(js_name = "resolveThreadFollow")]
pub fn resolve_thread_follow(is_participating: bool, pref: Option<String>) -> napi::Result<String> {
    run(move || {
        let out = port::resolve_thread_follow(is_participating, pref)?;
        crate::js::to_json(&out)
    })
}

/// `timelineWindowOf`.
#[napi(js_name = "timelineWindowOf")]
pub fn timeline_window_of(
    messages: String,
    lines: String,
    start: String,
    reach: String,
) -> napi::Result<String> {
    run(move || {
        let messages = crate::js::from_json(&messages)?;
        let lines = crate::js::from_json(&lines)?;
        let start = crate::js::from_json(&start)?;
        let reach = crate::js::from_json(&reach)?;
        let out = port::timeline_window_of(messages, lines, start, reach)?;
        crate::js::to_json(&out)
    })
}

/// `olderStartOf`.
#[napi(js_name = "olderStartOf")]
pub fn older_start_of(messages: String, hidden_above: f64) -> napi::Result<String> {
    run(move || {
        let messages = crate::js::from_json(&messages)?;
        let out = port::older_start_of(messages, hidden_above)?;
        crate::js::to_json(&out)
    })
}

/// `commonSettingsUrlFor`.
#[napi(js_name = "commonSettingsUrlFor")]
pub fn common_settings_url_for(
    settings: String,
    user_id: String,
) -> napi::Result<napi::bindgen_prelude::Either<String, ()>> {
    run(move || {
        let settings = crate::js::from_json(&settings)?;
        let out = port::common_settings_url_for(settings, user_id)?;
        Ok(out.map_or(
            napi::bindgen_prelude::Either::B(()),
            napi::bindgen_prelude::Either::A,
        ))
    })
}

/// `findWebLinks`.
#[napi(js_name = "findWebLinks")]
pub fn find_web_links(text: String) -> napi::Result<String> {
    run(move || {
        let out = port::find_web_links(text)?;
        crate::js::to_json(&out)
    })
}

/// `toHref`.
#[napi(js_name = "toHref")]
pub fn to_href(url: String) -> napi::Result<String> {
    run(move || {
        let out = port::to_href(url)?;
        Ok(out)
    })
}

// ---- the binary wire ------------------------------------------------------------------------

thread_local! {
    static WIRE: std::cell::RefCell<Option<(napi::bindgen_prelude::Uint8Array, napi::bindgen_prelude::Uint8Array)>> =
        const { std::cell::RefCell::new(None) };
    static PENDING: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Registers the argument and result buffers `index.cjs` writes to and reads from.
#[napi(js_name = "rustAnythingWireInit")]
pub fn rust_anything_wire_init(
    input: napi::bindgen_prelude::Uint8Array,
    output: napi::bindgen_prelude::Uint8Array,
) {
    WIRE.with(|w| *w.borrow_mut() = Some((input, output)));
}

/// Copies a result that did not fit into the (now larger) result buffer.
#[napi(js_name = "rustAnythingWireFlush")]
pub fn rust_anything_wire_flush() -> f64 {
    let bytes = PENDING.with(|p| std::mem::take(&mut *p.borrow_mut()));
    WIRE.with(|w| {
        let mut w = w.borrow_mut();
        let Some((_, output)) = w.as_mut() else {
            return -1.0;
        };
        // SAFETY: the call is synchronous; JS does not touch `output` until it returns.
        let out = unsafe { output.as_mut() };
        let n = bytes.len().min(out.len());
        out[..n].copy_from_slice(&bytes[..n]);
        n as f64
    })
}

/// Reads `len` argument bytes, runs `f`, and writes its result; a negative return is the size
/// of a result that did not fit, kept for `rustAnythingWireFlush`.
fn wire_io(
    len: u32,
    f: impl FnOnce(&mut wire::Reader, &mut wire::Writer) -> Result<(), String>,
) -> napi::Result<f64> {
    WIRE.with(|cell| {
        let mut guard = cell.borrow_mut();
        let Some((input, output)) = guard.as_mut() else {
            return Err(napi::Error::from_reason(
                "rust-anything: the wire buffers are not registered",
            ));
        };
        let len = (len as usize).min(input.len());
        let mut w = wire::Writer::default();
        match run(|| f(&mut wire::Reader::new(&input[..len]), &mut w)) {
            // A result of another shape than the TS type: JS runs the call through JSON.
            Err(e) if e.reason.starts_with(wire::MISMATCH) => return Ok(f64::NAN),
            other => other?,
        }
        // SAFETY: the call is synchronous; JS does not touch `output` until it returns.
        let out = unsafe { output.as_mut() };
        let n = w.0.len();
        if n > out.len() {
            PENDING.with(|p| *p.borrow_mut() = w.0);
            return Ok(-(n as f64));
        }
        out[..n].copy_from_slice(&w.0);
        Ok(n as f64)
    })
}

static S_BUILD_ACTIVITY_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("roomName", wire::Schema::Str16),
    (
        "message",
        wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("senderId", wire::Schema::Str16),
            ("senderName", wire::Schema::Str16),
            ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isFromBot", wire::Schema::Bool),
            ("text", wire::Schema::Str16),
            ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("undecryptable", wire::Schema::Dyn),
            ("notice", wire::Schema::Dyn),
            ("attachment", wire::Schema::Dyn),
            (
                "location",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("lat", wire::Schema::Dyn),
                    ("lon", wire::Schema::Dyn),
                ])),
            ),
            (
                "poll",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("question", wire::Schema::Dyn),
                    ("answers", wire::Schema::Dyn),
                    ("maxSelections", wire::Schema::Dyn),
                    ("isDisclosed", wire::Schema::Dyn),
                    ("isAnonymous", wire::Schema::Dyn),
                    ("endedAt", wire::Schema::Dyn),
                    ("votes", wire::Schema::Dyn),
                    ("voters", wire::Schema::Dyn),
                    ("totalVotes", wire::Schema::Dyn),
                    ("ownAnswerIds", wire::Schema::Dyn),
                    ("canVote", wire::Schema::Dyn),
                    ("canEnd", wire::Schema::Dyn),
                    ("canEdit", wire::Schema::Dyn),
                ])),
            ),
            ("question", wire::Schema::Dyn),
            ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "call",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("direction", wire::Schema::Dyn),
                    ("isVideo", wire::Schema::Dyn),
                    ("outcome", wire::Schema::Dyn),
                    ("durationMs", wire::Schema::Dyn),
                ])),
            ),
            ("sentAt", wire::Schema::Num),
            ("isOwn", wire::Schema::Bool),
            ("isReadByOthers", wire::Schema::Bool),
            (
                "lastReadBy",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
            ),
            ("thread", wire::Schema::Dyn),
            ("replyTo", wire::Schema::Dyn),
            ("isEdited", wire::Schema::Bool),
            ("reactions", wire::Schema::List(&wire::Schema::Dyn)),
            ("sendStatus", wire::Schema::Str16),
        ]),
    ),
]));
static S_BUILD_ACTIVITY_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("roomName", wire::Schema::Str16),
    (
        "thread",
        wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            (
                "unread",
                wire::Schema::Obj(&[
                    ("unreadCount", wire::Schema::Dyn),
                    ("mentionCount", wire::Schema::Dyn),
                    ("isMarkedUnread", wire::Schema::Dyn),
                ]),
            ),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]),
    ),
]));
static S_BUILD_ACTIVITY_2: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
        ("roomId", wire::Schema::Str16),
        ("roomName", wire::Schema::Str16),
        ("key", wire::Schema::Str16),
        ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("senderName", wire::Schema::Str16),
        ("reactedAt", wire::Schema::Num),
        (
            "message",
            wire::Schema::Obj(&[
                ("id", wire::Schema::Str16),
                ("senderId", wire::Schema::Str16),
                ("senderName", wire::Schema::Str16),
                ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
                ("isFromBot", wire::Schema::Bool),
                ("text", wire::Schema::Str16),
                ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
                ("undecryptable", wire::Schema::Dyn),
                ("notice", wire::Schema::Dyn),
                ("attachment", wire::Schema::Dyn),
                (
                    "location",
                    wire::Schema::OptJs(&wire::Schema::Obj(&[
                        ("lat", wire::Schema::Dyn),
                        ("lon", wire::Schema::Dyn),
                    ])),
                ),
                (
                    "poll",
                    wire::Schema::OptJs(&wire::Schema::Obj(&[
                        ("question", wire::Schema::Dyn),
                        ("answers", wire::Schema::Dyn),
                        ("maxSelections", wire::Schema::Dyn),
                        ("isDisclosed", wire::Schema::Dyn),
                        ("isAnonymous", wire::Schema::Dyn),
                        ("endedAt", wire::Schema::Dyn),
                        ("votes", wire::Schema::Dyn),
                        ("voters", wire::Schema::Dyn),
                        ("totalVotes", wire::Schema::Dyn),
                        ("ownAnswerIds", wire::Schema::Dyn),
                        ("canVote", wire::Schema::Dyn),
                        ("canEnd", wire::Schema::Dyn),
                        ("canEdit", wire::Schema::Dyn),
                    ])),
                ),
                ("question", wire::Schema::Dyn),
                ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
                ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
                ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
                (
                    "call",
                    wire::Schema::OptJs(&wire::Schema::Obj(&[
                        ("direction", wire::Schema::Dyn),
                        ("isVideo", wire::Schema::Dyn),
                        ("outcome", wire::Schema::Dyn),
                        ("durationMs", wire::Schema::Dyn),
                    ])),
                ),
                ("sentAt", wire::Schema::Num),
                ("isOwn", wire::Schema::Bool),
                ("isReadByOthers", wire::Schema::Bool),
                (
                    "lastReadBy",
                    wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
                ),
                ("thread", wire::Schema::Dyn),
                ("replyTo", wire::Schema::Dyn),
                ("isEdited", wire::Schema::Bool),
                ("reactions", wire::Schema::List(&wire::Schema::Dyn)),
                ("sendStatus", wire::Schema::Str16),
            ]),
        ),
    ])));
static S_BUILD_ACTIVITY_3: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
        ("roomId", wire::Schema::Str16),
        ("roomName", wire::Schema::Str16),
        ("eventId", wire::Schema::Str16),
        ("callerId", wire::Schema::Str16),
        ("callerName", wire::Schema::Str16),
        ("callerAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("calledAt", wire::Schema::Num),
        ("canCallBack", wire::Schema::Bool),
    ])));
static S_BUILD_ACTIVITY_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "mentions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("roomId", wire::Schema::Str16),
            ("roomName", wire::Schema::Str16),
            (
                "message",
                wire::Schema::Obj(&[
                    ("id", wire::Schema::Dyn),
                    ("senderId", wire::Schema::Dyn),
                    ("senderName", wire::Schema::Dyn),
                    ("senderAvatarUrl", wire::Schema::Dyn),
                    ("isFromBot", wire::Schema::Dyn),
                    ("text", wire::Schema::Dyn),
                    ("html", wire::Schema::Dyn),
                    ("undecryptable", wire::Schema::Dyn),
                    ("notice", wire::Schema::Dyn),
                    ("attachment", wire::Schema::Dyn),
                    ("location", wire::Schema::Dyn),
                    ("poll", wire::Schema::Dyn),
                    ("question", wire::Schema::Dyn),
                    ("isEmote", wire::Schema::Dyn),
                    ("mentionsRoom", wire::Schema::Dyn),
                    ("isCallThreadRoot", wire::Schema::Dyn),
                    ("call", wire::Schema::Dyn),
                    ("sentAt", wire::Schema::Dyn),
                    ("isOwn", wire::Schema::Dyn),
                    ("isReadByOthers", wire::Schema::Dyn),
                    ("lastReadBy", wire::Schema::Dyn),
                    ("thread", wire::Schema::Dyn),
                    ("replyTo", wire::Schema::Dyn),
                    ("isEdited", wire::Schema::Dyn),
                    ("reactions", wire::Schema::Dyn),
                    ("sendStatus", wire::Schema::Dyn),
                ]),
            ),
        ])),
    ),
    (
        "threads",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("roomId", wire::Schema::Str16),
            ("roomName", wire::Schema::Str16),
            (
                "thread",
                wire::Schema::Obj(&[
                    ("threadId", wire::Schema::Dyn),
                    ("title", wire::Schema::Dyn),
                    ("replyCount", wire::Schema::Dyn),
                    ("lastReply", wire::Schema::Dyn),
                    ("unread", wire::Schema::Dyn),
                    ("isParticipating", wire::Schema::Dyn),
                    ("isMuted", wire::Schema::Dyn),
                    ("isResolved", wire::Schema::Dyn),
                    ("lastOwnReplyAt", wire::Schema::Dyn),
                ]),
            ),
        ])),
    ),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("roomId", wire::Schema::Str16),
            ("roomName", wire::Schema::Str16),
            ("key", wire::Schema::Str16),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderName", wire::Schema::Str16),
            ("reactedAt", wire::Schema::Num),
            (
                "message",
                wire::Schema::Obj(&[
                    ("id", wire::Schema::Dyn),
                    ("senderId", wire::Schema::Dyn),
                    ("senderName", wire::Schema::Dyn),
                    ("senderAvatarUrl", wire::Schema::Dyn),
                    ("isFromBot", wire::Schema::Dyn),
                    ("text", wire::Schema::Dyn),
                    ("html", wire::Schema::Dyn),
                    ("undecryptable", wire::Schema::Dyn),
                    ("notice", wire::Schema::Dyn),
                    ("attachment", wire::Schema::Dyn),
                    ("location", wire::Schema::Dyn),
                    ("poll", wire::Schema::Dyn),
                    ("question", wire::Schema::Dyn),
                    ("isEmote", wire::Schema::Dyn),
                    ("mentionsRoom", wire::Schema::Dyn),
                    ("isCallThreadRoot", wire::Schema::Dyn),
                    ("call", wire::Schema::Dyn),
                    ("sentAt", wire::Schema::Dyn),
                    ("isOwn", wire::Schema::Dyn),
                    ("isReadByOthers", wire::Schema::Dyn),
                    ("lastReadBy", wire::Schema::Dyn),
                    ("thread", wire::Schema::Dyn),
                    ("replyTo", wire::Schema::Dyn),
                    ("isEdited", wire::Schema::Dyn),
                    ("reactions", wire::Schema::Dyn),
                    ("sendStatus", wire::Schema::Dyn),
                ]),
            ),
        ])),
    ),
    (
        "missedCalls",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("roomId", wire::Schema::Str16),
            ("roomName", wire::Schema::Str16),
            ("eventId", wire::Schema::Str16),
            ("callerId", wire::Schema::Str16),
            ("callerName", wire::Schema::Str16),
            ("callerAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("calledAt", wire::Schema::Num),
            ("canCallBack", wire::Schema::Bool),
        ])),
    ),
]);

/// `buildActivity` over the binary wire.
#[napi(js_name = "buildActivityWire")]
pub fn build_activity_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let mentions: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_BUILD_ACTIVITY_0.inner()))?;
        let threads: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_BUILD_ACTIVITY_1.inner()))?;
        let reactions: serde_json::Value = wire::read_value(r, &S_BUILD_ACTIVITY_2)?;
        let missed_calls: serde_json::Value = wire::read_value(r, &S_BUILD_ACTIVITY_3)?;
        let out = &port::build_activity(mentions, threads, reactions, missed_calls)?;
        wire::write_value(w, out, &S_BUILD_ACTIVITY_OUT)?;
        Ok(())
    })
}

static S_BUILD_FEED_0: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    (
        "mentions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("roomId", wire::Schema::Str16),
            ("roomName", wire::Schema::Str16),
            ("message", wire::Schema::Dyn),
        ])),
    ),
    (
        "threads",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("roomId", wire::Schema::Str16),
            ("roomName", wire::Schema::Str16),
            ("thread", wire::Schema::Dyn),
        ])),
    ),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("roomId", wire::Schema::Str16),
            ("roomName", wire::Schema::Str16),
            ("key", wire::Schema::Str16),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderName", wire::Schema::Str16),
            ("reactedAt", wire::Schema::Num),
            ("message", wire::Schema::Dyn),
        ])),
    ),
    (
        "missedCalls",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("roomId", wire::Schema::Str16),
            ("roomName", wire::Schema::Str16),
            ("eventId", wire::Schema::Str16),
            ("callerId", wire::Schema::Str16),
            ("callerName", wire::Schema::Str16),
            ("callerAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("calledAt", wire::Schema::Num),
            ("canCallBack", wire::Schema::Bool),
        ])),
    ),
]));
static S_BUILD_FEED_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("roomName", wire::Schema::Str16),
    ("unreadCount", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "lastMessage",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Dyn),
            ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("text", wire::Schema::Str16),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderName", wire::Schema::Str16),
            ("isOwn", wire::Schema::Bool),
            ("sentAt", wire::Schema::Num),
            ("tick", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
]));
static S_BUILD_FEED_2: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomName", wire::Schema::Str16),
    (
        "message",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("senderId", wire::Schema::Str16),
            ("senderName", wire::Schema::Str16),
            ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isFromBot", wire::Schema::Bool),
            ("text", wire::Schema::Str16),
            ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("undecryptable", wire::Schema::Dyn),
            ("notice", wire::Schema::Dyn),
            ("attachment", wire::Schema::Dyn),
            ("location", wire::Schema::Dyn),
            ("poll", wire::Schema::Dyn),
            ("question", wire::Schema::Dyn),
            ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("call", wire::Schema::Dyn),
            ("sentAt", wire::Schema::Num),
            ("isOwn", wire::Schema::Bool),
            ("isReadByOthers", wire::Schema::Bool),
            (
                "lastReadBy",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
            ),
            ("thread", wire::Schema::Dyn),
            ("replyTo", wire::Schema::Dyn),
            ("isEdited", wire::Schema::Bool),
            ("reactions", wire::Schema::List(&wire::Schema::Dyn)),
            ("sendStatus", wire::Schema::Dyn),
        ])),
    ),
]));
static S_BUILD_FEED_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("roomId", wire::Schema::Str16),
    ("roomName", wire::Schema::Str16),
    ("at", wire::Schema::Num),
    ("actorName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("actorAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("quote", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("count", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("threadTitle", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isResolved", wire::Schema::Bool),
    ("canCallBack", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("target", wire::Schema::Dyn),
]));

/// `buildFeed` over the binary wire.
#[napi(js_name = "buildFeedWire")]
pub fn build_feed_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let activity: Option<serde_json::Value> =
            r.opt_js(|r| wire::read_value(r, S_BUILD_FEED_0.inner()))?;
        let unread: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_BUILD_FEED_1.inner()))?;
        let saved: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_BUILD_FEED_2.inner()))?;
        let out = &port::build_feed(activity, unread, saved)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_BUILD_FEED_OUT.inner())?;
        }
        Ok(())
    })
}

static S_FILTER_FEED_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("roomId", wire::Schema::Str16),
    ("roomName", wire::Schema::Str16),
    ("at", wire::Schema::Num),
    ("actorName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("actorAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("quote", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("count", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("threadTitle", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isResolved", wire::Schema::Bool),
    ("canCallBack", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("target", wire::Schema::Dyn),
]));
static S_FILTER_FEED_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("roomId", wire::Schema::Str16),
    ("roomName", wire::Schema::Str16),
    ("at", wire::Schema::Num),
    ("actorName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("actorAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("quote", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("count", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("threadTitle", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isResolved", wire::Schema::Bool),
    ("canCallBack", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("target", wire::Schema::Dyn),
]));

/// `filterFeed` over the binary wire.
#[napi(js_name = "filterFeedWire")]
pub fn filter_feed_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let items: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_FILTER_FEED_0.inner()))?;
        let filter: String = r.string16()?;
        let out = &port::filter_feed(items, filter)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_FILTER_FEED_OUT.inner())?;
        }
        Ok(())
    })
}

static S_SEARCH_FEED_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("roomId", wire::Schema::Str16),
    ("roomName", wire::Schema::Str16),
    ("at", wire::Schema::Num),
    ("actorName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("actorAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("quote", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("count", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("threadTitle", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isResolved", wire::Schema::Bool),
    ("canCallBack", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("target", wire::Schema::Dyn),
]));
static S_SEARCH_FEED_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("roomId", wire::Schema::Str16),
    ("roomName", wire::Schema::Str16),
    ("at", wire::Schema::Num),
    ("actorName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("actorAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("quote", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("count", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("threadTitle", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isResolved", wire::Schema::Bool),
    ("canCallBack", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("target", wire::Schema::Dyn),
]));

/// `searchFeed` over the binary wire.
#[napi(js_name = "searchFeedWire")]
pub fn search_feed_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let items: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_SEARCH_FEED_0.inner()))?;
        let query: String = r.string16()?;
        let out = &port::search_feed(items, query)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SEARCH_FEED_OUT.inner())?;
        }
        Ok(())
    })
}

static S_GROUP_BY_DAY_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("roomId", wire::Schema::Str16),
    ("roomName", wire::Schema::Str16),
    ("at", wire::Schema::Num),
    ("actorName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("actorAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("quote", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("count", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("threadTitle", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isResolved", wire::Schema::Bool),
    ("canCallBack", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("target", wire::Schema::Dyn),
]));
static S_GROUP_BY_DAY_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("day", wire::Schema::Dyn),
    (
        "items",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("kind", wire::Schema::Dyn),
            ("roomId", wire::Schema::Str16),
            ("roomName", wire::Schema::Str16),
            ("at", wire::Schema::Num),
            ("actorName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("actorAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("quote", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("count", wire::Schema::Num),
            ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("threadTitle", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isResolved", wire::Schema::Bool),
            ("canCallBack", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("target", wire::Schema::Dyn),
        ])),
    ),
]));

/// `groupByDay` over the binary wire.
#[napi(js_name = "groupByDayWire")]
pub fn group_by_day_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let items: serde_json::Value = wire::read_value(r, &S_GROUP_BY_DAY_0)?;
        let now: f64 = r.f64()?;
        let out = &port::group_by_day(items, now)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_GROUP_BY_DAY_OUT.inner())?;
        }
        Ok(())
    })
}

static S_ASSISTANT_CONSENT_STATUS_0: wire::Schema = wire::Schema::Obj(&[
    (
        "requests",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("assistantId", wire::Schema::Str16),
            ("requesterId", wire::Schema::Str16),
            ("at", wire::Schema::Num),
        ])),
    ),
    (
        "answers",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("assistantId", wire::Schema::Str16),
            ("userId", wire::Schema::Str16),
            ("accepted", wire::Schema::Bool),
            ("at", wire::Schema::Num),
        ])),
    ),
]);
static S_ASSISTANT_CONSENT_STATUS_OUT: wire::Schema = wire::Schema::Dyn;

/// `assistantConsentStatus` over the binary wire.
#[napi(js_name = "assistantConsentStatusWire")]
pub fn assistant_consent_status_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let consent: serde_json::Value = wire::read_value(r, &S_ASSISTANT_CONSENT_STATUS_0)?;
        let assistant_id: String = r.string16()?;
        let member_ids: Vec<String> = r.list(|r| r.string16())?;
        let out = &port::assistant_consent_status(consent, assistant_id, member_ids)?;
        wire::write_value(w, out, &S_ASSISTANT_CONSENT_STATUS_OUT)?;
        Ok(())
    })
}

static S_REQUESTS_WAITING_FOR_0: wire::Schema = wire::Schema::Obj(&[
    (
        "requests",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("assistantId", wire::Schema::Str16),
            ("requesterId", wire::Schema::Str16),
            ("at", wire::Schema::Num),
        ])),
    ),
    (
        "answers",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("assistantId", wire::Schema::Str16),
            ("userId", wire::Schema::Str16),
            ("accepted", wire::Schema::Bool),
            ("at", wire::Schema::Num),
        ])),
    ),
]);
static S_REQUESTS_WAITING_FOR_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("assistantId", wire::Schema::Str16),
    ("requesterId", wire::Schema::Str16),
    ("at", wire::Schema::Num),
]));

/// `requestsWaitingFor` over the binary wire.
#[napi(js_name = "requestsWaitingForWire")]
pub fn requests_waiting_for_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let consent: serde_json::Value = wire::read_value(r, &S_REQUESTS_WAITING_FOR_0)?;
        let user_id: String = r.string16()?;
        let member_ids: Vec<String> = r.list(|r| r.string16())?;
        let out = &port::requests_waiting_for(consent, user_id, member_ids)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_REQUESTS_WAITING_FOR_OUT.inner())?;
        }
        Ok(())
    })
}

static S_PEOPLE_OF_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("isBot", wire::Schema::Bool),
]));
static S_PEOPLE_OF_1: wire::Schema = wire::Schema::Obj(&[
    (
        "requests",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("assistantId", wire::Schema::Str16),
            ("requesterId", wire::Schema::Str16),
            ("at", wire::Schema::Num),
        ])),
    ),
    (
        "answers",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("assistantId", wire::Schema::Str16),
            ("userId", wire::Schema::Str16),
            ("accepted", wire::Schema::Bool),
            ("at", wire::Schema::Num),
        ])),
    ),
]);

/// `peopleOf` over the binary wire.
#[napi(js_name = "peopleOfWire")]
pub fn people_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let members: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_PEOPLE_OF_0.inner()))?;
        let consent: serde_json::Value = wire::read_value(r, &S_PEOPLE_OF_1)?;
        let out = &port::people_of(members, consent)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_PRESENT_REQUEST_0: wire::Schema = wire::Schema::Obj(&[
    (
        "requests",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("assistantId", wire::Schema::Str16),
            ("requesterId", wire::Schema::Str16),
            ("at", wire::Schema::Num),
        ])),
    ),
    (
        "answers",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("assistantId", wire::Schema::Str16),
            ("userId", wire::Schema::Str16),
            ("accepted", wire::Schema::Bool),
            ("at", wire::Schema::Num),
        ])),
    ),
]);
static S_PRESENT_REQUEST_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
]));
static S_PRESENT_REQUEST_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("assistantId", wire::Schema::Str16),
    ("requesterId", wire::Schema::Str16),
    ("at", wire::Schema::Num),
]));

/// `presentRequest` over the binary wire.
#[napi(js_name = "presentRequestWire")]
pub fn present_request_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let consent: serde_json::Value = wire::read_value(r, &S_PRESENT_REQUEST_0)?;
        let members: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_PRESENT_REQUEST_1.inner()))?;
        let out = &port::present_request(consent, members)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_PRESENT_REQUEST_OUT.inner())?;
            }
            None => w.tag(0),
        }
        Ok(())
    })
}

static S_FIND_OPEN_QUESTION_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Dyn),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            ("dimensions", wire::Schema::Dyn),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("voice", wire::Schema::Dyn),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            ("answers", wire::Schema::List(&wire::Schema::Dyn)),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            ("unread", wire::Schema::Dyn),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]));
static S_FIND_OPEN_QUESTION_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("threadId", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("replyCount", wire::Schema::Num),
    (
        "lastReply",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("senderId", wire::Schema::Str16),
            ("senderName", wire::Schema::Str16),
            ("text", wire::Schema::Str16),
            ("sentAt", wire::Schema::Num),
        ])),
    ),
    (
        "unread",
        wire::Schema::Obj(&[
            ("unreadCount", wire::Schema::Num),
            ("mentionCount", wire::Schema::Num),
            ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
        ]),
    ),
    ("isParticipating", wire::Schema::Bool),
    ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
]));
static S_FIND_OPEN_QUESTION_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("messageId", wire::Schema::Str16),
    ("questionId", wire::Schema::Str16),
    ("closesAt", wire::Schema::Num),
    ("text", wire::Schema::Str16),
    ("sentAt", wire::Schema::Num),
    ("title", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "meeting",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("roomId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("withIds", wire::Schema::List(&wire::Schema::Str16)),
            ("startTs", wire::Schema::Num),
            ("endTs", wire::Schema::Num),
        ])),
    ),
]));

/// `findOpenQuestion` over the binary wire.
#[napi(js_name = "findOpenQuestionWire")]
pub fn find_open_question_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let messages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_FIND_OPEN_QUESTION_0.inner()))?;
        let threads: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_FIND_OPEN_QUESTION_1.inner()))?;
        let asker_ids: Vec<String> = r.list(|r| r.string16())?;
        let now: f64 = r.f64()?;
        let out = &port::find_open_question(messages, threads, asker_ids, now)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_FIND_OPEN_QUESTION_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_COMPUTE_WAVEFORM_0: wire::Schema = wire::Schema::Dyn;

/// `computeWaveform` over the binary wire.
#[napi(js_name = "computeWaveformWire")]
pub fn compute_waveform_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let samples: serde_json::Value = wire::read_value(r, &S_COMPUTE_WAVEFORM_0)?;
        let points: Option<serde_json::Value> =
            serde_json::from_value(wire::read_dyn(r)?).map_err(|e| format!("TypeError: {e}"))?;
        let out = &port::compute_waveform(samples, points)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.f64(*x0);
        }
        Ok(())
    })
}

static S_IS_PLAYABLE_AUDIO_0: wire::Schema = wire::Schema::Obj(&[
    ("kind", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("mimeType", wire::Schema::Str16),
    ("size", wire::Schema::OptJs(&wire::Schema::Num)),
    ("isEncrypted", wire::Schema::Bool),
    (
        "dimensions",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("width", wire::Schema::Num),
            ("height", wire::Schema::Num),
        ])),
    ),
    ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
    ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "voice",
        wire::Schema::OptJs(&wire::Schema::Obj(&[(
            "waveform",
            wire::Schema::List(&wire::Schema::Num),
        )])),
    ),
    ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `isPlayableAudio` over the binary wire.
#[napi(js_name = "isPlayableAudioWire")]
pub fn is_playable_audio_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let attachment: serde_json::Value = wire::read_value(r, &S_IS_PLAYABLE_AUDIO_0)?;
        let out = &port::is_playable_audio(attachment)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_VIEWABLE_PDF_0: wire::Schema = wire::Schema::Obj(&[
    ("kind", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("mimeType", wire::Schema::Str16),
    ("size", wire::Schema::OptJs(&wire::Schema::Num)),
    ("isEncrypted", wire::Schema::Bool),
    (
        "dimensions",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("width", wire::Schema::Num),
            ("height", wire::Schema::Num),
        ])),
    ),
    ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
    ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "voice",
        wire::Schema::OptJs(&wire::Schema::Obj(&[(
            "waveform",
            wire::Schema::List(&wire::Schema::Num),
        )])),
    ),
    ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `isViewablePdf` over the binary wire.
#[napi(js_name = "isViewablePdfWire")]
pub fn is_viewable_pdf_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let attachment: serde_json::Value = wire::read_value(r, &S_IS_VIEWABLE_PDF_0)?;
        let can_show_pdf: bool = r.bool()?;
        let out = &port::is_viewable_pdf(attachment, can_show_pdf)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_PREVIEWABLE_0: wire::Schema = wire::Schema::Obj(&[
    ("kind", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("mimeType", wire::Schema::Str16),
    ("size", wire::Schema::OptJs(&wire::Schema::Num)),
    ("isEncrypted", wire::Schema::Bool),
    (
        "dimensions",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("width", wire::Schema::Num),
            ("height", wire::Schema::Num),
        ])),
    ),
    ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
    ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "voice",
        wire::Schema::OptJs(&wire::Schema::Obj(&[(
            "waveform",
            wire::Schema::List(&wire::Schema::Num),
        )])),
    ),
    ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `isPreviewable` over the binary wire.
#[napi(js_name = "isPreviewableWire")]
pub fn is_previewable_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let attachment: serde_json::Value = wire::read_value(r, &S_IS_PREVIEWABLE_0)?;
        let out = &port::is_previewable(attachment)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_GIF_0: wire::Schema = wire::Schema::Obj(&[("mimeType", wire::Schema::Str16)]);

/// `isGif` over the binary wire.
#[napi(js_name = "isGifWire")]
pub fn is_gif_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let attachment: serde_json::Value = wire::read_value(r, &S_IS_GIF_0)?;
        let out = &port::is_gif(attachment)?;
        w.bool(*out);
        Ok(())
    })
}

static S_SVG_CANVAS_SIZE_OUT: wire::Schema =
    wire::Schema::Obj(&[("width", wire::Schema::Num), ("height", wire::Schema::Num)]);

/// `svgCanvasSize` over the binary wire.
#[napi(js_name = "svgCanvasSizeWire")]
pub fn svg_canvas_size_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let width: f64 = r.f64()?;
        let height: f64 = r.f64()?;
        let out = &port::svg_canvas_size(width, height)?;
        wire::write_value(w, out, &S_SVG_CANVAS_SIZE_OUT)?;
        Ok(())
    })
}

static S_THUMBNAIL_SIZE_0: wire::Schema = wire::Schema::Obj(&[
    ("mimeType", wire::Schema::Str16),
    ("size", wire::Schema::Num),
]);
static S_THUMBNAIL_SIZE_1: wire::Schema =
    wire::Schema::Obj(&[("width", wire::Schema::Num), ("height", wire::Schema::Num)]);
static S_THUMBNAIL_SIZE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]));

/// `thumbnailSize` over the binary wire.
#[napi(js_name = "thumbnailSizeWire")]
pub fn thumbnail_size_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let media: serde_json::Value = wire::read_value(r, &S_THUMBNAIL_SIZE_0)?;
        let dimensions: serde_json::Value = wire::read_value(r, &S_THUMBNAIL_SIZE_1)?;
        let out = &port::thumbnail_size(media, dimensions)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_THUMBNAIL_SIZE_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_BOT_OF_0: wire::Schema = wire::Schema::Dyn;
static S_BOT_OF_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("deviceId", wire::Schema::Str16),
    ("masterKey", wire::Schema::Str16),
]));

/// `botOf` over the binary wire.
#[napi(js_name = "botOfWire")]
pub fn bot_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let answer: serde_json::Value = wire::read_value(r, &S_BOT_OF_0)?;
        let out = &port::bot_of(answer)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_BOT_OF_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_FIND_COMMAND_QUERY_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("start", wire::Schema::Num),
    ("query", wire::Schema::Str16),
]));

/// `findCommandQuery` over the binary wire.
#[napi(js_name = "findCommandQueryWire")]
pub fn find_command_query_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let caret: f64 = r.f64()?;
        let out = &port::find_command_query(text, caret)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_FIND_COMMAND_QUERY_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_MATCH_COMMANDS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("botUserId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("syntax", wire::Schema::Str16),
    ("description", wire::Schema::Str16),
    ("argumentNames", wire::Schema::List(&wire::Schema::Str16)),
    ("isLastArgumentVariadic", wire::Schema::Bool),
]));
static S_MATCH_COMMANDS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("botUserId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("syntax", wire::Schema::Str16),
    ("description", wire::Schema::Str16),
    ("argumentNames", wire::Schema::List(&wire::Schema::Str16)),
    ("isLastArgumentVariadic", wire::Schema::Bool),
]));

/// `matchCommands` over the binary wire.
#[napi(js_name = "matchCommandsWire")]
pub fn match_commands_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let commands: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_MATCH_COMMANDS_0.inner()))?;
        let query: String = r.string16()?;
        let out = &port::match_commands(commands, query)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MATCH_COMMANDS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_FIND_LOCAL_COMMAND_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `findLocalCommand` over the binary wire.
#[napi(js_name = "findLocalCommandWire")]
pub fn find_local_command_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let out = &port::find_local_command(text)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_FIND_LOCAL_COMMAND_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_PREPARE_COMMAND_TEXT_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("botUserId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("syntax", wire::Schema::Str16),
    ("description", wire::Schema::Str16),
    ("argumentNames", wire::Schema::List(&wire::Schema::Str16)),
    ("isLastArgumentVariadic", wire::Schema::Bool),
]));
static S_PREPARE_COMMAND_TEXT_OUT: wire::Schema = wire::Schema::Obj(&[
    ("text", wire::Schema::Str16),
    (
        "command",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("botUserId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("syntax", wire::Schema::Str16),
            ("description", wire::Schema::Str16),
            ("argumentNames", wire::Schema::List(&wire::Schema::Str16)),
            ("isLastArgumentVariadic", wire::Schema::Bool),
        ])),
    ),
    ("isEmote", wire::Schema::Bool),
    ("unknownCommand", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `prepareCommandText` over the binary wire.
#[napi(js_name = "prepareCommandTextWire")]
pub fn prepare_command_text_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let commands: serde_json::Value = wire::read_value(r, &S_PREPARE_COMMAND_TEXT_1)?;
        let out = &port::prepare_command_text(text, commands)?;
        wire::write_value(w, out, &S_PREPARE_COMMAND_TEXT_OUT)?;
        Ok(())
    })
}

static S_PREPARE_OUTGOING_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
]));
static S_PREPARE_OUTGOING_2: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("botUserId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("syntax", wire::Schema::Str16),
    ("description", wire::Schema::Str16),
    ("argumentNames", wire::Schema::List(&wire::Schema::Str16)),
    ("isLastArgumentVariadic", wire::Schema::Bool),
]));
static S_PREPARE_OUTGOING_OUT: wire::Schema = wire::Schema::Obj(&[
    ("error", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("text", wire::Schema::Str16),
    (
        "mentions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
        ])),
    ),
    (
        "command",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("botUserId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("syntax", wire::Schema::Str16),
            ("description", wire::Schema::Str16),
            ("argumentNames", wire::Schema::List(&wire::Schema::Str16)),
            ("isLastArgumentVariadic", wire::Schema::Bool),
        ])),
    ),
    ("isEmote", wire::Schema::Bool),
]);

/// `prepareOutgoing` over the binary wire.
#[napi(js_name = "prepareOutgoingWire")]
pub fn prepare_outgoing_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let mentions: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_PREPARE_OUTGOING_1.inner()))?;
        let commands: serde_json::Value = wire::read_value(r, &S_PREPARE_OUTGOING_2)?;
        let out = &port::prepare_outgoing(text, mentions, commands)?;
        wire::write_value(w, out, &S_PREPARE_OUTGOING_OUT)?;
        Ok(())
    })
}

static S_SHOULD_MIRROR_0: wire::Schema = wire::Schema::Dyn;
static S_SHOULD_MIRROR_OUT: wire::Schema = wire::Schema::Bool;

/// `shouldMirror` over the binary wire.
#[napi(js_name = "shouldMirrorWire")]
pub fn should_mirror_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let tile: serde_json::Value = wire::read_value(r, &S_SHOULD_MIRROR_0)?;
        let out = &port::should_mirror(tile)?;
        wire::write_value(w, out, &S_SHOULD_MIRROR_OUT)?;
        Ok(())
    })
}

static S_CURRENT_DEVICE_ID_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
]));

/// `currentDeviceId` over the binary wire.
#[napi(js_name = "currentDeviceIdWire")]
pub fn current_device_id_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let devices: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_CURRENT_DEVICE_ID_0.inner()))?;
        let active_id: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::current_device_id(devices, active_id)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_COUNT_CALL_PEOPLE_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("deviceId", wire::Schema::Str16),
    ("joinedAt", wire::Schema::OptJs(&wire::Schema::Num)),
]));

/// `countCallPeople` over the binary wire.
#[napi(js_name = "countCallPeopleWire")]
pub fn count_call_people_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let members: serde_json::Value = wire::read_value(r, &S_COUNT_CALL_PEOPLE_0)?;
        let out = &port::count_call_people(members)?;
        w.f64(*out);
        Ok(())
    })
}

static S_RING_ASKS_0: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("eventId", wire::Schema::Str16),
    ("until", wire::Schema::Num),
    ("isDeclined", wire::Schema::Bool),
]));
static S_RING_ASKS_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("deviceId", wire::Schema::Str16),
    ("joinedAt", wire::Schema::OptJs(&wire::Schema::Num)),
]));

/// `ringAsks` over the binary wire.
#[napi(js_name = "ringAsksWire")]
pub fn ring_asks_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let ring: Option<serde_json::Value> =
            r.opt_js(|r| wire::read_value(r, S_RING_ASKS_0.inner()))?;
        let members: serde_json::Value = wire::read_value(r, &S_RING_ASKS_1)?;
        let own_user_id: String = r.string16()?;
        let is_direct: bool = r.bool()?;
        let now: f64 = r.f64()?;
        let out = &port::ring_asks(ring, members, own_user_id, is_direct, now)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_RINGING_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("deviceId", wire::Schema::Str16),
    ("joinedAt", wire::Schema::OptJs(&wire::Schema::Num)),
]));

/// `isRinging` over the binary wire.
#[napi(js_name = "isRingingWire")]
pub fn is_ringing_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let members: serde_json::Value = wire::read_value(r, &S_IS_RINGING_0)?;
        let own_user_id: String = r.string16()?;
        let is_direct: bool = r.bool()?;
        let is_joined: bool = r.bool()?;
        let left_at: Option<f64> = r.opt_js(|r| r.f64())?;
        let out = &port::is_ringing(members, own_user_id, is_direct, is_joined, left_at)?;
        w.bool(*out);
        Ok(())
    })
}

static S_FIND_CALL_THREAD_0: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);
static S_FIND_CALL_THREAD_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `findCallThread` over the binary wire.
#[napi(js_name = "findCallThreadWire")]
pub fn find_call_thread_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let messages: serde_json::Value = wire::read_value(r, &S_FIND_CALL_THREAD_0)?;
        let now: f64 = r.f64()?;
        let out = &port::find_call_thread(messages, now)?;
        wire::write_value(w, out, &S_FIND_CALL_THREAD_OUT)?;
        Ok(())
    })
}

static S_TO_CALL_LINE_0: wire::Schema = wire::Schema::Dyn;
static S_TO_CALL_LINE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("direction", wire::Schema::Str16),
    ("isVideo", wire::Schema::Bool),
    ("outcome", wire::Schema::Str16),
    ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
]));

/// `toCallLine` over the binary wire.
#[napi(js_name = "toCallLineWire")]
pub fn to_call_line_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let content: serde_json::Value = wire::read_value(r, &S_TO_CALL_LINE_0)?;
        let is_own: bool = r.bool()?;
        let out = &port::to_call_line(content, is_own)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_TO_CALL_LINE_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_FIT_WINDOW_0: wire::Schema = wire::Schema::Obj(&[
    ("x", wire::Schema::Num),
    ("y", wire::Schema::Num),
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]);
static S_FIT_WINDOW_1: wire::Schema =
    wire::Schema::Obj(&[("width", wire::Schema::Num), ("height", wire::Schema::Num)]);
static S_FIT_WINDOW_OUT: wire::Schema = wire::Schema::Obj(&[
    ("x", wire::Schema::Num),
    ("y", wire::Schema::Num),
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]);

/// `fitWindow` over the binary wire.
#[napi(js_name = "fitWindowWire")]
pub fn fit_window_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let rect: serde_json::Value = wire::read_value(r, &S_FIT_WINDOW_0)?;
        let screen: serde_json::Value = wire::read_value(r, &S_FIT_WINDOW_1)?;
        let out = &port::fit_window(rect, screen)?;
        wire::write_value(w, out, &S_FIT_WINDOW_OUT)?;
        Ok(())
    })
}

static S_DEFAULT_CALL_WINDOW_0: wire::Schema =
    wire::Schema::Obj(&[("width", wire::Schema::Num), ("height", wire::Schema::Num)]);
static S_DEFAULT_CALL_WINDOW_OUT: wire::Schema = wire::Schema::Obj(&[
    ("x", wire::Schema::Num),
    ("y", wire::Schema::Num),
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]);

/// `defaultCallWindow` over the binary wire.
#[napi(js_name = "defaultCallWindowWire")]
pub fn default_call_window_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let screen: serde_json::Value = wire::read_value(r, &S_DEFAULT_CALL_WINDOW_0)?;
        let out = &port::default_call_window(screen)?;
        wire::write_value(w, out, &S_DEFAULT_CALL_WINDOW_OUT)?;
        Ok(())
    })
}

static S_MOVE_WINDOW_0: wire::Schema = wire::Schema::Obj(&[
    ("x", wire::Schema::Num),
    ("y", wire::Schema::Num),
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]);
static S_MOVE_WINDOW_3: wire::Schema =
    wire::Schema::Obj(&[("width", wire::Schema::Num), ("height", wire::Schema::Num)]);
static S_MOVE_WINDOW_OUT: wire::Schema = wire::Schema::Obj(&[
    ("x", wire::Schema::Num),
    ("y", wire::Schema::Num),
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]);

/// `moveWindow` over the binary wire.
#[napi(js_name = "moveWindowWire")]
pub fn move_window_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let rect: serde_json::Value = wire::read_value(r, &S_MOVE_WINDOW_0)?;
        let dx: f64 = r.f64()?;
        let dy: f64 = r.f64()?;
        let screen: serde_json::Value = wire::read_value(r, &S_MOVE_WINDOW_3)?;
        let out = &port::move_window(rect, dx, dy, screen)?;
        wire::write_value(w, out, &S_MOVE_WINDOW_OUT)?;
        Ok(())
    })
}

static S_PLACE_IN_CORNER_0: wire::Schema = wire::Schema::Obj(&[
    ("x", wire::Schema::Num),
    ("y", wire::Schema::Num),
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]);
static S_PLACE_IN_CORNER_2: wire::Schema =
    wire::Schema::Obj(&[("width", wire::Schema::Num), ("height", wire::Schema::Num)]);
static S_PLACE_IN_CORNER_OUT: wire::Schema = wire::Schema::Obj(&[
    ("x", wire::Schema::Num),
    ("y", wire::Schema::Num),
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]);

/// `placeInCorner` over the binary wire.
#[napi(js_name = "placeInCornerWire")]
pub fn place_in_corner_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let rect: serde_json::Value = wire::read_value(r, &S_PLACE_IN_CORNER_0)?;
        let corner: String = r.string16()?;
        let screen: serde_json::Value = wire::read_value(r, &S_PLACE_IN_CORNER_2)?;
        let out = &port::place_in_corner(rect, corner, screen)?;
        wire::write_value(w, out, &S_PLACE_IN_CORNER_OUT)?;
        Ok(())
    })
}

static S_RESIZE_WINDOW_0: wire::Schema = wire::Schema::Obj(&[
    ("x", wire::Schema::Num),
    ("y", wire::Schema::Num),
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]);
static S_RESIZE_WINDOW_4: wire::Schema =
    wire::Schema::Obj(&[("width", wire::Schema::Num), ("height", wire::Schema::Num)]);
static S_RESIZE_WINDOW_OUT: wire::Schema = wire::Schema::Obj(&[
    ("x", wire::Schema::Num),
    ("y", wire::Schema::Num),
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]);

/// `resizeWindow` over the binary wire.
#[napi(js_name = "resizeWindowWire")]
pub fn resize_window_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let rect: serde_json::Value = wire::read_value(r, &S_RESIZE_WINDOW_0)?;
        let edge: String = r.string16()?;
        let dx: f64 = r.f64()?;
        let dy: f64 = r.f64()?;
        let screen: serde_json::Value = wire::read_value(r, &S_RESIZE_WINDOW_4)?;
        let out = &port::resize_window(rect, edge, dx, dy, screen)?;
        wire::write_value(w, out, &S_RESIZE_WINDOW_OUT)?;
        Ok(())
    })
}

static S_PARSE_CALL_WINDOW_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("x", wire::Schema::Num),
    ("y", wire::Schema::Num),
    ("width", wire::Schema::Num),
    ("height", wire::Schema::Num),
]));

/// `parseCallWindow` over the binary wire.
#[napi(js_name = "parseCallWindowWire")]
pub fn parse_call_window_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let stored: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::parse_call_window(stored)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_PARSE_CALL_WINDOW_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_PLACE_CALL_0: wire::Schema = wire::Schema::Obj(&[
    ("detachedRoomId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("shownRoomId", wire::Schema::OptJs(&wire::Schema::Str16)),
]);
static S_PLACE_CALL_1: wire::Schema = wire::Schema::Obj(&[
    ("shownRoomId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("callRoomId", wire::Schema::OptJs(&wire::Schema::Str16)),
]);
static S_PLACE_CALL_OUT: wire::Schema = wire::Schema::Obj(&[
    ("detachedRoomId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("shownRoomId", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `placeCall` over the binary wire.
#[napi(js_name = "placeCallWire")]
pub fn place_call_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let state: serde_json::Value = wire::read_value(r, &S_PLACE_CALL_0)?;
        let now: serde_json::Value = wire::read_value(r, &S_PLACE_CALL_1)?;
        let out = &port::place_call(state, now)?;
        wire::write_value(w, out, &S_PLACE_CALL_OUT)?;
        Ok(())
    })
}

static S_IS_CALL_FLOATING_0: wire::Schema = wire::Schema::Obj(&[
    ("detachedRoomId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("shownRoomId", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `isCallFloating` over the binary wire.
#[napi(js_name = "isCallFloatingWire")]
pub fn is_call_floating_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let state: serde_json::Value = wire::read_value(r, &S_IS_CALL_FLOATING_0)?;
        let call_room_id: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::is_call_floating(state, call_room_id)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PARSE_CLIENT_CONFIG_0: wire::Schema = wire::Schema::Dyn;
static S_PARSE_CLIENT_CONFIG_OUT: wire::Schema = wire::Schema::Obj(&[
    ("applicationName", wire::Schema::Str16),
    ("welcomeMessage", wire::Schema::Str16),
    ("homeserverUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "twakeWorkplaceHomeserverUrl",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("privacyUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("registrationUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("supportUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "qrCodeDownloadUrl",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("renderHtml", wire::Schema::Bool),
    ("hideRedactedEvents", wire::Schema::Bool),
    ("hideUnknownEvents", wire::Schema::Bool),
    ("appGridAvailable", wire::Schema::Bool),
    ("enableLogs", wire::Schema::Bool),
    ("webPushEnabled", wire::Schema::Bool),
    ("vapidPublicKey", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("pushGatewayUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("pushAppId", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `parseClientConfig` over the binary wire.
#[napi(js_name = "parseClientConfigWire")]
pub fn parse_client_config_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let raw: serde_json::Value = wire::read_value(r, &S_PARSE_CLIENT_CONFIG_0)?;
        let out = &port::parse_client_config(raw)?;
        wire::write_value(w, out, &S_PARSE_CLIENT_CONFIG_OUT)?;
        Ok(())
    })
}

static S_READ_COMPOSER_STYLE_0: wire::Schema = wire::Schema::Dyn;
static S_READ_COMPOSER_STYLE_OUT: wire::Schema = wire::Schema::Dyn;

/// `readComposerStyle` over the binary wire.
#[napi(js_name = "readComposerStyleWire")]
pub fn read_composer_style_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let content: serde_json::Value = wire::read_value(r, &S_READ_COMPOSER_STYLE_0)?;
        let out = &port::read_composer_style(content)?;
        wire::write_value(w, out, &S_READ_COMPOSER_STYLE_OUT)?;
        Ok(())
    })
}

static S_PARSE_INVITATION_TARGET_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("contact", wire::Schema::Str16),
    ("medium", wire::Schema::Str16),
]));

/// `parseInvitationTarget` over the binary wire.
#[napi(js_name = "parseInvitationTargetWire")]
pub fn parse_invitation_target_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let input: String = r.string16()?;
        let out = &port::parse_invitation_target(input)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_PARSE_INVITATION_TARGET_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_GROUP_BY_INITIAL_0: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[("displayName", wire::Schema::Str16)]));
static S_GROUP_BY_INITIAL_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("letter", wire::Schema::Str16),
    (
        "people",
        wire::Schema::List(&wire::Schema::Obj(&[("displayName", wire::Schema::Str16)])),
    ),
]));

/// `groupByInitial` over the binary wire.
#[napi(js_name = "groupByInitialWire")]
pub fn group_by_initial_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let people: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_GROUP_BY_INITIAL_0.inner()))?;
        let out = &port::group_by_initial(people)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_GROUP_BY_INITIAL_OUT.inner())?;
        }
        Ok(())
    })
}

static S_PARSE_CONTACTS_VISIBILITY_0: wire::Schema = wire::Schema::Dyn;

/// `parseContactsVisibility` over the binary wire.
#[napi(js_name = "parseContactsVisibilityWire")]
pub fn parse_contacts_visibility_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let value: serde_json::Value = wire::read_value(r, &S_PARSE_CONTACTS_VISIBILITY_0)?;
        let out = &port::parse_contacts_visibility(value)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_EMPTY_CARD_OF_0: wire::Schema = wire::Schema::Obj(&[
    ("isLanding", wire::Schema::Bool),
    ("isWaiting", wire::Schema::Bool),
    ("hasView", wire::Schema::Bool),
    ("isEmpty", wire::Schema::Bool),
    ("room", wire::Schema::Dyn),
    ("isEmbedded", wire::Schema::Bool),
    ("isAssistantRoom", wire::Schema::Bool),
    ("isPeerBlocked", wire::Schema::Bool),
    ("mayPost", wire::Schema::Bool),
]);

/// `emptyCardOf` over the binary wire.
#[napi(js_name = "emptyCardOfWire")]
pub fn empty_card_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let facts: serde_json::Value = wire::read_value(r, &S_EMPTY_CARD_OF_0)?;
        let out = &port::empty_card_of(facts)?;
        wire::write_dyn(w, &serde_json::to_value(out).map_err(|e| e.to_string())?);
        Ok(())
    })
}

static S_FOOTER_OF_0: wire::Schema = wire::Schema::Obj(&[
    ("hasView", wire::Schema::Bool),
    ("isAbsent", wire::Schema::Bool),
    ("isLanding", wire::Schema::Bool),
    ("isPeerGone", wire::Schema::Bool),
    ("isPeerBlocked", wire::Schema::Bool),
    ("peerId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("mayPost", wire::Schema::Bool),
    ("isRecording", wire::Schema::Bool),
    ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `footerOf` over the binary wire.
#[napi(js_name = "footerOfWire")]
pub fn footer_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let facts: serde_json::Value = wire::read_value(r, &S_FOOTER_OF_0)?;
        let out = &port::footer_of(facts)?;
        wire::write_dyn(w, &serde_json::to_value(out).map_err(|e| e.to_string())?);
        Ok(())
    })
}

static S_TIMELINE_HEAD_OF_0: wire::Schema = wire::Schema::Obj(&[
    ("isLanding", wire::Schema::Bool),
    (
        "history",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("hasOlderMessages", wire::Schema::Bool),
            ("deletedBefore", wire::Schema::OptJs(&wire::Schema::Num)),
            ("messageCount", wire::Schema::Num),
        ])),
    ),
    ("predecessor", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isAbsent", wire::Schema::Bool),
    ("isSettled", wire::Schema::Bool),
    ("isOutside", wire::Schema::Bool),
    ("isWaiting", wire::Schema::Bool),
    ("hasTimedOut", wire::Schema::Bool),
]);
static S_TIMELINE_HEAD_OF_OUT: wire::Schema = wire::Schema::Obj(&[
    ("hasOlder", wire::Schema::Bool),
    ("deletedBefore", wire::Schema::OptJs(&wire::Schema::Num)),
    ("predecessor", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isUnavailable", wire::Schema::Bool),
    ("loading", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `timelineHeadOf` over the binary wire.
#[napi(js_name = "timelineHeadOfWire")]
pub fn timeline_head_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let facts: serde_json::Value = wire::read_value(r, &S_TIMELINE_HEAD_OF_0)?;
        let out = &port::timeline_head_of(facts)?;
        wire::write_value(w, out, &S_TIMELINE_HEAD_OF_OUT)?;
        Ok(())
    })
}

static S_HEAD_OVER_WINDOW_0: wire::Schema = wire::Schema::Obj(&[
    ("hasOlder", wire::Schema::Bool),
    ("deletedBefore", wire::Schema::OptJs(&wire::Schema::Num)),
    ("predecessor", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isUnavailable", wire::Schema::Bool),
    ("loading", wire::Schema::OptJs(&wire::Schema::Str16)),
]);
static S_HEAD_OVER_WINDOW_OUT: wire::Schema = wire::Schema::Obj(&[
    ("hasOlder", wire::Schema::Bool),
    ("deletedBefore", wire::Schema::OptJs(&wire::Schema::Num)),
    ("predecessor", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isUnavailable", wire::Schema::Bool),
    ("loading", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `headOverWindow` over the binary wire.
#[napi(js_name = "headOverWindowWire")]
pub fn head_over_window_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let head: serde_json::Value = wire::read_value(r, &S_HEAD_OVER_WINDOW_0)?;
        let hidden_above: f64 = r.f64()?;
        let out = &port::head_over_window(head, hidden_above)?;
        wire::write_value(w, out, &S_HEAD_OVER_WINDOW_OUT)?;
        Ok(())
    })
}

/// `isLandingRoom` over the binary wire.
#[napi(js_name = "isLandingRoomWire")]
pub fn is_landing_room_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: Option<serde_json::Value> =
            serde_json::from_value(wire::read_dyn(r)?).map_err(|e| format!("TypeError: {e}"))?;
        let is_embedded: bool = r.bool()?;
        let out = &port::is_landing_room(room, is_embedded)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MAY_POST_IN_0: wire::Schema = wire::Schema::Obj(&[
    ("hasView", wire::Schema::Bool),
    ("isLanding", wire::Schema::Bool),
    ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "room",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("kind", wire::Schema::Str16),
            ("isSpace", wire::Schema::Bool),
            ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("postPowerLevel", wire::Schema::Num),
            ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
        ])),
    ),
    ("ownPowerLevel", wire::Schema::OptJs(&wire::Schema::Num)),
]);

/// `mayPostIn` over the binary wire.
#[napi(js_name = "mayPostInWire")]
pub fn may_post_in_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let facts: serde_json::Value = wire::read_value(r, &S_MAY_POST_IN_0)?;
        let out = &port::may_post_in(facts)?;
        w.bool(*out);
        Ok(())
    })
}

/// `isDirectRoom` over the binary wire.
#[napi(js_name = "isDirectRoomWire")]
pub fn is_direct_room_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: Option<serde_json::Value> =
            serde_json::from_value(wire::read_dyn(r)?).map_err(|e| format!("TypeError: {e}"))?;
        let direct_user_id: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::is_direct_room(room, direct_user_id)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PEER_STATE_OF_0: wire::Schema = wire::Schema::Obj(&[
    ("isDirect", wire::Schema::Bool),
    ("members", wire::Schema::List(&wire::Schema::Dyn)),
    ("hasMembers", wire::Schema::Bool),
    ("ownUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("ignoredUsers", wire::Schema::List(&wire::Schema::Str16)),
]);
static S_PEER_STATE_OF_OUT: wire::Schema = wire::Schema::Obj(&[
    ("peer", wire::Schema::Dyn),
    ("peerId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isPeerBlocked", wire::Schema::Bool),
    ("isPeerGone", wire::Schema::Bool),
]);

/// `peerStateOf` over the binary wire.
#[napi(js_name = "peerStateOfWire")]
pub fn peer_state_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let facts: serde_json::Value = wire::read_value(r, &S_PEER_STATE_OF_0)?;
        let out = &port::peer_state_of(facts)?;
        wire::write_value(w, out, &S_PEER_STATE_OF_OUT)?;
        Ok(())
    })
}

static S_SPLITS_CALL_CHAT_0: wire::Schema = wire::Schema::Obj(&[
    ("isInCall", wire::Schema::Bool),
    ("isDirect", wire::Schema::Bool),
    ("isVoice", wire::Schema::Bool),
    ("mayPost", wire::Schema::Bool),
]);
static S_SPLITS_CALL_CHAT_OUT: wire::Schema = wire::Schema::Bool;

/// `splitsCallChat` over the binary wire.
#[napi(js_name = "splitsCallChatWire")]
pub fn splits_call_chat_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let facts: serde_json::Value = wire::read_value(r, &S_SPLITS_CALL_CHAT_0)?;
        let out = &port::splits_call_chat(facts)?;
        wire::write_value(w, out, &S_SPLITS_CALL_CHAT_OUT)?;
        Ok(())
    })
}

/// `isEncryptedConversation` over the binary wire.
#[napi(js_name = "isEncryptedConversationWire")]
pub fn is_encrypted_conversation_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: Option<serde_json::Value> =
            serde_json::from_value(wire::read_dyn(r)?).map_err(|e| format!("TypeError: {e}"))?;
        let out = &port::is_encrypted_conversation(room)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_AWAITING_ROOM_0: wire::Schema = wire::Schema::Obj(&[
    ("isOutside", wire::Schema::Bool),
    ("isPending", wire::Schema::Bool),
    ("hasRoom", wire::Schema::Bool),
    ("isSettled", wire::Schema::Bool),
    ("messageCount", wire::Schema::Num),
    ("isSyncReady", wire::Schema::Bool),
    ("isLanding", wire::Schema::Bool),
]);

/// `isAwaitingRoom` over the binary wire.
#[napi(js_name = "isAwaitingRoomWire")]
pub fn is_awaiting_room_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let facts: serde_json::Value = wire::read_value(r, &S_IS_AWAITING_ROOM_0)?;
        let out = &port::is_awaiting_room(facts)?;
        w.bool(*out);
        Ok(())
    })
}

static S_ROOM_NAME_OF_0: wire::Schema = wire::Schema::List(&wire::Schema::OptJs(
    &wire::Schema::Obj(&[("name", wire::Schema::OptJs(&wire::Schema::Str16))]),
));

/// `roomNameOf` over the binary wire.
#[napi(js_name = "roomNameOfWire")]
pub fn room_name_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let sources: Vec<Option<serde_json::Value>> =
            r.list(|r| r.opt_js(|r| wire::read_value(r, S_ROOM_NAME_OF_0.inner().inner())))?;
        let out = &port::room_name_of(sources)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(0),
        }
        Ok(())
    })
}

static S_ASSISTANT_OFFER_IN_0: wire::Schema = wire::Schema::Obj(&[
    ("isEncrypted", wire::Schema::Bool),
    ("myBotId", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "permissions",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("canInvite", wire::Schema::Bool),
            ("canRemove", wire::Schema::Bool),
            ("canBan", wire::Schema::Bool),
            ("canSetLevels", wire::Schema::Bool),
            ("readOnlyLevel", wire::Schema::Num),
            ("defaultLevel", wire::Schema::Num),
            ("canRedact", wire::Schema::Bool),
            ("canEditSettings", wire::Schema::Bool),
            ("canEncrypt", wire::Schema::Bool),
            ("canSetJoinRules", wire::Schema::Bool),
            ("canAddChildren", wire::Schema::Bool),
            ("ownPowerLevel", wire::Schema::Num),
            ("canMentionEveryone", wire::Schema::Bool),
            ("isManagedBySpace", wire::Schema::Bool),
            ("canTransferOwnership", wire::Schema::Bool),
            ("canAddOwner", wire::Schema::Bool),
        ])),
    ),
    (
        "members",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("isBot", wire::Schema::Bool),
            ("isInvited", wire::Schema::Bool),
        ])),
    ),
    (
        "consent",
        wire::Schema::Obj(&[
            (
                "requests",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("assistantId", wire::Schema::Dyn),
                    ("requesterId", wire::Schema::Dyn),
                    ("at", wire::Schema::Dyn),
                ])),
            ),
            (
                "answers",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("assistantId", wire::Schema::Dyn),
                    ("userId", wire::Schema::Dyn),
                    ("accepted", wire::Schema::Dyn),
                    ("at", wire::Schema::Dyn),
                ])),
            ),
        ]),
    ),
]);

/// `assistantOfferIn` over the binary wire.
#[napi(js_name = "assistantOfferInWire")]
pub fn assistant_offer_in_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let facts: serde_json::Value = wire::read_value(r, &S_ASSISTANT_OFFER_IN_0)?;
        let out = &port::assistant_offer_in(facts)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_HAS_ASSISTANT_IN_1: wire::Schema = wire::Schema::Obj(&[
    (
        "requests",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("assistantId", wire::Schema::Str16),
            ("requesterId", wire::Schema::Str16),
            ("at", wire::Schema::Num),
        ])),
    ),
    (
        "answers",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("assistantId", wire::Schema::Str16),
            ("userId", wire::Schema::Str16),
            ("accepted", wire::Schema::Bool),
            ("at", wire::Schema::Num),
        ])),
    ),
]);
static S_HAS_ASSISTANT_IN_2: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
]));

/// `hasAssistantIn` over the binary wire.
#[napi(js_name = "hasAssistantInWire")]
pub fn has_assistant_in_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: Option<serde_json::Value> =
            serde_json::from_value(wire::read_dyn(r)?).map_err(|e| format!("TypeError: {e}"))?;
        let consent: serde_json::Value = wire::read_value(r, &S_HAS_ASSISTANT_IN_1)?;
        let members: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_HAS_ASSISTANT_IN_2.inner()))?;
        let out = &port::has_assistant_in(room, consent, members)?;
        w.bool(*out);
        Ok(())
    })
}

static S_OFFERS_ADD_CONTACT_0: wire::Schema = wire::Schema::Obj(&[
    ("isDirect", wire::Schema::Bool),
    ("peerId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("ownUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isPeerBlocked", wire::Schema::Bool),
    ("isEmbedded", wire::Schema::Bool),
    ("isAssistantRoom", wire::Schema::Bool),
    ("hasView", wire::Schema::Bool),
]);

/// `offersAddContact` over the binary wire.
#[napi(js_name = "offersAddContactWire")]
pub fn offers_add_contact_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let facts: serde_json::Value = wire::read_value(r, &S_OFFERS_ADD_CONTACT_0)?;
        let out = &port::offers_add_contact(facts)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MATCH_PLACE_OF_OUT: wire::Schema = wire::Schema::Obj(&[
    ("index", wire::Schema::Num),
    ("hasOlder", wire::Schema::Bool),
    ("hasNewer", wire::Schema::Bool),
]);

/// `matchPlaceOf` over the binary wire.
#[napi(js_name = "matchPlaceOfWire")]
pub fn match_place_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let message_ids: Vec<String> = r.list(|r| r.string16())?;
        let focus_event_id: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::match_place_of(message_ids, focus_event_id)?;
        wire::write_value(w, out, &S_MATCH_PLACE_OF_OUT)?;
        Ok(())
    })
}

static S_READ_DELETED_CHATS_0: wire::Schema = wire::Schema::Dyn;
static S_READ_DELETED_CHATS_OUT: wire::Schema = wire::Schema::Dyn;

/// `readDeletedChats` over the binary wire.
#[napi(js_name = "readDeletedChatsWire")]
pub fn read_deleted_chats_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let content: serde_json::Value = wire::read_value(r, &S_READ_DELETED_CHATS_0)?;
        let out = &port::read_deleted_chats(content)?;
        wire::write_value(w, out, &S_READ_DELETED_CHATS_OUT)?;
        Ok(())
    })
}

static S_IS_CHAT_HIDDEN_0: wire::Schema = wire::Schema::Dyn;

/// `isChatHidden` over the binary wire.
#[napi(js_name = "isChatHiddenWire")]
pub fn is_chat_hidden_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let deleted: serde_json::Value = wire::read_value(r, &S_IS_CHAT_HIDDEN_0)?;
        let room_id: String = r.string16()?;
        let last_activity_at: f64 = r.f64()?;
        let out = &port::is_chat_hidden(deleted, room_id, last_activity_at)?;
        w.bool(*out);
        Ok(())
    })
}

static S_WITHOUT_DELETED_0: wire::Schema = wire::Schema::Dyn;
static S_WITHOUT_DELETED_2: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Dyn),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            ("dimensions", wire::Schema::Dyn),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("voice", wire::Schema::Dyn),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            ("answers", wire::Schema::List(&wire::Schema::Dyn)),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            ("unread", wire::Schema::Dyn),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]));
static S_WITHOUT_DELETED_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Dyn),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            ("dimensions", wire::Schema::Dyn),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("voice", wire::Schema::Dyn),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            ("answers", wire::Schema::List(&wire::Schema::Dyn)),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            ("unread", wire::Schema::Dyn),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]));

/// `withoutDeleted` over the binary wire.
#[napi(js_name = "withoutDeletedWire")]
pub fn without_deleted_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let deleted: serde_json::Value = wire::read_value(r, &S_WITHOUT_DELETED_0)?;
        let room_id: String = r.string16()?;
        let messages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_WITHOUT_DELETED_2.inner()))?;
        let out = &port::without_deleted(deleted, room_id, messages)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_WITHOUT_DELETED_OUT.inner())?;
        }
        Ok(())
    })
}

static S_IS_AFTER_DELETION_0: wire::Schema = wire::Schema::Dyn;

/// `isAfterDeletion` over the binary wire.
#[napi(js_name = "isAfterDeletionWire")]
pub fn is_after_deletion_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let deleted: serde_json::Value = wire::read_value(r, &S_IS_AFTER_DELETION_0)?;
        let room_id: String = r.string16()?;
        let at: f64 = r.f64()?;
        let out = &port::is_after_deletion(deleted, room_id, at)?;
        w.bool(*out);
        Ok(())
    })
}

static S_READ_DISMISSED_QUESTIONS_0: wire::Schema = wire::Schema::Dyn;
static S_READ_DISMISSED_QUESTIONS_OUT: wire::Schema = wire::Schema::Dyn;

/// `readDismissedQuestions` over the binary wire.
#[napi(js_name = "readDismissedQuestionsWire")]
pub fn read_dismissed_questions_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let content: serde_json::Value = wire::read_value(r, &S_READ_DISMISSED_QUESTIONS_0)?;
        let out = &port::read_dismissed_questions(content)?;
        wire::write_value(w, out, &S_READ_DISMISSED_QUESTIONS_OUT)?;
        Ok(())
    })
}

static S_WITH_DISMISSED_QUESTION_0: wire::Schema = wire::Schema::Dyn;
static S_WITH_DISMISSED_QUESTION_OUT: wire::Schema = wire::Schema::Dyn;

/// `withDismissedQuestion` over the binary wire.
#[napi(js_name = "withDismissedQuestionWire")]
pub fn with_dismissed_question_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let dismissed: serde_json::Value = wire::read_value(r, &S_WITH_DISMISSED_QUESTION_0)?;
        let id: String = r.string16()?;
        let closes_at: f64 = r.f64()?;
        let now: f64 = r.f64()?;
        let out = &port::with_dismissed_question(dismissed, id, closes_at, now)?;
        wire::write_value(w, out, &S_WITH_DISMISSED_QUESTION_OUT)?;
        Ok(())
    })
}

static S_FIND_EMOJI_QUERY_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("start", wire::Schema::Num),
    ("query", wire::Schema::Str16),
]));

/// `findEmojiQuery` over the binary wire.
#[napi(js_name = "findEmojiQueryWire")]
pub fn find_emoji_query_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let caret: f64 = r.f64()?;
        let out = &port::find_emoji_query(text, caret)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_FIND_EMOJI_QUERY_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

/// `isEmojiDrawable` over the binary wire.
#[napi(js_name = "isEmojiDrawableWire")]
pub fn is_emoji_drawable_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let emoji: String = r.string16()?;
        let max: Option<serde_json::Value> =
            serde_json::from_value(wire::read_dyn(r)?).map_err(|e| format!("TypeError: {e}"))?;
        let flags: Option<bool> = r.opt_js(|r| r.bool())?;
        let out = &port::is_emoji_drawable(emoji, max, flags)?;
        w.bool(*out);
        Ok(())
    })
}

static S_HAS_INK_0: wire::Schema = wire::Schema::Dyn;

/// `hasInk` over the binary wire.
#[napi(js_name = "hasInkWire")]
pub fn has_ink_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let pixels: serde_json::Value = wire::read_value(r, &S_HAS_INK_0)?;
        let out = &port::has_ink(pixels)?;
        w.bool(*out);
        Ok(())
    })
}

static S_HAS_COLOUR_0: wire::Schema = wire::Schema::Dyn;

/// `hasColour` over the binary wire.
#[napi(js_name = "hasColourWire")]
pub fn has_colour_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let pixels: serde_json::Value = wire::read_value(r, &S_HAS_COLOUR_0)?;
        let out = &port::has_colour(pixels)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_DRAWN_0: wire::Schema = wire::Schema::Dyn;
static S_IS_DRAWN_1: wire::Schema = wire::Schema::Dyn;

/// `isDrawn` over the binary wire.
#[napi(js_name = "isDrawnWire")]
pub fn is_drawn_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let sample: serde_json::Value = wire::read_value(r, &S_IS_DRAWN_0)?;
        let blank: serde_json::Value = wire::read_value(r, &S_IS_DRAWN_1)?;
        let out = &port::is_drawn(sample, blank)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_NOTIFICATION_SERVICE_0: wire::Schema = wire::Schema::Dyn;

/// `isNotificationService` over the binary wire.
#[napi(js_name = "isNotificationServiceWire")]
pub fn is_notification_service_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let device: serde_json::Value = wire::read_value(r, &S_IS_NOTIFICATION_SERVICE_0)?;
        let out = &port::is_notification_service(device)?;
        w.bool(*out);
        Ok(())
    })
}

static S_READ_FOLDERS_0: wire::Schema = wire::Schema::Dyn;
static S_READ_FOLDERS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hidden", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("color", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("include", wire::Schema::List(&wire::Schema::Str16)),
    ("exclude", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `readFolders` over the binary wire.
#[napi(js_name = "readFoldersWire")]
pub fn read_folders_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let content: serde_json::Value = wire::read_value(r, &S_READ_FOLDERS_0)?;
        let out = &port::read_folders(content)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_READ_FOLDERS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_WRITE_FOLDERS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hidden", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("color", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("include", wire::Schema::List(&wire::Schema::Str16)),
    ("exclude", wire::Schema::List(&wire::Schema::Str16)),
]));
static S_WRITE_FOLDERS_OUT: wire::Schema =
    wire::Schema::Obj(&[("folders", wire::Schema::List(&wire::Schema::Dyn))]);

/// `writeFolders` over the binary wire.
#[napi(js_name = "writeFoldersWire")]
pub fn write_folders_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let folders: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_WRITE_FOLDERS_0.inner()))?;
        let out = &port::write_folders(folders)?;
        wire::write_value(w, out, &S_WRITE_FOLDERS_OUT)?;
        Ok(())
    })
}

static S_WITH_HIDDEN_BUILTINS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hidden", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("color", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("include", wire::Schema::List(&wire::Schema::Str16)),
    ("exclude", wire::Schema::List(&wire::Schema::Str16)),
]));
static S_WITH_HIDDEN_BUILTINS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hidden", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("color", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("include", wire::Schema::List(&wire::Schema::Str16)),
    ("exclude", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `withHiddenBuiltins` over the binary wire.
#[napi(js_name = "withHiddenBuiltinsWire")]
pub fn with_hidden_builtins_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let folders: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_WITH_HIDDEN_BUILTINS_0.inner()))?;
        let out = &port::with_hidden_builtins(folders)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_WITH_HIDDEN_BUILTINS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_LIST_FOLDERS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hidden", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("color", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("include", wire::Schema::List(&wire::Schema::Str16)),
    ("exclude", wire::Schema::List(&wire::Schema::Str16)),
]));
static S_LIST_FOLDERS_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("room", wire::Schema::Dyn),
    (
        "folders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
]));
static S_LIST_FOLDERS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hidden", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("color", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("include", wire::Schema::List(&wire::Schema::Str16)),
    ("exclude", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `listFolders` over the binary wire.
#[napi(js_name = "listFoldersWire")]
pub fn list_folders_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let saved: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_LIST_FOLDERS_0.inner()))?;
        let entries: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_LIST_FOLDERS_1.inner()))?;
        let out = &port::list_folders(saved, entries)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_LIST_FOLDERS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_FOLDER_CHAT_TYPE_OF_0: wire::Schema = wire::Schema::Dyn;

/// `folderChatTypeOf` over the binary wire.
#[napi(js_name = "folderChatTypeOfWire")]
pub fn folder_chat_type_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_FOLDER_CHAT_TYPE_OF_0)?;
        let out = &port::folder_chat_type_of(room)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_IS_IN_FOLDER_0: wire::Schema = wire::Schema::Obj(&[
    ("room", wire::Schema::Dyn),
    (
        "folders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
]);
static S_IS_IN_FOLDER_1: wire::Schema = wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hidden", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("color", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("include", wire::Schema::List(&wire::Schema::Str16)),
    ("exclude", wire::Schema::List(&wire::Schema::Str16)),
]);

/// `isInFolder` over the binary wire.
#[napi(js_name = "isInFolderWire")]
pub fn is_in_folder_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let entry: serde_json::Value = wire::read_value(r, &S_IS_IN_FOLDER_0)?;
        let folder: serde_json::Value = wire::read_value(r, &S_IS_IN_FOLDER_1)?;
        let out = &port::is_in_folder(entry, folder)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_RECOMMENDATION_TAKEN_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hidden", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("color", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("include", wire::Schema::List(&wire::Schema::Str16)),
    ("exclude", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `isRecommendationTaken` over the binary wire.
#[napi(js_name = "isRecommendationTakenWire")]
pub fn is_recommendation_taken_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let r#type: String = r.string16()?;
        let folders: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_IS_RECOMMENDATION_TAKEN_1.inner()))?;
        let out = &port::is_recommendation_taken(r#type, folders)?;
        w.bool(*out);
        Ok(())
    })
}

static S_FIT_PREVIEW_0: wire::Schema = wire::Schema::Dyn;
static S_FIT_PREVIEW_OUT: wire::Schema =
    wire::Schema::Obj(&[("width", wire::Schema::Num), ("height", wire::Schema::Num)]);

/// `fitPreview` over the binary wire.
#[napi(js_name = "fitPreviewWire")]
pub fn fit_preview_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let dimensions: serde_json::Value = wire::read_value(r, &S_FIT_PREVIEW_0)?;
        let max_side: f64 = r.f64()?;
        let empty_ratio: Option<f64> = r.opt_js(|r| r.f64())?;
        let out = &port::fit_preview(dimensions, max_side, empty_ratio)?;
        wire::write_value(w, out, &S_FIT_PREVIEW_OUT)?;
        Ok(())
    })
}

static S_IS_RECORD_0: wire::Schema = wire::Schema::Dyn;

/// `isRecord` over the binary wire.
#[napi(js_name = "isRecordWire")]
pub fn is_record_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let value: serde_json::Value = wire::read_value(r, &S_IS_RECORD_0)?;
        let out = &port::is_record(value)?;
        wire::write_dyn(w, &serde_json::to_value(out).map_err(|e| e.to_string())?);
        Ok(())
    })
}

static S_READ_TEXT_0: wire::Schema = wire::Schema::Map16(&wire::Schema::Dyn);

/// `readText` over the binary wire.
#[napi(js_name = "readTextWire")]
pub fn read_text_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let source: serde_json::Value = wire::read_value(r, &S_READ_TEXT_0)?;
        let key: String = r.string16()?;
        let out = &port::read_text(source, key)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_READ_TEXTS_0: wire::Schema = wire::Schema::Map16(&wire::Schema::Dyn);

/// `readTexts` over the binary wire.
#[napi(js_name = "readTextsWire")]
pub fn read_texts_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let source: serde_json::Value = wire::read_value(r, &S_READ_TEXTS_0)?;
        let key: String = r.string16()?;
        let out = &port::read_texts(source, key)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_READ_BOOLEAN_0: wire::Schema = wire::Schema::Map16(&wire::Schema::Dyn);

/// `readBoolean` over the binary wire.
#[napi(js_name = "readBooleanWire")]
pub fn read_boolean_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let source: serde_json::Value = wire::read_value(r, &S_READ_BOOLEAN_0)?;
        let key: String = r.string16()?;
        let fallback: bool = r.bool()?;
        let out = &port::read_boolean(source, key, fallback)?;
        w.bool(*out);
        Ok(())
    })
}

static S_AUTO_TRANSLATION_OF_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            (
                "dimensions",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("width", wire::Schema::Dyn),
                    ("height", wire::Schema::Dyn),
                ])),
            ),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "voice",
                wire::Schema::OptJs(&wire::Schema::Obj(&[("waveform", wire::Schema::Dyn)])),
            ),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            (
                "answers",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("id", wire::Schema::Dyn),
                    ("text", wire::Schema::Dyn),
                ])),
            ),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            (
                "unread",
                wire::Schema::Obj(&[
                    ("unreadCount", wire::Schema::Dyn),
                    ("mentionCount", wire::Schema::Dyn),
                    ("isMarkedUnread", wire::Schema::Dyn),
                ]),
            ),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]);

/// `autoTranslationOf` over the binary wire.
#[napi(js_name = "autoTranslationOfWire")]
pub fn auto_translation_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let message: serde_json::Value = wire::read_value(r, &S_AUTO_TRANSLATION_OF_0)?;
        let prompt: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::auto_translation_of(message, prompt)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(0),
        }
        Ok(())
    })
}

static S_PARSE_GEO_URI_0: wire::Schema = wire::Schema::Dyn;
static S_PARSE_GEO_URI_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("lat", wire::Schema::Num),
    ("lon", wire::Schema::Num),
]));

/// `parseGeoUri` over the binary wire.
#[napi(js_name = "parseGeoUriWire")]
pub fn parse_geo_uri_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let uri: serde_json::Value = wire::read_value(r, &S_PARSE_GEO_URI_0)?;
        let out = &port::parse_geo_uri(uri)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_PARSE_GEO_URI_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_OPEN_STREET_MAP_URL_0: wire::Schema =
    wire::Schema::Obj(&[("lat", wire::Schema::Num), ("lon", wire::Schema::Num)]);

/// `openStreetMapUrl` over the binary wire.
#[napi(js_name = "openStreetMapUrlWire")]
pub fn open_street_map_url_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let arg0: serde_json::Value = wire::read_value(r, &S_OPEN_STREET_MAP_URL_0)?;
        let out = &port::open_street_map_url(arg0)?;
        w.string16(out);
        Ok(())
    })
}

static S_PICK_ATTENDEES_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("isBot", wire::Schema::Bool),
]));
static S_PICK_ATTENDEES_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("displayName", wire::Schema::Str16),
    ("firstName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("lastName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("emails", wire::Schema::List(&wire::Schema::Str16)),
    ("phones", wire::Schema::List(&wire::Schema::Str16)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("contactId", wire::Schema::OptJs(&wire::Schema::Str16)),
]));
static S_PICK_ATTENDEES_OUT: wire::Schema = wire::Schema::Obj(&[
    ("emails", wire::Schema::List(&wire::Schema::Str16)),
    ("missingCount", wire::Schema::Num),
]);

/// `pickAttendees` over the binary wire.
#[napi(js_name = "pickAttendeesWire")]
pub fn pick_attendees_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let members: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_PICK_ATTENDEES_0.inner()))?;
        let address_book: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_PICK_ATTENDEES_1.inner()))?;
        let own_user_id: String = r.string16()?;
        let out = &port::pick_attendees(members, address_book, own_user_id)?;
        wire::write_value(w, out, &S_PICK_ATTENDEES_OUT)?;
        Ok(())
    })
}

static S_READ_CREATED_MEETING_0: wire::Schema = wire::Schema::Dyn;
static S_READ_CREATED_MEETING_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("uid", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("start", wire::Schema::Str16),
    ("end", wire::Schema::Str16),
    ("allDay", wire::Schema::Bool),
    ("timeZone", wire::Schema::Str16),
    (
        "videoConferenceUrl",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("url", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `readCreatedMeeting` over the binary wire.
#[napi(js_name = "readCreatedMeetingWire")]
pub fn read_created_meeting_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let value: serde_json::Value = wire::read_value(r, &S_READ_CREATED_MEETING_0)?;
        let out = &port::read_created_meeting(value)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_READ_CREATED_MEETING_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_MEETING_MESSAGE_TEXT_0: wire::Schema = wire::Schema::Obj(&[
    ("uid", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("start", wire::Schema::Str16),
    ("end", wire::Schema::Str16),
    ("allDay", wire::Schema::Bool),
    ("timeZone", wire::Schema::Str16),
    (
        "videoConferenceUrl",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("url", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `meetingMessageText` over the binary wire.
#[napi(js_name = "meetingMessageTextWire")]
pub fn meeting_message_text_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let meeting: serde_json::Value = wire::read_value(r, &S_MEETING_MESSAGE_TEXT_0)?;
        let lang: String = r.string16()?;
        let sender_zone: String = r.string16()?;
        let out = &port::meeting_message_text(meeting, lang, sender_zone)?;
        w.string16(out);
        Ok(())
    })
}

/// `describePeople` over the binary wire.
#[napi(js_name = "describePeopleWire")]
pub fn describe_people_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let names: Vec<String> = r.list(|r| r.string16())?;
        let total: Option<serde_json::Value> =
            serde_json::from_value(wire::read_dyn(r)?).map_err(|e| format!("TypeError: {e}"))?;
        let out = &port::describe_people(names, total)?;
        w.string16(out);
        Ok(())
    })
}

static S_MATCH_MEMBERS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
]));
static S_MATCH_MEMBERS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `matchMembers` over the binary wire.
#[napi(js_name = "matchMembersWire")]
pub fn match_members_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let members: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_MATCH_MEMBERS_0.inner()))?;
        let query: String = r.string16()?;
        let own_user_id: String = r.string16()?;
        let limit: Option<f64> = r.opt_js(|r| r.f64())?;
        let out = &port::match_members(members, query, own_user_id, limit)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MATCH_MEMBERS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_MENTION_TEXT_0: wire::Schema = wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `mentionText` over the binary wire.
#[napi(js_name = "mentionTextWire")]
pub fn mention_text_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let member: serde_json::Value = wire::read_value(r, &S_MENTION_TEXT_0)?;
        let out = &port::mention_text(member)?;
        w.string16(out);
        Ok(())
    })
}

static S_KEEP_PRESENT_MENTIONS_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
]));
static S_KEEP_PRESENT_MENTIONS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
]));

/// `keepPresentMentions` over the binary wire.
#[napi(js_name = "keepPresentMentionsWire")]
pub fn keep_present_mentions_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let mentions: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_KEEP_PRESENT_MENTIONS_1.inner()))?;
        let out = &port::keep_present_mentions(text, mentions)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_KEEP_PRESENT_MENTIONS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_MATCH_ROOMS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
]));
static S_MATCH_ROOMS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `matchRooms` over the binary wire.
#[napi(js_name = "matchRoomsWire")]
pub fn match_rooms_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let rooms: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_MATCH_ROOMS_0.inner()))?;
        let query: String = r.string16()?;
        let limit: Option<f64> = r.opt_js(|r| r.f64())?;
        let out = &port::match_rooms(rooms, query, limit)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MATCH_ROOMS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_ROOM_MENTION_REF_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `roomMentionRef` over the binary wire.
#[napi(js_name = "roomMentionRefWire")]
pub fn room_mention_ref_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_ROOM_MENTION_REF_0)?;
        let out = &port::room_mention_ref(room)?;
        w.string16(out);
        Ok(())
    })
}

static S_ROOM_MENTION_TEXT_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `roomMentionText` over the binary wire.
#[napi(js_name = "roomMentionTextWire")]
pub fn room_mention_text_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_ROOM_MENTION_TEXT_0)?;
        let out = &port::room_mention_text(room)?;
        w.string16(out);
        Ok(())
    })
}

static S_IS_ROOM_MENTION_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
]);

/// `isRoomMention` over the binary wire.
#[napi(js_name = "isRoomMentionWire")]
pub fn is_room_mention_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let mention: serde_json::Value = wire::read_value(r, &S_IS_ROOM_MENTION_0)?;
        let out = &port::is_room_mention(mention)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_EVERYONE_MENTION_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
]);

/// `isEveryoneMention` over the binary wire.
#[napi(js_name = "isEveryoneMentionWire")]
pub fn is_everyone_mention_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let mention: serde_json::Value = wire::read_value(r, &S_IS_EVERYONE_MENTION_0)?;
        let out = &port::is_everyone_mention(mention)?;
        w.bool(*out);
        Ok(())
    })
}

static S_COUNTS_AS_UNREAD_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            (
                "dimensions",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("width", wire::Schema::Dyn),
                    ("height", wire::Schema::Dyn),
                ])),
            ),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "voice",
                wire::Schema::OptJs(&wire::Schema::Obj(&[("waveform", wire::Schema::Dyn)])),
            ),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            (
                "answers",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("id", wire::Schema::Dyn),
                    ("text", wire::Schema::Dyn),
                ])),
            ),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            (
                "unread",
                wire::Schema::Obj(&[
                    ("unreadCount", wire::Schema::Dyn),
                    ("mentionCount", wire::Schema::Dyn),
                    ("isMarkedUnread", wire::Schema::Dyn),
                ]),
            ),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]);

/// `countsAsUnread` over the binary wire.
#[napi(js_name = "countsAsUnreadWire")]
pub fn counts_as_unread_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let message: serde_json::Value = wire::read_value(r, &S_COUNTS_AS_UNREAD_0)?;
        let out = &port::counts_as_unread(message)?;
        w.bool(*out);
        Ok(())
    })
}

static S_FIND_FIRST_UNREAD_ID_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Dyn),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            ("dimensions", wire::Schema::Dyn),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("voice", wire::Schema::Dyn),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            ("answers", wire::Schema::List(&wire::Schema::Dyn)),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            ("unread", wire::Schema::Dyn),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]));

/// `findFirstUnreadId` over the binary wire.
#[napi(js_name = "findFirstUnreadIdWire")]
pub fn find_first_unread_id_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let messages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_FIND_FIRST_UNREAD_ID_0.inner()))?;
        let unread_count: f64 = r.f64()?;
        let out = &port::find_first_unread_id(messages, unread_count)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_IS_SELECTABLE_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            (
                "dimensions",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("width", wire::Schema::Dyn),
                    ("height", wire::Schema::Dyn),
                ])),
            ),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "voice",
                wire::Schema::OptJs(&wire::Schema::Obj(&[("waveform", wire::Schema::Dyn)])),
            ),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            (
                "answers",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("id", wire::Schema::Dyn),
                    ("text", wire::Schema::Dyn),
                ])),
            ),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            (
                "unread",
                wire::Schema::Obj(&[
                    ("unreadCount", wire::Schema::Dyn),
                    ("mentionCount", wire::Schema::Dyn),
                    ("isMarkedUnread", wire::Schema::Dyn),
                ]),
            ),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]);

/// `isSelectable` over the binary wire.
#[napi(js_name = "isSelectableWire")]
pub fn is_selectable_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let message: serde_json::Value = wire::read_value(r, &S_IS_SELECTABLE_0)?;
        let out = &port::is_selectable(message)?;
        w.bool(*out);
        Ok(())
    })
}

static S_STARTS_GROUP_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Dyn),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            ("dimensions", wire::Schema::Dyn),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("voice", wire::Schema::Dyn),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            ("answers", wire::Schema::List(&wire::Schema::Dyn)),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            ("unread", wire::Schema::Dyn),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]));

/// `startsGroup` over the binary wire.
#[napi(js_name = "startsGroupWire")]
pub fn starts_group_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let messages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_STARTS_GROUP_0.inner()))?;
        let index: f64 = r.f64()?;
        let out = &port::starts_group(messages, index)?;
        w.bool(*out);
        Ok(())
    })
}

static S_GROUP_ALBUMS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Dyn),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            ("dimensions", wire::Schema::Dyn),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("voice", wire::Schema::Dyn),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            ("answers", wire::Schema::List(&wire::Schema::Dyn)),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            ("unread", wire::Schema::Dyn),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]));
static S_GROUP_ALBUMS_OUT: wire::Schema =
    wire::Schema::List(&wire::Schema::List(&wire::Schema::Obj(&[
        ("id", wire::Schema::Str16),
        ("senderId", wire::Schema::Str16),
        ("senderName", wire::Schema::Str16),
        ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("isFromBot", wire::Schema::Bool),
        ("text", wire::Schema::Str16),
        ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
        (
            "attachment",
            wire::Schema::OptJs(&wire::Schema::Obj(&[
                ("kind", wire::Schema::Dyn),
                ("name", wire::Schema::Dyn),
                ("mimeType", wire::Schema::Dyn),
                ("size", wire::Schema::Dyn),
                ("isEncrypted", wire::Schema::Dyn),
                ("dimensions", wire::Schema::Dyn),
                ("duration", wire::Schema::Dyn),
                ("hasThumbnail", wire::Schema::Dyn),
                ("voice", wire::Schema::Dyn),
                ("batch", wire::Schema::Dyn),
            ])),
        ),
        (
            "location",
            wire::Schema::OptJs(&wire::Schema::Obj(&[
                ("lat", wire::Schema::Num),
                ("lon", wire::Schema::Num),
            ])),
        ),
        (
            "poll",
            wire::Schema::OptJs(&wire::Schema::Obj(&[
                ("question", wire::Schema::Str16),
                ("answers", wire::Schema::List(&wire::Schema::Dyn)),
                ("maxSelections", wire::Schema::Num),
                ("isDisclosed", wire::Schema::Bool),
                ("isAnonymous", wire::Schema::Bool),
                ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
                ("votes", wire::Schema::Dyn),
                ("voters", wire::Schema::Dyn),
                ("totalVotes", wire::Schema::Num),
                ("ownAnswerIds", wire::Schema::List(&wire::Schema::Dyn)),
                ("canVote", wire::Schema::Bool),
                ("canEnd", wire::Schema::Bool),
                ("canEdit", wire::Schema::Bool),
            ])),
        ),
        ("question", wire::Schema::Dyn),
        ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
        (
            "call",
            wire::Schema::OptJs(&wire::Schema::Obj(&[
                ("direction", wire::Schema::Str16),
                ("isVideo", wire::Schema::Bool),
                ("outcome", wire::Schema::Str16),
                ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
            ])),
        ),
        ("sentAt", wire::Schema::Num),
        ("isOwn", wire::Schema::Bool),
        ("isReadByOthers", wire::Schema::Bool),
        (
            "lastReadBy",
            wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
                ("userId", wire::Schema::Dyn),
                ("name", wire::Schema::Dyn),
                ("avatarUrl", wire::Schema::Dyn),
                ("readAt", wire::Schema::Dyn),
            ]))),
        ),
        (
            "thread",
            wire::Schema::OptJs(&wire::Schema::Obj(&[
                ("threadId", wire::Schema::Dyn),
                ("title", wire::Schema::Dyn),
                ("replyCount", wire::Schema::Dyn),
                ("lastReply", wire::Schema::Dyn),
                ("unread", wire::Schema::Dyn),
                ("isParticipating", wire::Schema::Dyn),
                ("isMuted", wire::Schema::Dyn),
                ("isResolved", wire::Schema::Dyn),
                ("lastOwnReplyAt", wire::Schema::Dyn),
            ])),
        ),
        (
            "replyTo",
            wire::Schema::OptJs(&wire::Schema::Obj(&[
                ("messageId", wire::Schema::Dyn),
                ("senderName", wire::Schema::Dyn),
                ("senderId", wire::Schema::Dyn),
                ("text", wire::Schema::Dyn),
                ("isDeleted", wire::Schema::Dyn),
            ])),
        ),
        ("isEdited", wire::Schema::Bool),
        (
            "reactions",
            wire::Schema::List(&wire::Schema::Obj(&[
                ("key", wire::Schema::Dyn),
                ("count", wire::Schema::Dyn),
                ("ownReactionId", wire::Schema::Dyn),
                ("senders", wire::Schema::Dyn),
            ])),
        ),
        ("sendStatus", wire::Schema::Str16),
    ])));

/// `groupAlbums` over the binary wire.
#[napi(js_name = "groupAlbumsWire")]
pub fn group_albums_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let messages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_GROUP_ALBUMS_0.inner()))?;
        let break_before: Option<String> = r.opt_js(|r| r.string16())?;
        let hides_gifs: Option<bool> = r.opt_js(|r| r.bool())?;
        let out = &port::group_albums(messages, break_before, hides_gifs)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.tag(1);
            w.count(x0.len());
            for x1 in x0.iter() {
                wire::write_value(w, x1, S_GROUP_ALBUMS_OUT.inner().inner())?;
            }
        }
        Ok(())
    })
}

static S_STARTS_DAY_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Dyn),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            ("dimensions", wire::Schema::Dyn),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("voice", wire::Schema::Dyn),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            ("answers", wire::Schema::List(&wire::Schema::Dyn)),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            ("unread", wire::Schema::Dyn),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]));

/// `startsDay` over the binary wire.
#[napi(js_name = "startsDayWire")]
pub fn starts_day_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let messages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_STARTS_DAY_0.inner()))?;
        let index: f64 = r.f64()?;
        let out = &port::starts_day(messages, index)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_TRANSLATABLE_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            (
                "dimensions",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("width", wire::Schema::Dyn),
                    ("height", wire::Schema::Dyn),
                ])),
            ),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "voice",
                wire::Schema::OptJs(&wire::Schema::Obj(&[("waveform", wire::Schema::Dyn)])),
            ),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            (
                "answers",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("id", wire::Schema::Dyn),
                    ("text", wire::Schema::Dyn),
                ])),
            ),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            (
                "unread",
                wire::Schema::Obj(&[
                    ("unreadCount", wire::Schema::Dyn),
                    ("mentionCount", wire::Schema::Dyn),
                    ("isMarkedUnread", wire::Schema::Dyn),
                ]),
            ),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]);

/// `isTranslatable` over the binary wire.
#[napi(js_name = "isTranslatableWire")]
pub fn is_translatable_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let message: serde_json::Value = wire::read_value(r, &S_IS_TRANSLATABLE_0)?;
        let out = &port::is_translatable(message)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_EDITABLE_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            (
                "dimensions",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("width", wire::Schema::Dyn),
                    ("height", wire::Schema::Dyn),
                ])),
            ),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "voice",
                wire::Schema::OptJs(&wire::Schema::Obj(&[("waveform", wire::Schema::Dyn)])),
            ),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            (
                "answers",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("id", wire::Schema::Dyn),
                    ("text", wire::Schema::Dyn),
                ])),
            ),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            (
                "unread",
                wire::Schema::Obj(&[
                    ("unreadCount", wire::Schema::Dyn),
                    ("mentionCount", wire::Schema::Dyn),
                    ("isMarkedUnread", wire::Schema::Dyn),
                ]),
            ),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]);

/// `isEditable` over the binary wire.
#[napi(js_name = "isEditableWire")]
pub fn is_editable_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let message: serde_json::Value = wire::read_value(r, &S_IS_EDITABLE_0)?;
        let out = &port::is_editable(message)?;
        w.bool(*out);
        Ok(())
    })
}

static S_SHOULD_NOTIFY_0: wire::Schema = wire::Schema::Obj(&[
    ("isOwn", wire::Schema::Bool),
    ("isDirect", wire::Schema::Bool),
    ("isMention", wire::Schema::Bool),
]);
static S_SHOULD_NOTIFY_2: wire::Schema = wire::Schema::Obj(&[
    ("directMessages", wire::Schema::Str16),
    ("groups", wire::Schema::Str16),
    ("mentionsOfMe", wire::Schema::Bool),
    ("invitations", wire::Schema::Bool),
    ("memberEvents", wire::Schema::Bool),
    ("notices", wire::Schema::Bool),
]);

/// `shouldNotify` over the binary wire.
#[napi(js_name = "shouldNotifyWire")]
pub fn should_notify_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let message: serde_json::Value = wire::read_value(r, &S_SHOULD_NOTIFY_0)?;
        let level: String = r.string16()?;
        let settings: serde_json::Value = wire::read_value(r, &S_SHOULD_NOTIFY_2)?;
        let out = &port::should_notify(message, level, settings)?;
        w.bool(*out);
        Ok(())
    })
}

/// `parsePermalink` over the binary wire.
#[napi(js_name = "parsePermalinkWire")]
pub fn parse_permalink_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let href: String = r.string16()?;
        let app_origin: String = r.string16()?;
        let out = &port::parse_permalink(href, app_origin)?;
        wire::write_dyn(w, &serde_json::to_value(out).map_err(|e| e.to_string())?);
        Ok(())
    })
}

static S_PARSE_ROOM_ADDRESS_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("via", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `parseRoomAddress` over the binary wire.
#[napi(js_name = "parseRoomAddressWire")]
pub fn parse_room_address_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let own_server: String = r.string16()?;
        let out = &port::parse_room_address(text, own_server)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_PARSE_ROOM_ADDRESS_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_READ_SEEN_STEPS_0: wire::Schema = wire::Schema::Dyn;

/// `readSeenSteps` over the binary wire.
#[napi(js_name = "readSeenStepsWire")]
pub fn read_seen_steps_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let content: serde_json::Value = wire::read_value(r, &S_READ_SEEN_STEPS_0)?;
        let out = &port::read_seen_steps(content)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_TALLY_POLL_0: wire::Schema = wire::Schema::Dyn;
static S_TALLY_POLL_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("senderId", wire::Schema::Str16),
    ("answerIds", wire::Schema::List(&wire::Schema::Str16)),
    ("sentAt", wire::Schema::Num),
]));
static S_TALLY_POLL_OUT: wire::Schema = wire::Schema::Dyn;

/// `tallyPoll` over the binary wire.
#[napi(js_name = "tallyPollWire")]
pub fn tally_poll_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let poll: serde_json::Value = wire::read_value(r, &S_TALLY_POLL_0)?;
        let responses: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_TALLY_POLL_1.inner()))?;
        let ended_at: Option<f64> = r.opt_js(|r| r.f64())?;
        let user_id: String = r.string16()?;
        let out = &port::tally_poll(poll, responses, ended_at, user_id)?;
        wire::write_value(w, out, &S_TALLY_POLL_OUT)?;
        Ok(())
    })
}

static S_PRESS_ANSWER_0: wire::Schema = wire::Schema::Dyn;

/// `pressAnswer` over the binary wire.
#[napi(js_name = "pressAnswerWire")]
pub fn press_answer_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let poll: serde_json::Value = wire::read_value(r, &S_PRESS_ANSWER_0)?;
        let answer_id: String = r.string16()?;
        let out = &port::press_answer(poll, answer_id)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.tag(1);
                w.count(x0.len());
                for x1 in x0.iter() {
                    w.string16(x1);
                }
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_READ_NEW_POLL_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("question", wire::Schema::Str16),
    ("answers", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `readNewPoll` over the binary wire.
#[napi(js_name = "readNewPollWire")]
pub fn read_new_poll_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let question: String = r.string16()?;
        let answers: Vec<String> = r.list(|r| r.string16())?;
        let out = &port::read_new_poll(question, answers)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_READ_NEW_POLL_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_COMPUTE_ROOM_UNREAD_0: wire::Schema = wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    (
        "main",
        wire::Schema::Obj(&[
            ("latestEventId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("requestCount", wire::Schema::OptJs(&wire::Schema::Num)),
        ]),
    ),
    (
        "threads",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
]);
static S_COMPUTE_ROOM_UNREAD_1: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("notificationLevel", wire::Schema::Str16),
    ("isMarkedUnread", wire::Schema::Bool),
    ("tag", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "folders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
]));
static S_COMPUTE_ROOM_UNREAD_OUT: wire::Schema = wire::Schema::Obj(&[
    ("unreadCount", wire::Schema::Num),
    ("mentionCount", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
]);

/// `computeRoomUnread` over the binary wire.
#[napi(js_name = "computeRoomUnreadWire")]
pub fn compute_room_unread_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let state: serde_json::Value = wire::read_value(r, &S_COMPUTE_ROOM_UNREAD_0)?;
        let flags: Option<serde_json::Value> =
            r.opt_js(|r| wire::read_value(r, S_COMPUTE_ROOM_UNREAD_1.inner()))?;
        let out = &port::compute_room_unread(state, flags)?;
        wire::write_value(w, out, &S_COMPUTE_ROOM_UNREAD_OUT)?;
        Ok(())
    })
}

static S_COMPUTE_SILENT_UNREAD_0: wire::Schema = wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    (
        "main",
        wire::Schema::Obj(&[
            ("latestEventId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("requestCount", wire::Schema::OptJs(&wire::Schema::Num)),
        ]),
    ),
    (
        "threads",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
]);
static S_COMPUTE_SILENT_UNREAD_1: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("notificationLevel", wire::Schema::Str16),
    ("isMarkedUnread", wire::Schema::Bool),
    ("tag", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "folders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
]));

/// `computeSilentUnread` over the binary wire.
#[napi(js_name = "computeSilentUnreadWire")]
pub fn compute_silent_unread_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let state: serde_json::Value = wire::read_value(r, &S_COMPUTE_SILENT_UNREAD_0)?;
        let flags: Option<serde_json::Value> =
            r.opt_js(|r| wire::read_value(r, S_COMPUTE_SILENT_UNREAD_1.inner()))?;
        let out = &port::compute_silent_unread(state, flags)?;
        w.f64(*out);
        Ok(())
    })
}

static S_IS_UNREAD_0: wire::Schema = wire::Schema::Obj(&[
    ("unreadCount", wire::Schema::Num),
    ("mentionCount", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
]);

/// `isUnread` over the binary wire.
#[napi(js_name = "isUnreadWire")]
pub fn is_unread_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let count: serde_json::Value = wire::read_value(r, &S_IS_UNREAD_0)?;
        let out = &port::is_unread(count)?;
        w.bool(*out);
        Ok(())
    })
}

static S_COMPUTE_TOTAL_UNREAD_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("unreadCount", wire::Schema::Num),
    ("mentionCount", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
]));
static S_COMPUTE_TOTAL_UNREAD_OUT: wire::Schema = wire::Schema::Obj(&[
    ("unreadCount", wire::Schema::Num),
    ("mentionCount", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
]);

/// `computeTotalUnread` over the binary wire.
#[napi(js_name = "computeTotalUnreadWire")]
pub fn compute_total_unread_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let counts: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_COMPUTE_TOTAL_UNREAD_0.inner()))?;
        let out = &port::compute_total_unread(counts)?;
        wire::write_value(w, out, &S_COMPUTE_TOTAL_UNREAD_OUT)?;
        Ok(())
    })
}

static S_COMPUTE_RECEIPT_TARGETS_0: wire::Schema = wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    (
        "main",
        wire::Schema::Obj(&[
            ("latestEventId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("requestCount", wire::Schema::OptJs(&wire::Schema::Num)),
        ]),
    ),
    (
        "threads",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
]);
static S_COMPUTE_RECEIPT_TARGETS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("threadId", wire::Schema::Str16),
    ("eventId", wire::Schema::Str16),
]));

/// `computeReceiptTargets` over the binary wire.
#[napi(js_name = "computeReceiptTargetsWire")]
pub fn compute_receipt_targets_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let state: serde_json::Value = wire::read_value(r, &S_COMPUTE_RECEIPT_TARGETS_0)?;
        let out = &port::compute_receipt_targets(state)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_COMPUTE_RECEIPT_TARGETS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_IS_ROOM_UNREAD_0: wire::Schema = wire::Schema::Obj(&[
    ("unreadCount", wire::Schema::Num),
    ("mentionCount", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
]);

/// `isRoomUnread` over the binary wire.
#[napi(js_name = "isRoomUnreadWire")]
pub fn is_room_unread_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let unread: serde_json::Value = wire::read_value(r, &S_IS_ROOM_UNREAD_0)?;
        let silent_count: Option<f64> = r.opt_js(|r| r.f64())?;
        let out = &port::is_room_unread(unread, silent_count)?;
        w.bool(*out);
        Ok(())
    })
}

static S_SHOWN_UNREAD_COUNT_0: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    (
        "unread",
        wire::Schema::Obj(&[
            ("unreadCount", wire::Schema::Num),
            ("mentionCount", wire::Schema::Num),
            ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
        ]),
    ),
    ("silentCount", wire::Schema::OptJs(&wire::Schema::Num)),
]));

/// `shownUnreadCount` over the binary wire.
#[napi(js_name = "shownUnreadCountWire")]
pub fn shown_unread_count_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let entry: Option<serde_json::Value> =
            r.opt_js(|r| wire::read_value(r, S_SHOWN_UNREAD_COUNT_0.inner()))?;
        let out = &port::shown_unread_count(entry)?;
        w.f64(*out);
        Ok(())
    })
}

static S_EMPTY_READ_STATE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    (
        "main",
        wire::Schema::Obj(&[
            ("latestEventId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("requestCount", wire::Schema::OptJs(&wire::Schema::Num)),
        ]),
    ),
    (
        "threads",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
]);

/// `emptyReadState` over the binary wire.
#[napi(js_name = "emptyReadStateWire")]
pub fn empty_read_state_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room_id: String = r.string16()?;
        let out = &port::empty_read_state(room_id)?;
        wire::write_value(w, out, &S_EMPTY_READ_STATE_OUT)?;
        Ok(())
    })
}

static S_MAKE_READ_ROOM_STATE_0: wire::Schema = wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    (
        "main",
        wire::Schema::Obj(&[
            ("latestEventId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("requestCount", wire::Schema::OptJs(&wire::Schema::Num)),
        ]),
    ),
    (
        "threads",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
]);
static S_MAKE_READ_ROOM_STATE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    (
        "main",
        wire::Schema::Obj(&[
            ("latestEventId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("requestCount", wire::Schema::OptJs(&wire::Schema::Num)),
        ]),
    ),
    (
        "threads",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
]);

/// `makeReadRoomState` over the binary wire.
#[napi(js_name = "makeReadRoomStateWire")]
pub fn make_read_room_state_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let state: serde_json::Value = wire::read_value(r, &S_MAKE_READ_ROOM_STATE_0)?;
        let out = &port::make_read_room_state(state)?;
        wire::write_value(w, out, &S_MAKE_READ_ROOM_STATE_OUT)?;
        Ok(())
    })
}

static S_HAS_GIFS_OFF_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("isSpace", wire::Schema::Bool),
    ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
    ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("postPowerLevel", wire::Schema::Num),
    ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
]);

/// `hasGifsOff` over the binary wire.
#[napi(js_name = "hasGifsOffWire")]
pub fn has_gifs_off_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_HAS_GIFS_OFF_0)?;
        let out = &port::has_gifs_off(room)?;
        w.bool(*out);
        Ok(())
    })
}

static S_OFFERS_SCRIBE_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("isSpace", wire::Schema::Bool),
    ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
    ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("postPowerLevel", wire::Schema::Num),
    ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
]);

/// `offersScribe` over the binary wire.
#[napi(js_name = "offersScribeWire")]
pub fn offers_scribe_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_OFFERS_SCRIBE_0)?;
        let out = &port::offers_scribe(room)?;
        w.bool(*out);
        Ok(())
    })
}

static S_SPACE_OF_ROOM_0: wire::Schema = wire::Schema::Dyn;

/// `spaceOfRoom` over the binary wire.
#[napi(js_name = "spaceOfRoomWire")]
pub fn space_of_room_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_SPACE_OF_ROOM_0)?;
        let out = &port::space_of_room(room)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(0),
        }
        Ok(())
    })
}

static S_CAN_BE_LISTENED_TO_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("isSpace", wire::Schema::Bool),
    ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
    ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("postPowerLevel", wire::Schema::Num),
    ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
]);

/// `canBeListenedTo` over the binary wire.
#[napi(js_name = "canBeListenedToWire")]
pub fn can_be_listened_to_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_CAN_BE_LISTENED_TO_0)?;
        let out = &port::can_be_listened_to(room)?;
        w.bool(*out);
        Ok(())
    })
}

static S_COMPUTE_ROOM_KIND_0: wire::Schema = wire::Schema::Obj(&[
    ("isDirect", wire::Schema::Bool),
    ("postPowerLevel", wire::Schema::Num),
    ("defaultUserPowerLevel", wire::Schema::Num),
]);

/// `computeRoomKind` over the binary wire.
#[napi(js_name = "computeRoomKindWire")]
pub fn compute_room_kind_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let inputs: serde_json::Value = wire::read_value(r, &S_COMPUTE_ROOM_KIND_0)?;
        let out = &port::compute_room_kind(inputs)?;
        w.string16(out);
        Ok(())
    })
}

static S_CAN_POST_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("isSpace", wire::Schema::Bool),
    ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
    ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("postPowerLevel", wire::Schema::Num),
    ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
]);

/// `canPost` over the binary wire.
#[napi(js_name = "canPostWire")]
pub fn can_post_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_CAN_POST_0)?;
        let user_power_level: f64 = r.f64()?;
        let out = &port::can_post(room, user_power_level)?;
        w.bool(*out);
        Ok(())
    })
}

static S_FIND_SPACE_GENERAL_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("isSpace", wire::Schema::Bool),
    ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
    ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("postPowerLevel", wire::Schema::Num),
    ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
]));
static S_FIND_SPACE_GENERAL_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("isSpace", wire::Schema::Bool),
    ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
    ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("postPowerLevel", wire::Schema::Num),
    ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `findSpaceGeneral` over the binary wire.
#[napi(js_name = "findSpaceGeneralWire")]
pub fn find_space_general_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let rooms: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_FIND_SPACE_GENERAL_0.inner()))?;
        let space_id: String = r.string16()?;
        let out = &port::find_space_general(rooms, space_id)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_FIND_SPACE_GENERAL_OUT.inner())?;
            }
            None => w.tag(0),
        }
        Ok(())
    })
}

static S_READ_ARCHIVE_0: wire::Schema = wire::Schema::Dyn;
static S_READ_ARCHIVE_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("isDirect", wire::Schema::Bool),
    ("leftAt", wire::Schema::Num),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "ban",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("by", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("reason", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
]));

/// `readArchive` over the binary wire.
#[napi(js_name = "readArchiveWire")]
pub fn read_archive_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let content: serde_json::Value = wire::read_value(r, &S_READ_ARCHIVE_0)?;
        let out = &port::read_archive(content)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_READ_ARCHIVE_OUT.inner())?;
        }
        Ok(())
    })
}

static S_WITH_ARCHIVED_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("isDirect", wire::Schema::Bool),
    ("leftAt", wire::Schema::Num),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "ban",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("by", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("reason", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
]));
static S_WITH_ARCHIVED_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("isDirect", wire::Schema::Bool),
    ("leftAt", wire::Schema::Num),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "ban",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("by", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("reason", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
]));

/// `withArchived` over the binary wire.
#[napi(js_name = "withArchivedWire")]
pub fn with_archived_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let rooms: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_WITH_ARCHIVED_0.inner()))?;
        let room_id: String = r.string16()?;
        let room: Option<serde_json::Value> =
            serde_json::from_value(wire::read_dyn(r)?).map_err(|e| format!("TypeError: {e}"))?;
        let out = &port::with_archived(rooms, room_id, room)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_WITH_ARCHIVED_OUT.inner())?;
        }
        Ok(())
    })
}

static S_TO_ROOM_LINES_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("at", wire::Schema::Num),
    ("kind", wire::Schema::Str16),
    ("actorId", wire::Schema::Str16),
    ("actorName", wire::Schema::Str16),
    ("targetId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("targetName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("name", wire::Schema::OptJs(&wire::Schema::Str16)),
]));
static S_TO_ROOM_LINES_3: wire::Schema = wire::Schema::Dyn;
static S_TO_ROOM_LINES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);

/// `toRoomLines` over the binary wire.
#[napi(js_name = "toRoomLinesWire")]
pub fn to_room_lines_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let events: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_TO_ROOM_LINES_0.inner()))?;
        let message_times: Vec<f64> = r.list(|r| r.f64())?;
        let listener_id: Option<String> = r.opt_js(|r| r.string16())?;
        let deleted: serde_json::Value = wire::read_value(r, &S_TO_ROOM_LINES_3)?;
        let room_id: String = r.string16()?;
        let out = &port::to_room_lines(events, message_times, listener_id, deleted, room_id)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_TO_ROOM_LINES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_FOLDER_VIEW_0: wire::Schema = wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hidden", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("color", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("include", wire::Schema::List(&wire::Schema::Str16)),
    ("exclude", wire::Schema::List(&wire::Schema::Str16)),
]);
static S_FOLDER_VIEW_OUT: wire::Schema = wire::Schema::Dyn;

/// `folderView` over the binary wire.
#[napi(js_name = "folderViewWire")]
pub fn folder_view_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let folder: serde_json::Value = wire::read_value(r, &S_FOLDER_VIEW_0)?;
        let out = &port::folder_view(folder)?;
        wire::write_value(w, out, &S_FOLDER_VIEW_OUT)?;
        Ok(())
    })
}

static S_IS_IN_VIEW_0: wire::Schema = wire::Schema::Obj(&[
    (
        "room",
        wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("kind", wire::Schema::Str16),
            ("isSpace", wire::Schema::Bool),
            ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("postPowerLevel", wire::Schema::Num),
            ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
        ]),
    ),
    (
        "unread",
        wire::Schema::Obj(&[
            ("unreadCount", wire::Schema::Num),
            ("mentionCount", wire::Schema::Num),
            ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
        ]),
    ),
    ("tag", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "folders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
]);
static S_IS_IN_VIEW_1: wire::Schema = wire::Schema::Dyn;
static S_IS_IN_VIEW_2: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    (
        "room",
        wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("kind", wire::Schema::Str16),
            ("isSpace", wire::Schema::Bool),
            ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("postPowerLevel", wire::Schema::Num),
            ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
        ]),
    ),
    (
        "unread",
        wire::Schema::Obj(&[
            ("unreadCount", wire::Schema::Num),
            ("mentionCount", wire::Schema::Num),
            ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
        ]),
    ),
    ("tag", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "folders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
]));
static S_IS_IN_VIEW_3: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
        ("name", wire::Schema::Str16),
        ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("hidden", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("emoji", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("color", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("include", wire::Schema::List(&wire::Schema::Str16)),
        ("exclude", wire::Schema::List(&wire::Schema::Str16)),
    ])));

/// `isInView` over the binary wire.
#[napi(js_name = "isInViewWire")]
pub fn is_in_view_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let entry: serde_json::Value = wire::read_value(r, &S_IS_IN_VIEW_0)?;
        let view: serde_json::Value = wire::read_value(r, &S_IS_IN_VIEW_1)?;
        let all: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_IS_IN_VIEW_2.inner()))?;
        let folders: Option<Vec<serde_json::Value>> =
            r.opt_js(|r| r.list(|r| wire::read_value(r, S_IS_IN_VIEW_3.inner().inner())))?;
        let out = &port::is_in_view(entry, view, all, folders)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_ROOM_OWNER_0: wire::Schema = wire::Schema::Obj(&[
    ("canInvite", wire::Schema::Bool),
    ("canRemove", wire::Schema::Bool),
    ("canBan", wire::Schema::Bool),
    ("canSetLevels", wire::Schema::Bool),
    ("readOnlyLevel", wire::Schema::Num),
    ("defaultLevel", wire::Schema::Num),
    ("canRedact", wire::Schema::Bool),
    ("canEditSettings", wire::Schema::Bool),
    ("canEncrypt", wire::Schema::Bool),
    ("canSetJoinRules", wire::Schema::Bool),
    ("canAddChildren", wire::Schema::Bool),
    ("ownPowerLevel", wire::Schema::Num),
    ("canMentionEveryone", wire::Schema::Bool),
    ("isManagedBySpace", wire::Schema::Bool),
    ("canTransferOwnership", wire::Schema::Bool),
    ("canAddOwner", wire::Schema::Bool),
]);

/// `isRoomOwner` over the binary wire.
#[napi(js_name = "isRoomOwnerWire")]
pub fn is_room_owner_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let permissions: serde_json::Value = wire::read_value(r, &S_IS_ROOM_OWNER_0)?;
        let out = &port::is_room_owner(permissions)?;
        w.bool(*out);
        Ok(())
    })
}

static S_CAN_MANAGE_MEMBER_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("powerLevel", wire::Schema::Num),
    ("role", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
    ("isBot", wire::Schema::Bool),
    ("isReadOnly", wire::Schema::Bool),
    ("isCreator", wire::Schema::Bool),
    ("isOwner", wire::Schema::Bool),
]);
static S_CAN_MANAGE_MEMBER_1: wire::Schema = wire::Schema::Obj(&[
    ("canInvite", wire::Schema::Bool),
    ("canRemove", wire::Schema::Bool),
    ("canBan", wire::Schema::Bool),
    ("canSetLevels", wire::Schema::Bool),
    ("readOnlyLevel", wire::Schema::Num),
    ("defaultLevel", wire::Schema::Num),
    ("canRedact", wire::Schema::Bool),
    ("canEditSettings", wire::Schema::Bool),
    ("canEncrypt", wire::Schema::Bool),
    ("canSetJoinRules", wire::Schema::Bool),
    ("canAddChildren", wire::Schema::Bool),
    ("ownPowerLevel", wire::Schema::Num),
    ("canMentionEveryone", wire::Schema::Bool),
    ("isManagedBySpace", wire::Schema::Bool),
    ("canTransferOwnership", wire::Schema::Bool),
    ("canAddOwner", wire::Schema::Bool),
]);

/// `canManageMember` over the binary wire.
#[napi(js_name = "canManageMemberWire")]
pub fn can_manage_member_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let member: serde_json::Value = wire::read_value(r, &S_CAN_MANAGE_MEMBER_0)?;
        let permissions: serde_json::Value = wire::read_value(r, &S_CAN_MANAGE_MEMBER_1)?;
        let is_self: bool = r.bool()?;
        let out = &port::can_manage_member(member, permissions, is_self)?;
        w.bool(*out);
        Ok(())
    })
}

static S_CAN_CHANGE_ROLE_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("powerLevel", wire::Schema::Num),
    ("role", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
    ("isBot", wire::Schema::Bool),
    ("isReadOnly", wire::Schema::Bool),
    ("isCreator", wire::Schema::Bool),
    ("isOwner", wire::Schema::Bool),
]);
static S_CAN_CHANGE_ROLE_1: wire::Schema = wire::Schema::Obj(&[
    ("canInvite", wire::Schema::Bool),
    ("canRemove", wire::Schema::Bool),
    ("canBan", wire::Schema::Bool),
    ("canSetLevels", wire::Schema::Bool),
    ("readOnlyLevel", wire::Schema::Num),
    ("defaultLevel", wire::Schema::Num),
    ("canRedact", wire::Schema::Bool),
    ("canEditSettings", wire::Schema::Bool),
    ("canEncrypt", wire::Schema::Bool),
    ("canSetJoinRules", wire::Schema::Bool),
    ("canAddChildren", wire::Schema::Bool),
    ("ownPowerLevel", wire::Schema::Num),
    ("canMentionEveryone", wire::Schema::Bool),
    ("isManagedBySpace", wire::Schema::Bool),
    ("canTransferOwnership", wire::Schema::Bool),
    ("canAddOwner", wire::Schema::Bool),
]);

/// `canChangeRole` over the binary wire.
#[napi(js_name = "canChangeRoleWire")]
pub fn can_change_role_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let member: serde_json::Value = wire::read_value(r, &S_CAN_CHANGE_ROLE_0)?;
        let permissions: serde_json::Value = wire::read_value(r, &S_CAN_CHANGE_ROLE_1)?;
        let is_self: bool = r.bool()?;
        let out = &port::can_change_role(member, permissions, is_self)?;
        w.bool(*out);
        Ok(())
    })
}

static S_ASSIGNABLE_ROLES_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("powerLevel", wire::Schema::Num),
    ("role", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
    ("isBot", wire::Schema::Bool),
    ("isReadOnly", wire::Schema::Bool),
    ("isCreator", wire::Schema::Bool),
    ("isOwner", wire::Schema::Bool),
]);
static S_ASSIGNABLE_ROLES_1: wire::Schema = wire::Schema::Obj(&[
    ("canInvite", wire::Schema::Bool),
    ("canRemove", wire::Schema::Bool),
    ("canBan", wire::Schema::Bool),
    ("canSetLevels", wire::Schema::Bool),
    ("readOnlyLevel", wire::Schema::Num),
    ("defaultLevel", wire::Schema::Num),
    ("canRedact", wire::Schema::Bool),
    ("canEditSettings", wire::Schema::Bool),
    ("canEncrypt", wire::Schema::Bool),
    ("canSetJoinRules", wire::Schema::Bool),
    ("canAddChildren", wire::Schema::Bool),
    ("ownPowerLevel", wire::Schema::Num),
    ("canMentionEveryone", wire::Schema::Bool),
    ("isManagedBySpace", wire::Schema::Bool),
    ("canTransferOwnership", wire::Schema::Bool),
    ("canAddOwner", wire::Schema::Bool),
]);

/// `assignableRoles` over the binary wire.
#[napi(js_name = "assignableRolesWire")]
pub fn assignable_roles_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let member: serde_json::Value = wire::read_value(r, &S_ASSIGNABLE_ROLES_0)?;
        let permissions: serde_json::Value = wire::read_value(r, &S_ASSIGNABLE_ROLES_1)?;
        let is_self: bool = r.bool()?;
        let out = &port::assignable_roles(member, permissions, is_self)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_CAN_BAN_MEMBER_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("powerLevel", wire::Schema::Num),
    ("role", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
    ("isBot", wire::Schema::Bool),
    ("isReadOnly", wire::Schema::Bool),
    ("isCreator", wire::Schema::Bool),
    ("isOwner", wire::Schema::Bool),
]);
static S_CAN_BAN_MEMBER_1: wire::Schema = wire::Schema::Obj(&[
    ("canInvite", wire::Schema::Bool),
    ("canRemove", wire::Schema::Bool),
    ("canBan", wire::Schema::Bool),
    ("canSetLevels", wire::Schema::Bool),
    ("readOnlyLevel", wire::Schema::Num),
    ("defaultLevel", wire::Schema::Num),
    ("canRedact", wire::Schema::Bool),
    ("canEditSettings", wire::Schema::Bool),
    ("canEncrypt", wire::Schema::Bool),
    ("canSetJoinRules", wire::Schema::Bool),
    ("canAddChildren", wire::Schema::Bool),
    ("ownPowerLevel", wire::Schema::Num),
    ("canMentionEveryone", wire::Schema::Bool),
    ("isManagedBySpace", wire::Schema::Bool),
    ("canTransferOwnership", wire::Schema::Bool),
    ("canAddOwner", wire::Schema::Bool),
]);

/// `canBanMember` over the binary wire.
#[napi(js_name = "canBanMemberWire")]
pub fn can_ban_member_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let member: serde_json::Value = wire::read_value(r, &S_CAN_BAN_MEMBER_0)?;
        let permissions: serde_json::Value = wire::read_value(r, &S_CAN_BAN_MEMBER_1)?;
        let is_self: bool = r.bool()?;
        let out = &port::can_ban_member(member, permissions, is_self)?;
        w.bool(*out);
        Ok(())
    })
}

static S_CAN_SET_READ_ONLY_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("powerLevel", wire::Schema::Num),
    ("role", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
    ("isBot", wire::Schema::Bool),
    ("isReadOnly", wire::Schema::Bool),
    ("isCreator", wire::Schema::Bool),
    ("isOwner", wire::Schema::Bool),
]);
static S_CAN_SET_READ_ONLY_1: wire::Schema = wire::Schema::Obj(&[
    ("canInvite", wire::Schema::Bool),
    ("canRemove", wire::Schema::Bool),
    ("canBan", wire::Schema::Bool),
    ("canSetLevels", wire::Schema::Bool),
    ("readOnlyLevel", wire::Schema::Num),
    ("defaultLevel", wire::Schema::Num),
    ("canRedact", wire::Schema::Bool),
    ("canEditSettings", wire::Schema::Bool),
    ("canEncrypt", wire::Schema::Bool),
    ("canSetJoinRules", wire::Schema::Bool),
    ("canAddChildren", wire::Schema::Bool),
    ("ownPowerLevel", wire::Schema::Num),
    ("canMentionEveryone", wire::Schema::Bool),
    ("isManagedBySpace", wire::Schema::Bool),
    ("canTransferOwnership", wire::Schema::Bool),
    ("canAddOwner", wire::Schema::Bool),
]);

/// `canSetReadOnly` over the binary wire.
#[napi(js_name = "canSetReadOnlyWire")]
pub fn can_set_read_only_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let member: serde_json::Value = wire::read_value(r, &S_CAN_SET_READ_ONLY_0)?;
        let permissions: serde_json::Value = wire::read_value(r, &S_CAN_SET_READ_ONLY_1)?;
        let is_self: bool = r.bool()?;
        let out = &port::can_set_read_only(member, permissions, is_self)?;
        w.bool(*out);
        Ok(())
    })
}

static S_OWNERSHIP_ACTION_0: wire::Schema = wire::Schema::Obj(&[
    ("canInvite", wire::Schema::Bool),
    ("canRemove", wire::Schema::Bool),
    ("canBan", wire::Schema::Bool),
    ("canSetLevels", wire::Schema::Bool),
    ("readOnlyLevel", wire::Schema::Num),
    ("defaultLevel", wire::Schema::Num),
    ("canRedact", wire::Schema::Bool),
    ("canEditSettings", wire::Schema::Bool),
    ("canEncrypt", wire::Schema::Bool),
    ("canSetJoinRules", wire::Schema::Bool),
    ("canAddChildren", wire::Schema::Bool),
    ("ownPowerLevel", wire::Schema::Num),
    ("canMentionEveryone", wire::Schema::Bool),
    ("isManagedBySpace", wire::Schema::Bool),
    ("canTransferOwnership", wire::Schema::Bool),
    ("canAddOwner", wire::Schema::Bool),
]);

/// `ownershipAction` over the binary wire.
#[napi(js_name = "ownershipActionWire")]
pub fn ownership_action_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let permissions: serde_json::Value = wire::read_value(r, &S_OWNERSHIP_ACTION_0)?;
        let out = &port::ownership_action(permissions)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_CAN_TAKE_OWNERSHIP_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("powerLevel", wire::Schema::Num),
    ("role", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
    ("isBot", wire::Schema::Bool),
    ("isReadOnly", wire::Schema::Bool),
    ("isCreator", wire::Schema::Bool),
    ("isOwner", wire::Schema::Bool),
]);
static S_CAN_TAKE_OWNERSHIP_1: wire::Schema = wire::Schema::Obj(&[
    ("canInvite", wire::Schema::Bool),
    ("canRemove", wire::Schema::Bool),
    ("canBan", wire::Schema::Bool),
    ("canSetLevels", wire::Schema::Bool),
    ("readOnlyLevel", wire::Schema::Num),
    ("defaultLevel", wire::Schema::Num),
    ("canRedact", wire::Schema::Bool),
    ("canEditSettings", wire::Schema::Bool),
    ("canEncrypt", wire::Schema::Bool),
    ("canSetJoinRules", wire::Schema::Bool),
    ("canAddChildren", wire::Schema::Bool),
    ("ownPowerLevel", wire::Schema::Num),
    ("canMentionEveryone", wire::Schema::Bool),
    ("isManagedBySpace", wire::Schema::Bool),
    ("canTransferOwnership", wire::Schema::Bool),
    ("canAddOwner", wire::Schema::Bool),
]);

/// `canTakeOwnership` over the binary wire.
#[napi(js_name = "canTakeOwnershipWire")]
pub fn can_take_ownership_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let member: serde_json::Value = wire::read_value(r, &S_CAN_TAKE_OWNERSHIP_0)?;
        let permissions: serde_json::Value = wire::read_value(r, &S_CAN_TAKE_OWNERSHIP_1)?;
        let is_self: bool = r.bool()?;
        let out = &port::can_take_ownership(member, permissions, is_self)?;
        w.bool(*out);
        Ok(())
    })
}

static S_CAN_TRANSFER_TO_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("powerLevel", wire::Schema::Num),
    ("role", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
    ("isBot", wire::Schema::Bool),
    ("isReadOnly", wire::Schema::Bool),
    ("isCreator", wire::Schema::Bool),
    ("isOwner", wire::Schema::Bool),
]);
static S_CAN_TRANSFER_TO_1: wire::Schema = wire::Schema::Obj(&[
    ("canInvite", wire::Schema::Bool),
    ("canRemove", wire::Schema::Bool),
    ("canBan", wire::Schema::Bool),
    ("canSetLevels", wire::Schema::Bool),
    ("readOnlyLevel", wire::Schema::Num),
    ("defaultLevel", wire::Schema::Num),
    ("canRedact", wire::Schema::Bool),
    ("canEditSettings", wire::Schema::Bool),
    ("canEncrypt", wire::Schema::Bool),
    ("canSetJoinRules", wire::Schema::Bool),
    ("canAddChildren", wire::Schema::Bool),
    ("ownPowerLevel", wire::Schema::Num),
    ("canMentionEveryone", wire::Schema::Bool),
    ("isManagedBySpace", wire::Schema::Bool),
    ("canTransferOwnership", wire::Schema::Bool),
    ("canAddOwner", wire::Schema::Bool),
]);

/// `canTransferTo` over the binary wire.
#[napi(js_name = "canTransferToWire")]
pub fn can_transfer_to_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let member: serde_json::Value = wire::read_value(r, &S_CAN_TRANSFER_TO_0)?;
        let permissions: serde_json::Value = wire::read_value(r, &S_CAN_TRANSFER_TO_1)?;
        let is_self: bool = r.bool()?;
        let out = &port::can_transfer_to(member, permissions, is_self)?;
        w.bool(*out);
        Ok(())
    })
}

static S_HAS_STAFF_ROLE_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("powerLevel", wire::Schema::Num),
    ("role", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
    ("isBot", wire::Schema::Bool),
    ("isReadOnly", wire::Schema::Bool),
    ("isCreator", wire::Schema::Bool),
    ("isOwner", wire::Schema::Bool),
]);

/// `hasStaffRole` over the binary wire.
#[napi(js_name = "hasStaffRoleWire")]
pub fn has_staff_role_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let member: serde_json::Value = wire::read_value(r, &S_HAS_STAFF_ROLE_0)?;
        let out = &port::has_staff_role(member)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PERMISSION_ROLE_OF_0: wire::Schema = wire::Schema::Obj(&[
    ("userId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("matrixName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("powerLevel", wire::Schema::Num),
    ("role", wire::Schema::Str16),
    ("isInvited", wire::Schema::Bool),
    ("isBot", wire::Schema::Bool),
    ("isReadOnly", wire::Schema::Bool),
    ("isCreator", wire::Schema::Bool),
    ("isOwner", wire::Schema::Bool),
]);
static S_PERMISSION_ROLE_OF_1: wire::Schema = wire::Schema::Dyn;
static S_PERMISSION_ROLE_OF_OUT: wire::Schema = wire::Schema::Dyn;

/// `permissionRoleOf` over the binary wire.
#[napi(js_name = "permissionRoleOfWire")]
pub fn permission_role_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let member: serde_json::Value = wire::read_value(r, &S_PERMISSION_ROLE_OF_0)?;
        let permissions: serde_json::Value = wire::read_value(r, &S_PERMISSION_ROLE_OF_1)?;
        let out = &port::permission_role_of(member, permissions)?;
        wire::write_value(w, out, &S_PERMISSION_ROLE_OF_OUT)?;
        Ok(())
    })
}

static S_GIVEN_ROLES_0: wire::Schema = wire::Schema::Dyn;

/// `givenRoles` over the binary wire.
#[napi(js_name = "givenRolesWire")]
pub fn given_roles_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let permissions: serde_json::Value = wire::read_value(r, &S_GIVEN_ROLES_0)?;
        let out = &port::given_roles(permissions)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_dyn(w, &serde_json::to_value(x0).map_err(|e| e.to_string())?);
        }
        Ok(())
    })
}

static S_CAN_HAVE_PUBLIC_PAGE_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("isSpace", wire::Schema::Bool),
    ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
    ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("postPowerLevel", wire::Schema::Num),
    ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
]);

/// `canHavePublicPage` over the binary wire.
#[napi(js_name = "canHavePublicPageWire")]
pub fn can_have_public_page_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_CAN_HAVE_PUBLIC_PAGE_0)?;
        let out = &port::can_have_public_page(room)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PUBLIC_PAGE_PATH_0: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("isSpace", wire::Schema::Bool),
    ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
    ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("postPowerLevel", wire::Schema::Num),
    ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
]);

/// `publicPagePath` over the binary wire.
#[napi(js_name = "publicPagePathWire")]
pub fn public_page_path_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_PUBLIC_PAGE_PATH_0)?;
        let out = &port::public_page_path(room)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_ROOM_ID_OF_ROUTE_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `roomIdOfRoute` over the binary wire.
#[napi(js_name = "roomIdOfRouteWire")]
pub fn room_id_of_route_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let route_id: String = r.string16()?;
        let rooms: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_ROOM_ID_OF_ROUTE_1.inner()))?;
        let out = &port::room_id_of_route(route_id, rooms)?;
        w.string16(out);
        Ok(())
    })
}

static S_READ_SAVED_REFS_0: wire::Schema = wire::Schema::Dyn;
static S_READ_SAVED_REFS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("eventId", wire::Schema::Str16),
    ("savedAt", wire::Schema::Num),
]));

/// `readSavedRefs` over the binary wire.
#[napi(js_name = "readSavedRefsWire")]
pub fn read_saved_refs_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let content: serde_json::Value = wire::read_value(r, &S_READ_SAVED_REFS_0)?;
        let out = &port::read_saved_refs(content)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_READ_SAVED_REFS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_WITH_SAVED_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("eventId", wire::Schema::Str16),
    ("savedAt", wire::Schema::Num),
]));
static S_WITH_SAVED_4: wire::Schema = wire::Schema::Num;
static S_WITH_SAVED_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("eventId", wire::Schema::Str16),
    ("savedAt", wire::Schema::Num),
]));

/// `withSaved` over the binary wire.
#[napi(js_name = "withSavedWire")]
pub fn with_saved_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let refs: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_WITH_SAVED_0.inner()))?;
        let room_id: String = r.string16()?;
        let event_id: String = r.string16()?;
        let is_saved: bool = r.bool()?;
        let now: serde_json::Value = wire::read_value(r, &S_WITH_SAVED_4)?;
        let out = &port::with_saved(refs, room_id, event_id, is_saved, now)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_WITH_SAVED_OUT.inner())?;
        }
        Ok(())
    })
}

static S_READ_AUTO_TRANSLATE_0: wire::Schema = wire::Schema::Dyn;

/// `readAutoTranslate` over the binary wire.
#[napi(js_name = "readAutoTranslateWire")]
pub fn read_auto_translate_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let content: serde_json::Value = wire::read_value(r, &S_READ_AUTO_TRANSLATE_0)?;
        let out = &port::read_auto_translate(content)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_TO_INDEXED_MESSAGE_1: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            (
                "dimensions",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("width", wire::Schema::Dyn),
                    ("height", wire::Schema::Dyn),
                ])),
            ),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "voice",
                wire::Schema::OptJs(&wire::Schema::Obj(&[("waveform", wire::Schema::Dyn)])),
            ),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "location",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("lat", wire::Schema::Num),
            ("lon", wire::Schema::Num),
        ])),
    ),
    (
        "poll",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("question", wire::Schema::Str16),
            (
                "answers",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("id", wire::Schema::Dyn),
                    ("text", wire::Schema::Dyn),
                ])),
            ),
            ("maxSelections", wire::Schema::Num),
            ("isDisclosed", wire::Schema::Bool),
            ("isAnonymous", wire::Schema::Bool),
            ("endedAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("votes", wire::Schema::Dyn),
            ("voters", wire::Schema::Dyn),
            ("totalVotes", wire::Schema::Num),
            ("ownAnswerIds", wire::Schema::List(&wire::Schema::Str16)),
            ("canVote", wire::Schema::Bool),
            ("canEnd", wire::Schema::Bool),
            ("canEdit", wire::Schema::Bool),
        ])),
    ),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "call",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("direction", wire::Schema::Str16),
            ("isVideo", wire::Schema::Bool),
            ("outcome", wire::Schema::Str16),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    (
        "thread",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("threadId", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("replyCount", wire::Schema::Num),
            ("lastReply", wire::Schema::Dyn),
            (
                "unread",
                wire::Schema::Obj(&[
                    ("unreadCount", wire::Schema::Dyn),
                    ("mentionCount", wire::Schema::Dyn),
                    ("isMarkedUnread", wire::Schema::Dyn),
                ]),
            ),
            ("isParticipating", wire::Schema::Bool),
            ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]);
static S_TO_INDEXED_MESSAGE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("messageId", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("text", wire::Schema::Str16),
    ("sentAt", wire::Schema::Num),
    ("hasFile", wire::Schema::Bool),
]));

/// `toIndexedMessage` over the binary wire.
#[napi(js_name = "toIndexedMessageWire")]
pub fn to_indexed_message_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room_id: String = r.string16()?;
        let message: serde_json::Value = wire::read_value(r, &S_TO_INDEXED_MESSAGE_1)?;
        let out = &port::to_indexed_message(room_id, message)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_TO_INDEXED_MESSAGE_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_IS_EMPTY_FILTER_0: wire::Schema = wire::Schema::Obj(&[
    ("query", wire::Schema::Str16),
    ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("since", wire::Schema::OptJs(&wire::Schema::Num)),
    ("hasFile", wire::Schema::Bool),
]);

/// `isEmptyFilter` over the binary wire.
#[napi(js_name = "isEmptyFilterWire")]
pub fn is_empty_filter_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let filter: serde_json::Value = wire::read_value(r, &S_IS_EMPTY_FILTER_0)?;
        let out = &port::is_empty_filter(filter)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MATCHES_FILTER_0: wire::Schema = wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("messageId", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("text", wire::Schema::Str16),
    ("sentAt", wire::Schema::Num),
    ("hasFile", wire::Schema::Bool),
]);
static S_MATCHES_FILTER_1: wire::Schema = wire::Schema::Obj(&[
    ("query", wire::Schema::Str16),
    ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("since", wire::Schema::OptJs(&wire::Schema::Num)),
    ("hasFile", wire::Schema::Bool),
]);

/// `matchesFilter` over the binary wire.
#[napi(js_name = "matchesFilterWire")]
pub fn matches_filter_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let entry: serde_json::Value = wire::read_value(r, &S_MATCHES_FILTER_0)?;
        let filter: serde_json::Value = wire::read_value(r, &S_MATCHES_FILTER_1)?;
        let out = &port::matches_filter(entry, filter)?;
        w.bool(*out);
        Ok(())
    })
}

static S_COMPUTE_SHARED_CONTENT_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("senderId", wire::Schema::Str16),
    ("senderName", wire::Schema::Str16),
    ("senderAvatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isFromBot", wire::Schema::Bool),
    ("text", wire::Schema::Str16),
    ("html", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("undecryptable", wire::Schema::Dyn),
    ("notice", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "attachment",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("kind", wire::Schema::Dyn),
            ("name", wire::Schema::Str16),
            ("mimeType", wire::Schema::Str16),
            ("size", wire::Schema::OptJs(&wire::Schema::Num)),
            ("isEncrypted", wire::Schema::Bool),
            ("dimensions", wire::Schema::Dyn),
            ("duration", wire::Schema::OptJs(&wire::Schema::Num)),
            ("hasThumbnail", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("voice", wire::Schema::Dyn),
            ("batch", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    ("location", wire::Schema::Dyn),
    ("poll", wire::Schema::Dyn),
    ("question", wire::Schema::Dyn),
    ("isEmote", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mentionsRoom", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isCallThreadRoot", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("call", wire::Schema::Dyn),
    ("sentAt", wire::Schema::Num),
    ("isOwn", wire::Schema::Bool),
    ("isReadByOthers", wire::Schema::Bool),
    (
        "lastReadBy",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("userId", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("readAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ]))),
    ),
    ("thread", wire::Schema::Dyn),
    (
        "replyTo",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("senderName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senderId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("text", wire::Schema::Str16),
            ("isDeleted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("isEdited", wire::Schema::Bool),
    (
        "reactions",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("count", wire::Schema::Num),
            ("ownReactionId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("senders", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("sendStatus", wire::Schema::Str16),
]));
static S_COMPUTE_SHARED_CONTENT_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "media",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            (
                "attachment",
                wire::Schema::Obj(&[
                    ("kind", wire::Schema::Dyn),
                    ("name", wire::Schema::Dyn),
                    ("mimeType", wire::Schema::Dyn),
                    ("size", wire::Schema::Dyn),
                    ("isEncrypted", wire::Schema::Dyn),
                    ("dimensions", wire::Schema::Dyn),
                    ("duration", wire::Schema::Dyn),
                    ("hasThumbnail", wire::Schema::Dyn),
                    ("voice", wire::Schema::Dyn),
                    ("batch", wire::Schema::Dyn),
                ]),
            ),
            ("senderName", wire::Schema::Str16),
            ("sentAt", wire::Schema::Num),
        ])),
    ),
    (
        "links",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            ("url", wire::Schema::Str16),
            ("senderName", wire::Schema::Str16),
            ("sentAt", wire::Schema::Num),
        ])),
    ),
    (
        "files",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("messageId", wire::Schema::Str16),
            (
                "attachment",
                wire::Schema::Obj(&[
                    ("kind", wire::Schema::Dyn),
                    ("name", wire::Schema::Dyn),
                    ("mimeType", wire::Schema::Dyn),
                    ("size", wire::Schema::Dyn),
                    ("isEncrypted", wire::Schema::Dyn),
                    ("dimensions", wire::Schema::Dyn),
                    ("duration", wire::Schema::Dyn),
                    ("hasThumbnail", wire::Schema::Dyn),
                    ("voice", wire::Schema::Dyn),
                    ("batch", wire::Schema::Dyn),
                ]),
            ),
            ("senderName", wire::Schema::Str16),
            ("sentAt", wire::Schema::Num),
        ])),
    ),
]);

/// `computeSharedContent` over the binary wire.
#[napi(js_name = "computeSharedContentWire")]
pub fn compute_shared_content_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let messages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_COMPUTE_SHARED_CONTENT_0.inner()))?;
        let out = &port::compute_shared_content(messages)?;
        wire::write_value(w, out, &S_COMPUTE_SHARED_CONTENT_OUT)?;
        Ok(())
    })
}

static S_ROOM_BADGE_COUNT_0: wire::Schema = wire::Schema::Obj(&[
    ("unreadCount", wire::Schema::Num),
    ("mentionCount", wire::Schema::Num),
    ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
]);

/// `roomBadgeCount` over the binary wire.
#[napi(js_name = "roomBadgeCountWire")]
pub fn room_badge_count_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let unread: serde_json::Value = wire::read_value(r, &S_ROOM_BADGE_COUNT_0)?;
        let out = &port::room_badge_count(unread)?;
        w.f64(*out);
        Ok(())
    })
}

static S_COMPUTE_SPACE_BADGES_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    (
        "room",
        wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("kind", wire::Schema::Str16),
            ("isSpace", wire::Schema::Bool),
            ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("postPowerLevel", wire::Schema::Num),
            ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
        ]),
    ),
    (
        "unread",
        wire::Schema::Obj(&[
            ("unreadCount", wire::Schema::Num),
            ("mentionCount", wire::Schema::Num),
            ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
        ]),
    ),
]));
static S_COMPUTE_SPACE_BADGES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("spaceId", wire::Schema::Str16),
    ("count", wire::Schema::Num),
]));

/// `computeSpaceBadges` over the binary wire.
#[napi(js_name = "computeSpaceBadgesWire")]
pub fn compute_space_badges_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let entries: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_COMPUTE_SPACE_BADGES_0.inner()))?;
        let out = &port::compute_space_badges(entries)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_COMPUTE_SPACE_BADGES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_CAN_MOVE_TO_SPACE_0: wire::Schema = wire::Schema::Dyn;
static S_CAN_MOVE_TO_SPACE_1: wire::Schema = wire::Schema::Dyn;

/// `canMoveToSpace` over the binary wire.
#[napi(js_name = "canMoveToSpaceWire")]
pub fn can_move_to_space_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let room: serde_json::Value = wire::read_value(r, &S_CAN_MOVE_TO_SPACE_0)?;
        let permissions: serde_json::Value = wire::read_value(r, &S_CAN_MOVE_TO_SPACE_1)?;
        let out = &port::can_move_to_space(room, permissions)?;
        w.bool(*out);
        Ok(())
    })
}

static S_ARRANGE_ROOMS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    (
        "room",
        wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("kind", wire::Schema::Str16),
            ("isSpace", wire::Schema::Bool),
            ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("postPowerLevel", wire::Schema::Num),
            ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
        ]),
    ),
    ("tag", wire::Schema::OptJs(&wire::Schema::Str16)),
]));
static S_ARRANGE_ROOMS_OUT: wire::Schema = wire::Schema::Dyn;

/// `arrangeRooms` over the binary wire.
#[napi(js_name = "arrangeRoomsWire")]
pub fn arrange_rooms_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let items: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_ARRANGE_ROOMS_0.inner()))?;
        let pinned_first: Option<bool> = r.opt_js(|r| r.bool())?;
        let out = &port::arrange_rooms(items, pinned_first)?;
        wire::write_value(w, out, &S_ARRANGE_ROOMS_OUT)?;
        Ok(())
    })
}

static S_STAYS_IN_TREE_0: wire::Schema = wire::Schema::Obj(&[
    (
        "room",
        wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("kind", wire::Schema::Str16),
            ("isSpace", wire::Schema::Bool),
            ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
            ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("postPowerLevel", wire::Schema::Num),
            ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
        ]),
    ),
    ("tag", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `staysInTree` over the binary wire.
#[napi(js_name = "staysInTreeWire")]
pub fn stays_in_tree_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let item: serde_json::Value = wire::read_value(r, &S_STAYS_IN_TREE_0)?;
        let out = &port::stays_in_tree(item)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PARENT_SPACE_OF_0: wire::Schema = wire::Schema::Obj(&[(
    "room",
    wire::Schema::Obj(&[
        ("id", wire::Schema::Str16),
        ("name", wire::Schema::Str16),
        ("kind", wire::Schema::Str16),
        ("isSpace", wire::Schema::Bool),
        ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("postPowerLevel", wire::Schema::Num),
        ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
    ]),
)]);
static S_PARENT_SPACE_OF_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[(
    "room",
    wire::Schema::Obj(&[
        ("id", wire::Schema::Str16),
        ("name", wire::Schema::Str16),
        ("kind", wire::Schema::Str16),
        ("isSpace", wire::Schema::Bool),
        ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("postPowerLevel", wire::Schema::Num),
        ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
    ]),
)]));
static S_PARENT_SPACE_OF_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[(
    "room",
    wire::Schema::Obj(&[
        ("id", wire::Schema::Str16),
        ("name", wire::Schema::Str16),
        ("kind", wire::Schema::Str16),
        ("isSpace", wire::Schema::Bool),
        ("directUserId", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("hasName", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isVoice", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isPage", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("avatarUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("alias", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("isEncrypted", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isWorldReadable", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isOpenToAll", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("isAlone", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("hasAssistant", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("createdAt", wire::Schema::OptJs(&wire::Schema::Num)),
        ("gifsOff", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("scribeOn", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("hasPublicPage", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("replacedBy", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("isPeerGone", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("postPowerLevel", wire::Schema::Num),
        ("parentSpaceIds", wire::Schema::List(&wire::Schema::Str16)),
    ]),
)]));

/// `parentSpaceOf` over the binary wire.
#[napi(js_name = "parentSpaceOfWire")]
pub fn parent_space_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let item: serde_json::Value = wire::read_value(r, &S_PARENT_SPACE_OF_0)?;
        let items: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_PARENT_SPACE_OF_1.inner()))?;
        let out = &port::parent_space_of(item, items)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_PARENT_SPACE_OF_OUT.inner())?;
            }
            None => w.tag(0),
        }
        Ok(())
    })
}

static S_PRESENCE_OF_PRESET_0: wire::Schema = wire::Schema::Dyn;

/// `presenceOfPreset` over the binary wire.
#[napi(js_name = "presenceOfPresetWire")]
pub fn presence_of_preset_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let preset: serde_json::Value = wire::read_value(r, &S_PRESENCE_OF_PRESET_0)?;
        let out = &port::presence_of_preset(preset)?;
        w.string16(out);
        Ok(())
    })
}

static S_SORT_THREADS_BY_ACTIVITY_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("threadId", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("replyCount", wire::Schema::Num),
    (
        "lastReply",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("senderId", wire::Schema::Str16),
            ("senderName", wire::Schema::Str16),
            ("text", wire::Schema::Str16),
            ("sentAt", wire::Schema::Num),
        ])),
    ),
    (
        "unread",
        wire::Schema::Obj(&[
            ("unreadCount", wire::Schema::Num),
            ("mentionCount", wire::Schema::Num),
            ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
        ]),
    ),
    ("isParticipating", wire::Schema::Bool),
    ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
]));
static S_SORT_THREADS_BY_ACTIVITY_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("threadId", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("replyCount", wire::Schema::Num),
    (
        "lastReply",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("senderId", wire::Schema::Str16),
            ("senderName", wire::Schema::Str16),
            ("text", wire::Schema::Str16),
            ("sentAt", wire::Schema::Num),
        ])),
    ),
    (
        "unread",
        wire::Schema::Obj(&[
            ("unreadCount", wire::Schema::Num),
            ("mentionCount", wire::Schema::Num),
            ("isMarkedUnread", wire::Schema::OptJs(&wire::Schema::Bool)),
        ]),
    ),
    ("isParticipating", wire::Schema::Bool),
    ("isMuted", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("isResolved", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("lastOwnReplyAt", wire::Schema::OptJs(&wire::Schema::Num)),
]));

/// `sortThreadsByActivity` over the binary wire.
#[napi(js_name = "sortThreadsByActivityWire")]
pub fn sort_threads_by_activity_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let threads: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_SORT_THREADS_BY_ACTIVITY_0.inner()))?;
        let out = &port::sort_threads_by_activity(threads)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SORT_THREADS_BY_ACTIVITY_OUT.inner())?;
        }
        Ok(())
    })
}

static S_READ_THREAD_PREFS_0: wire::Schema = wire::Schema::Dyn;
static S_READ_THREAD_PREFS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("threadId", wire::Schema::Str16),
    ("pref", wire::Schema::Str16),
]));

/// `readThreadPrefs` over the binary wire.
#[napi(js_name = "readThreadPrefsWire")]
pub fn read_thread_prefs_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let content: serde_json::Value = wire::read_value(r, &S_READ_THREAD_PREFS_0)?;
        let out = &port::read_thread_prefs(content)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_READ_THREAD_PREFS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_WITH_THREAD_PREF_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("threadId", wire::Schema::Str16),
    ("pref", wire::Schema::Str16),
]));
static S_WITH_THREAD_PREF_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("roomId", wire::Schema::Str16),
    ("threadId", wire::Schema::Str16),
    ("pref", wire::Schema::Str16),
]));

/// `withThreadPref` over the binary wire.
#[napi(js_name = "withThreadPrefWire")]
pub fn with_thread_pref_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let entries: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_WITH_THREAD_PREF_0.inner()))?;
        let room_id: String = r.string16()?;
        let thread_id: String = r.string16()?;
        let pref: String = r.string16()?;
        let out = &port::with_thread_pref(entries, room_id, thread_id, pref)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_WITH_THREAD_PREF_OUT.inner())?;
        }
        Ok(())
    })
}

static S_RESOLVE_THREAD_FOLLOW_OUT: wire::Schema = wire::Schema::Obj(&[
    ("isFollowing", wire::Schema::Bool),
    ("isMuted", wire::Schema::Bool),
]);

/// `resolveThreadFollow` over the binary wire.
#[napi(js_name = "resolveThreadFollowWire")]
pub fn resolve_thread_follow_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let is_participating: bool = r.bool()?;
        let pref: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::resolve_thread_follow(is_participating, pref)?;
        wire::write_value(w, out, &S_RESOLVE_THREAD_FOLLOW_OUT)?;
        Ok(())
    })
}

static S_TIMELINE_WINDOW_OF_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("sentAt", wire::Schema::Num),
]));
static S_TIMELINE_WINDOW_OF_1: wire::Schema = wire::Schema::OptJs(&wire::Schema::List(
    &wire::Schema::Obj(&[("at", wire::Schema::Num)]),
));
static S_TIMELINE_WINDOW_OF_2: wire::Schema = wire::Schema::Dyn;
static S_TIMELINE_WINDOW_OF_3: wire::Schema = wire::Schema::Obj(&[
    (
        "ids",
        wire::Schema::List(&wire::Schema::OptJs(&wire::Schema::Str16)),
    ),
    ("since", wire::Schema::OptJs(&wire::Schema::Num)),
]);
static S_TIMELINE_WINDOW_OF_OUT: wire::Schema = wire::Schema::Obj(&[
    ("messages", wire::Schema::List(&wire::Schema::Dyn)),
    (
        "lines",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
    ),
    ("hiddenAbove", wire::Schema::Num),
    ("start", wire::Schema::Dyn),
]);

/// `timelineWindowOf` over the binary wire.
#[napi(js_name = "timelineWindowOfWire")]
pub fn timeline_window_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let messages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_TIMELINE_WINDOW_OF_0.inner()))?;
        let lines: Option<Vec<serde_json::Value>> =
            r.opt_js(|r| r.list(|r| wire::read_value(r, S_TIMELINE_WINDOW_OF_1.inner().inner())))?;
        let start: serde_json::Value = wire::read_value(r, &S_TIMELINE_WINDOW_OF_2)?;
        let reach: serde_json::Value = wire::read_value(r, &S_TIMELINE_WINDOW_OF_3)?;
        let out = &port::timeline_window_of(messages, lines, start, reach)?;
        wire::write_value(w, out, &S_TIMELINE_WINDOW_OF_OUT)?;
        Ok(())
    })
}

static S_OLDER_START_OF_0: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[("id", wire::Schema::Str16)]));
static S_OLDER_START_OF_OUT: wire::Schema = wire::Schema::Dyn;

/// `olderStartOf` over the binary wire.
#[napi(js_name = "olderStartOfWire")]
pub fn older_start_of_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let messages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_OLDER_START_OF_0.inner()))?;
        let hidden_above: f64 = r.f64()?;
        let out = &port::older_start_of(messages, hidden_above)?;
        wire::write_value(w, out, &S_OLDER_START_OF_OUT)?;
        Ok(())
    })
}

static S_COMMON_SETTINGS_URL_FOR_0: wire::Schema =
    wire::Schema::Obj(&[("applicationUrl", wire::Schema::OptJs(&wire::Schema::Str16))]);

/// `commonSettingsUrlFor` over the binary wire.
#[napi(js_name = "commonSettingsUrlForWire")]
pub fn common_settings_url_for_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let settings: serde_json::Value = wire::read_value(r, &S_COMMON_SETTINGS_URL_FOR_0)?;
        let user_id: String = r.string16()?;
        let out = &port::common_settings_url_for(settings, user_id)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_FIND_WEB_LINKS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("index", wire::Schema::Num),
    ("url", wire::Schema::Str16),
]));

/// `findWebLinks` over the binary wire.
#[napi(js_name = "findWebLinksWire")]
pub fn find_web_links_wire(len: u32) -> napi::Result<f64> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let out = &port::find_web_links(text)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_FIND_WEB_LINKS_OUT.inner())?;
        }
        Ok(())
    })
}
