//! Calls (M5): who is in the call of a room through MatrixRTC (MSC4143), as the web's
//! `MatrixCalls` does with matrix-js-sdk 43: its membership state event, the token of
//! the LiveKit focus (MSC4195, `/sfu/get`), and, in an encrypted room, a media key per
//! device shared over encrypted to-device messages (`RTCEncryptionManager`). The media
//! go through twake-call (ADR-018).

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use base64::engine::DecodePaddingMode;
use base64::engine::general_purpose::{GeneralPurpose, GeneralPurposeConfig, STANDARD};
use matrix_sdk::deserialized_responses::{EncryptionInfo, RawAnySyncOrStrippedState};
use matrix_sdk::ruma::api::client::account::request_openid_token;
use matrix_sdk::ruma::api::client::delayed_events::{
    DelayParameters, delayed_state_event, update_delayed_event,
};
use matrix_sdk::ruma::events::{
    AnySyncStateEvent, AnySyncTimelineEvent, AnyToDeviceEvent, MessageLikeEventType, StateEventType,
};
use matrix_sdk::ruma::serde::Raw;
use matrix_sdk::ruma::{OwnedRoomId, RoomId, UserId};
use matrix_sdk::{Client, Room};
use matrix_sdk_base::crypto::CollectStrategy;
use serde_json::{Value, json};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::engine::{Emit, Event};
use crate::media::Picture;

/// The membership of a device in a call (matrix-js-sdk `EventType.GroupCallMemberPrefix`).
pub(crate) const CALL_MEMBER_TYPE: &str = "org.matrix.msc3401.call.member";
/// The ring of a call (MSC4075, `EventType.RTCNotification`), and its earlier name.
const RING_TYPES: [&str; 2] = [
    "org.matrix.msc4075.rtc.notification",
    "org.matrix.msc4075.call.notify",
];
/// The decline of a ring (MSC4310): sent as matrix-js-sdk names it, read under both.
const DECLINE_TYPES: [&str; 2] = ["org.matrix.msc4310.rtc.decline", "m.rtc.decline"];
/// A media key of an encrypted call (`EventType.CallEncryptionKeysPrefix`).
const KEYS_TYPE: &str = "io.element.call.encryption_keys";
/// How long a membership lasts unless renewed (matrix-js-sdk `DEFAULT_EXPIRE_DURATION`).
const EXPIRES_MS: u64 = 4 * 60 * 60 * 1000;
/// The membership ends this long after the app stops renewing it (MSC4140).
const LEAVE_DELAY: Duration = Duration::from_secs(8);
const LEAVE_RESTART: Duration = Duration::from_secs(5);
/// A new key is used this long after it is sent, so that the others have it.
const USE_KEY_DELAY: Duration = Duration::from_secs(1);
/// Someone joining a call whose key is younger than this gets it; else a new one is made.
const KEY_GRACE: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

/// One device of a user in the call (web `CallMember`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallMember {
    pub user_id: String,
    pub device_id: String,
    /// When it joined (ms).
    pub joined_at: u64,
    /// The LiveKit token service it announces (the first of `foci_preferred`).
    pub focus: Option<String>,
}

/// Where a call stands for the window.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CallStatus {
    Connecting,
    Joined {
        encrypted: bool,
    },
    /// The web's `CallError` code (`no_focus`, `token_refused`…).
    Failed(String),
    /// Whoever was rung declined (MSC4310).
    Declined,
    Left,
}

/// A person of the call grid (web `CallTile`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallTile {
    /// `<user>:<device>`, as LiveKit knows them.
    pub identity: String,
    pub user_id: String,
    pub is_local: bool,
    /// The screen someone shares (web `isScreen`).
    pub is_screen: bool,
    pub has_video: bool,
    pub is_muted: bool,
    pub is_speaking: bool,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
        .unwrap_or(0)
}

/// The membership is one of the call of the room: the `m.call` application, no other
/// call id (web `isRoomCallMembership`).
fn is_room_call(content: &Value) -> bool {
    content.get("application").is_none_or(|a| a == "m.call")
        && content.get("call_id").is_none_or(|c| c == "")
}

/// The device a membership event announces in the call now, if any (web
/// `liveMemberEvents`): a device id, the call of the room, not expired.
pub(crate) fn live_member(event: &Value, now: u64) -> Option<CallMember> {
    let content = event.get("content")?;
    let user_id = event.get("sender")?.as_str()?;
    let device_id = content.get("device_id")?.as_str()?;
    if !is_room_call(content) {
        return None;
    }
    let sent = event.get("origin_server_ts").and_then(Value::as_u64)?;
    let joined_at = content
        .get("created_ts")
        .and_then(Value::as_u64)
        .unwrap_or(sent);
    let expires = content.get("expires").and_then(Value::as_u64).unwrap_or(0);
    if expires > 0 && joined_at + expires <= now {
        return None;
    }
    let focus = content
        .get("foci_preferred")
        .and_then(|f| f.get(0))
        .and_then(|f| f.get("livekit_service_url"))
        .and_then(Value::as_str)
        .map(str::to_string);
    Some(CallMember {
        user_id: user_id.to_string(),
        device_id: device_id.to_string(),
        joined_at,
        focus,
    })
}

/// The membership events of the calls of the room.
async fn member_events(room: &Room) -> Vec<Value> {
    room.get_state_events(StateEventType::from(CALL_MEMBER_TYPE))
        .await
        .unwrap_or_default()
        .into_iter()
        .filter_map(|raw| match raw {
            RawAnySyncOrStrippedState::Sync(raw) => raw.deserialize_as::<Value>().ok(),
            RawAnySyncOrStrippedState::Stripped(_) => None,
        })
        .collect()
}

fn live_members(events: &[Value]) -> Vec<CallMember> {
    let now = now_ms();
    let mut members: Vec<CallMember> = events.iter().filter_map(|e| live_member(e, now)).collect();
    members.sort_by_key(|m| m.joined_at);
    members
}

/// The devices in the call of the room now, the user's included, oldest first.
pub async fn members(room: &Room) -> Vec<CallMember> {
    live_members(&member_events(room).await)
}

/// When the user last left the call of the room, from any device: a left device keeps
/// an empty membership, stamped when it left (web `fetchLeftAt`).
fn left_at(events: &[Value], own: &str) -> Option<u64> {
    events
        .iter()
        .filter(|e| e["sender"] == own && e["content"].get("device_id").is_none())
        .filter_map(|e| e["origin_server_ts"].as_u64())
        .max()
}

/// The users in the call (one per person, not per device: web `countCallPeople`).
fn people(members: &[CallMember]) -> Vec<String> {
    let mut seen = HashSet::new();
    members
        .iter()
        .filter(|m| seen.insert(m.user_id.clone()))
        .map(|m| m.user_id.clone())
        .collect()
}

/// A call that rings for the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingCall {
    pub room_id: String,
    /// Who rang, or the first other member when no ring came.
    pub caller_id: String,
    pub is_video: bool,
    pub is_direct: bool,
}

/// A ring someone else sent (MSC4075).
struct HeardRing {
    event_id: String,
    /// When it ends (ms).
    until: f64,
    sender: String,
    is_video: bool,
}

/// A ring as the web reads it (`readRing`): its end, capped; None for anything else.
fn read_ring(event: &Value) -> Option<HeardRing> {
    let content = event.get("content")?;
    let kind = content
        .get("notification_type")
        .or_else(|| content.get("notify_type"))?;
    if kind != "ring" {
        return None;
    }
    let sent = event.get("origin_server_ts")?.as_f64()?;
    // Written by the caller: a date to come would ring for ever
    let at = content
        .get("sender_ts")
        .and_then(Value::as_f64)
        .map_or(sent, |s| s.min(sent));
    let max = twake_domain::call::MAX_RING_MS as f64;
    let lifetime = content
        .get("lifetime")
        .and_then(Value::as_f64)
        .unwrap_or(max);
    Some(HeardRing {
        event_id: event.get("event_id")?.as_str()?.to_string(),
        until: at + lifetime.min(max),
        sender: event.get("sender")?.as_str()?.to_string(),
        is_video: content.get("m.call.intent").is_some_and(|i| i == "video"),
    })
}

/// The ring a decline answers.
fn declined_ring(event: &Value) -> Option<String> {
    Some(
        event
            .pointer("/content/m.relates_to/event_id")?
            .as_str()?
            .to_string(),
    )
}

fn event_type(raw: &Raw<AnySyncTimelineEvent>) -> Option<String> {
    raw.get_field::<String>("type").ok().flatten()
}

/// Reports who is in the call of each room and the calls that ring for the user, now
/// and on each change; sends the user's declines (`declines`: the rooms).
pub async fn watch_calls(
    client: Client,
    emit: Emit,
    mut declines: mpsc::UnboundedReceiver<String>,
) {
    let own = client.user_id().map(|u| u.to_string()).unwrap_or_default();
    let (changed, mut news) = mpsc::unbounded_channel::<(OwnedRoomId, Option<Value>)>();
    let state_news = changed.clone();
    client.add_event_handler(move |raw: Raw<AnySyncStateEvent>, room: Room| {
        let changed = state_news.clone();
        async move {
            if raw.get_field::<String>("type").ok().flatten().as_deref() == Some(CALL_MEMBER_TYPE) {
                let _ = changed.send((room.room_id().to_owned(), None));
            }
        }
    });
    client.add_event_handler(move |raw: Raw<AnySyncTimelineEvent>, room: Room| {
        let changed = changed.clone();
        async move {
            let kind = event_type(&raw).unwrap_or_default();
            if RING_TYPES.contains(&kind.as_str()) || DECLINE_TYPES.contains(&kind.as_str()) {
                let event = raw.deserialize_as::<Value>().ok();
                let _ = changed.send((room.room_id().to_owned(), event));
            }
        }
    });
    let mut watcher = Watcher {
        client: client.clone(),
        own,
        rings: HashMap::new(),
        declined: HashSet::new(),
        active: HashSet::new(),
        ringing: Vec::new(),
        emit,
    };
    for room in client.joined_rooms() {
        watcher.members_changed(&room).await;
    }
    watcher.report_ringing().await;
    // A ring ends with no event to say so: looked at again while calls go on
    let mut tick = tokio::time::interval(Duration::from_secs(5));
    loop {
        tokio::select! {
            Some((room_id, event)) = news.recv() => {
                if let Some(event) = event {
                    watcher.heard(&room_id, &event);
                }
                if let Some(room) = client.get_room(&room_id) {
                    watcher.members_changed(&room).await;
                }
                watcher.report_ringing().await;
            }
            Some(room_id) = declines.recv() => {
                watcher.decline(&room_id).await;
                watcher.report_ringing().await;
            }
            _ = tick.tick() => {
                if !watcher.active.is_empty() {
                    watcher.report_ringing().await;
                }
            }
            else => break,
        }
    }
}

struct Watcher {
    client: Client,
    own: String,
    /// The last ring of each room.
    rings: HashMap<String, HeardRing>,
    declined: HashSet<String>,
    /// The rooms where a call goes on.
    active: HashSet<String>,
    ringing: Vec<IncomingCall>,
    emit: Emit,
}

impl Watcher {
    fn heard(&mut self, room_id: &RoomId, event: &Value) {
        let kind = event["type"].as_str().unwrap_or_default();
        if DECLINE_TYPES.contains(&kind) {
            // Declined from another device of the user
            if event["sender"] == self.own.as_str()
                && let Some(ring) = declined_ring(event)
            {
                self.declined.insert(ring);
            }
            return;
        }
        if event["sender"] == self.own.as_str() {
            return;
        }
        if let Some(ring) = read_ring(event).filter(|r| r.until > now_ms() as f64) {
            let room = room_id.to_string();
            if self
                .rings
                .get(&room)
                .is_none_or(|known| known.until < ring.until)
            {
                self.rings.insert(room, ring);
            }
        }
    }

    async fn members_changed(&mut self, room: &Room) {
        let people = people(&members(room).await);
        let room_id = room.room_id().to_string();
        if people.is_empty() {
            if !self.active.remove(&room_id) {
                return;
            }
        } else {
            self.active.insert(room_id.clone());
        }
        (self.emit)(Event::CallMembers {
            room_id,
            user_ids: people,
        });
    }

    /// The calls that ring for the user now (web `fetchIncomingCalls`).
    async fn report_ringing(&mut self) {
        use twake_domain::call::{Ring, RingMember, is_ringing, ring_asks};
        let now = now_ms() as f64;
        self.rings.retain(|_, r| r.until > now);
        let mut ringing = Vec::new();
        for room_id in &self.active {
            let Some(room) = RoomId::parse(room_id)
                .ok()
                .and_then(|id| self.client.get_room(&id))
            else {
                continue;
            };
            let events = member_events(&room).await;
            let present = live_members(&events);
            let members: Vec<RingMember> = present
                .iter()
                .map(|m| RingMember {
                    user_id: m.user_id.clone(),
                    joined_at: Some(m.joined_at as f64),
                })
                .collect();
            let ring = self.rings.get(room_id);
            let is_direct = room.is_direct().await.unwrap_or(false);
            let asks = ring_asks(
                ring.map(|r| Ring {
                    is_declined: self.declined.contains(&r.event_id),
                }),
                &members,
                &self.own,
                is_direct,
                now,
            );
            let left = left_at(&events, &self.own).map(|t| t as f64);
            if is_ringing(&members, &self.own, asks, false, left) {
                let caller_id = ring.map(|r| r.sender.clone()).or_else(|| {
                    present
                        .iter()
                        .find(|m| m.user_id != self.own)
                        .map(|m| m.user_id.clone())
                });
                ringing.push(IncomingCall {
                    room_id: room_id.clone(),
                    caller_id: caller_id.unwrap_or_default(),
                    is_video: ring.is_some_and(|r| r.is_video),
                    is_direct,
                });
            }
        }
        ringing.sort_by(|a, b| a.room_id.cmp(&b.room_id));
        if ringing != self.ringing {
            self.ringing = ringing.clone();
            (self.emit)(Event::Ringing(ringing));
        }
    }

    /// Tells whoever rang that the user declines (MSC4310), in the clear as the ring.
    async fn decline(&mut self, room_id: &str) {
        let Some(ring) = self.rings.get(room_id) else {
            return;
        };
        self.declined.insert(ring.event_id.clone());
        let content =
            json!({"m.relates_to": {"rel_type": "m.reference", "event_id": ring.event_id}});
        if let Ok(id) = RoomId::parse(room_id) {
            send_in_clear(&self.client, &id, DECLINE_TYPES[0], content).await;
        }
    }
}

/// Sends an event that stays readable in an encrypted room: the push rules of rings
/// match its type, and the push service has no keys (web `#sendInClear`, D108).
async fn send_in_clear(
    client: &Client,
    room_id: &RoomId,
    kind: &str,
    content: Value,
) -> Option<String> {
    use matrix_sdk::ruma::TransactionId;
    use matrix_sdk::ruma::api::client::message::send_message_event;
    let request = send_message_event::v3::Request::new_raw(
        room_id.to_owned(),
        TransactionId::new(),
        kind.into(),
        Raw::from_json_string(content.to_string()).ok()?,
    );
    match client.send(request).await {
        Ok(response) => Some(response.event_id.to_string()),
        Err(e) => {
            tracing::info!("{kind} not sent: {e}");
            None
        }
    }
}

/// The token service of the homeserver (the well-known's LiveKit focus), as the web reads it.
async fn own_focus(client: &Client) -> Option<String> {
    use matrix_sdk::ruma::api::client::rtc::RtcTransport;
    let transports = client.well_known_rtc_transports().await.ok()?;
    transports.into_iter().find_map(|t| match t {
        RtcTransport::LiveKit(info) => Some(info.service_url.to_string()),
        _ => None,
    })
}

/// The LiveKit server and a token for this room, from the token service (web
/// `fetchCallServer`).
async fn call_server(
    client: &Client,
    room_id: &RoomId,
    focus: &str,
    user: &UserId,
    device: &str,
) -> Result<(String, String), String> {
    let openid = client
        .send(request_openid_token::v3::Request::new(user.to_owned()))
        .await
        .map_err(|_| "token_failed".to_string())?;
    let body = json!({
        "room": room_id,
        "openid_token": {
            "access_token": openid.access_token,
            "token_type": "Bearer",
            "matrix_server_name": openid.matrix_server_name,
            "expires_in": openid.expires_in.as_secs(),
        },
        "device_id": device,
    });
    let http = matrix_sdk::reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|_| "connection_failed".to_string())?;
    let response = http
        .post(format!("{}/sfu/get", focus.trim_end_matches('/')))
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .map_err(|_| "connection_failed".to_string())?;
    if !response.status().is_success() {
        tracing::info!("call server: HTTP {}", response.status());
        return Err("token_refused".into());
    }
    let answer: Value = response
        .bytes()
        .await
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .ok_or("token_bad_response")?;
    match (answer["url"].as_str(), answer["jwt"].as_str()) {
        (Some(url), Some(jwt)) => Ok((url.to_string(), jwt.to_string())),
        _ => Err("token_bad_response".into()),
    }
}

/// What the window asks of the call going on.
pub(crate) enum CallCommand {
    Microphone(bool),
    Camera(bool),
    Screen(bool),
    Leave,
}

/// The call this device is in.
pub(crate) struct CallHandle {
    commands: mpsc::UnboundedSender<CallCommand>,
    task: JoinHandle<()>,
}

impl CallHandle {
    pub fn send(&self, command: CallCommand) {
        let _ = self.commands.send(command);
    }

    /// Leaves and waits until the membership is gone.
    pub async fn leave(self) {
        self.send(CallCommand::Leave);
        let _ = self.task.await;
    }
}

pub(crate) fn start(
    client: Client,
    room_id: String,
    video: bool,
    thread_root: Option<String>,
    emit: Emit,
) -> CallHandle {
    let (commands, queue) = mpsc::unbounded_channel();
    let task = tokio::spawn(run(client, room_id, video, thread_root, emit, queue));
    CallHandle { commands, task }
}

/// The key this device encrypts its media with.
struct OwnKey {
    index: u8,
    key: Vec<u8>,
    made: Instant,
}

/// A call joined: its media, membership and keys.
struct Joined {
    client: Client,
    room: Room,
    media: twake_call::Media,
    state_key: String,
    own: (String, String),
    delay_id: Option<String>,
    encrypted: bool,
    key: Option<OwnKey>,
    /// The other devices in the call, as last seen.
    others: HashSet<(String, String)>,
    /// The call this device rang, when it started it (web `OutgoingCalls`).
    outgoing: Option<Outgoing>,
}

/// A call this user rang: what its line says when it ends.
struct Outgoing {
    ring_id: String,
    is_video: bool,
    started_at: u64,
    answered_at: Option<u64>,
}

async fn run(
    client: Client,
    room_id: String,
    video: bool,
    thread_root: Option<String>,
    emit: Emit,
    mut commands: mpsc::UnboundedReceiver<CallCommand>,
) {
    let status = |status| Event::CallStatus {
        room_id: room_id.clone(),
        status,
    };
    emit(status(CallStatus::Connecting));
    // Keys of the others may come as soon as the membership is out: heard from now on
    let (keys_sent, mut keys) = mpsc::unbounded_channel::<(String, Value)>();
    let keys_handler = client.add_event_handler(
        move |raw: Raw<AnyToDeviceEvent>, info: Option<EncryptionInfo>| {
            let keys_sent = keys_sent.clone();
            async move {
                // Only keys sent encrypted, by the sender they claim
                let Some(info) = info else { return };
                let Ok(event) = raw.deserialize_as::<Value>() else {
                    return;
                };
                if event["type"] == KEYS_TYPE && event["sender"] == info.sender.as_str() {
                    let _ = keys_sent.send((info.sender.to_string(), event["content"].clone()));
                }
            }
        },
    );
    let (members_changed, mut changes) = mpsc::unbounded_channel::<()>();
    let room_filter = room_id.clone();
    let members_handler =
        client.add_event_handler(move |raw: Raw<AnySyncStateEvent>, room: Room| {
            let members_changed = members_changed.clone();
            let mine = room.room_id().as_str() == room_filter;
            async move {
                if mine
                    && raw.get_field::<String>("type").ok().flatten().as_deref()
                        == Some(CALL_MEMBER_TYPE)
                {
                    let _ = members_changed.send(());
                }
            }
        });
    // The decline of this device's ring, by whoever was rung
    let (decline_heard, mut declines) = mpsc::unbounded_channel::<String>();
    let room_filter = room_id.clone();
    let declines_handler =
        client.add_event_handler(move |raw: Raw<AnySyncTimelineEvent>, room: Room| {
            let decline_heard = decline_heard.clone();
            let mine = room.room_id().as_str() == room_filter;
            async move {
                let kind = event_type(&raw).unwrap_or_default();
                if mine
                    && DECLINE_TYPES.contains(&kind.as_str())
                    && let Some(ring) = raw
                        .deserialize_as::<Value>()
                        .ok()
                        .as_ref()
                        .and_then(declined_ring)
                {
                    let _ = decline_heard.send(ring);
                }
            }
        });
    let (ended_sent, mut ended) = mpsc::unbounded_channel::<String>();
    let mut call = match join(&client, &room_id, video, thread_root, &emit, ended_sent).await {
        Ok(call) => call,
        Err(error) => {
            client.remove_event_handler(keys_handler);
            client.remove_event_handler(members_handler);
            client.remove_event_handler(declines_handler);
            emit(status(CallStatus::Failed(error)));
            return;
        }
    };
    emit(status(CallStatus::Joined {
        encrypted: call.encrypted,
    }));
    // The membership is renewed on its own: nothing of the call may delay it
    let renewal = call
        .delay_id
        .clone()
        .map(|id| tokio::spawn(renew_leave(client.clone(), id)));
    call.share_keys().await;
    if video && let Err(e) = call.media.set_camera(true).await {
        emit(Event::Error(format!("camera: {e}")));
    }
    // A screen chosen in the system's picker, and the end of a sharing the system stopped
    let (screen_picked, mut screens) = mpsc::unbounded_channel::<Option<twake_call::Screen>>();
    let (screen_stopped, mut screen_ends) = mpsc::unbounded_channel::<()>();
    let mut known_keys: HashMap<String, (u8, Vec<u8>)> = HashMap::new();
    // Memberships also end by expiry, which no event tells
    let mut recheck = tokio::time::interval(Duration::from_secs(30));
    loop {
        tokio::select! {
            command = commands.recv() => match command {
                Some(CallCommand::Microphone(on)) => call.media.set_microphone(on),
                Some(CallCommand::Camera(on)) => {
                    if let Err(e) = call.media.set_camera(on).await {
                        emit(Event::Error(format!("camera: {e}")));
                    }
                }
                Some(CallCommand::Screen(true)) => {
                    // The picker waits for the user: the call goes on meanwhile
                    let stopped = screen_stopped.clone();
                    let pick = call.media.screen_picker(std::sync::Arc::new(move || {
                        let _ = stopped.send(());
                    }));
                    let picked = screen_picked.clone();
                    tokio::spawn(async move {
                        let screen = tokio::task::spawn_blocking(pick).await.ok().flatten();
                        let _ = picked.send(screen);
                    });
                }
                Some(CallCommand::Screen(false)) => call.media.stop_screen().await,
                Some(CallCommand::Leave) | None => break,
            },
            Some(screen) = screens.recv() => match screen {
                Some(screen) => {
                    if let Err(e) = call.media.share_screen(screen).await {
                        emit(Event::Error(format!("screen: {e}")));
                    }
                }
                // Cancelled or refused: nothing is said (web), the tiles tell it is off
                None => call.media.stop_screen().await,
            },
            Some(()) = screen_ends.recv() => call.media.stop_screen().await,
            Some((sender, content)) = keys.recv() => {
                if let Some((identity, index, key)) = read_key(&sender, &content, &room_id) {
                    call.media.set_key(&identity, index, key.clone());
                    known_keys.insert(identity, (index, key));
                }
            }
            Some(()) = changes.recv() => call.members_changed().await,
            _ = recheck.tick() => call.members_changed().await,
            Some(ring) = declines.recv() => {
                if call.outgoing.as_ref().is_some_and(|o| o.ring_id == ring) {
                    emit(status(CallStatus::Declined));
                }
            }
            Some(reason) = ended.recv() => {
                tracing::info!("call ended by the server: {reason}");
                break;
            }
        }
    }
    if let Some(renewal) = renewal {
        renewal.abort();
    }
    client.remove_event_handler(keys_handler);
    client.remove_event_handler(members_handler);
    client.remove_event_handler(declines_handler);
    call.leave().await;
    emit(status(CallStatus::Left));
}

/// A key event of another device for this call: its identity, index and key.
fn read_key(sender: &str, content: &Value, room_id: &str) -> Option<(String, u8, Vec<u8>)> {
    if content.get("room_id")?.as_str()? != room_id {
        return None;
    }
    let keys = content.get("keys")?;
    let index = keys.get("index")?.as_u64()?;
    let key = decode_key(keys.get("key")?.as_str()?)?;
    let device = content.get("member")?.get("claimed_device_id")?.as_str()?;
    // The identity LiveKit knows the participant by (web `rtcBackendIdentity`)
    Some((format!("{sender}:{device}"), (index % 256) as u8, key))
}

fn decode_key(text: &str) -> Option<Vec<u8>> {
    const LENIENT: GeneralPurpose = GeneralPurpose::new(
        &base64::alphabet::STANDARD,
        GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent),
    );
    LENIENT.decode(text).ok().filter(|k| !k.is_empty())
}

async fn join(
    client: &Client,
    room_id: &str,
    video: bool,
    thread_root: Option<String>,
    emit: &Emit,
    ended: mpsc::UnboundedSender<String>,
) -> Result<Joined, String> {
    let room = RoomId::parse(room_id)
        .ok()
        .and_then(|id| client.get_room(&id))
        .ok_or("room_not_found")?;
    let user = client.user_id().ok_or("room_not_found")?.to_owned();
    let device = client.device_id().ok_or("room_not_found")?.to_string();
    // The call goes on the focus of its oldest member (`oldest_membership`), else ours
    let present = members(&room).await;
    let own_focus = own_focus(client).await;
    let focus = present
        .iter()
        .find_map(|m| m.focus.clone())
        .or(own_focus.clone())
        .ok_or("no_focus")?;
    let levels = room.power_levels().await.map_err(|_| "no_permission")?;
    if !levels.user_can_send_state(&user, StateEventType::from(CALL_MEMBER_TYPE)) {
        return Err("no_permission".into());
    }
    let (url, jwt) = call_server(client, room.room_id(), &focus, &user, &device).await?;
    let encrypted = room.encryption_state().is_encrypted();
    let emit_media = emit.clone();
    let on_event: twake_call::OnEvent = std::sync::Arc::new(move |event| match event {
        twake_call::MediaEvent::Tiles(tiles) => {
            emit_media(Event::CallTiles(tiles.into_iter().map(call_tile).collect()))
        }
        twake_call::MediaEvent::Frame { identity, frame } => emit_media(Event::CallFrame {
            identity,
            picture: Picture {
                width: frame.width,
                height: frame.height,
                rgba: frame.rgba,
            },
        }),
        twake_call::MediaEvent::Ended(reason) => {
            let _ = ended.send(reason);
        }
    });
    let media = twake_call::Media::connect(&url, &jwt, encrypted, true, on_event)
        .await
        .map_err(|e| {
            tracing::info!("call media: {e}");
            "connection_failed".to_string()
        })?;
    let state_key = format!("_{user}_{device}_m.call");
    let delay_id = delay_leave(client, room.room_id(), &state_key).await;
    let content = json!({
        "application": "m.call",
        "call_id": "",
        "scope": "m.room",
        "device_id": device,
        "membershipID": format!("{user}:{device}"),
        "expires": EXPIRES_MS,
        "m.call.intent": if video { "video" } else { "audio" },
        "focus_active": {"type": "livekit", "focus_selection": "oldest_membership"},
        "foci_preferred": [{"type": "livekit", "livekit_service_url": own_focus.unwrap_or(focus)}],
    });
    let membership = match room
        .send_state_event_raw(CALL_MEMBER_TYPE, &state_key, content)
        .await
    {
        Ok(response) => response.event_id,
        Err(e) => {
            tracing::info!("call membership: {e}");
            media.leave().await;
            return Err("no_permission".into());
        }
    };
    // Whoever starts the call rings everyone of the room (MSC4075); nothing is sent when
    // it already goes on (web `saveJoin`)
    let mut outgoing = None;
    if present.is_empty() {
        let started_at = now_ms();
        let ring = json!({
            "m.mentions": {"user_ids": [], "room": true},
            "notification_type": "ring",
            "m.relates_to": {"rel_type": "m.reference", "event_id": membership},
            "sender_ts": started_at,
            "lifetime": twake_domain::call::RING_MS,
            "m.call.intent": if video { "video" } else { "audio" },
        });
        if let Some(ring_id) = send_in_clear(client, room.room_id(), RING_TYPES[0], ring).await {
            outgoing = Some(Outgoing {
                ring_id,
                is_video: video,
                started_at,
                answered_at: None,
            });
        }
        // The chat of the call goes to a thread of its own, opened by a marked message
        // (web `saveCallThreadRoot`); a reader who may not post opens none
        if let Some(text) = thread_root
            && levels.user_can_send_message(&user, MessageLikeEventType::RoomMessage)
        {
            let root = json!({
                "msgtype": "m.text",
                "body": text,
                "m.mentions": {},
                (twake_domain::call::CALL_THREAD_KEY): {},
            });
            if let Err(e) = room.send_raw("m.room.message", root).await {
                tracing::info!("call thread root: {e}");
            }
        }
    }
    Ok(Joined {
        client: client.clone(),
        room,
        media,
        state_key,
        own: (user.to_string(), device),
        delay_id,
        encrypted,
        key: None,
        others: HashSet::new(),
        outgoing,
    })
}

fn call_tile(tile: twake_call::Tile) -> CallTile {
    let person = tile
        .identity
        .strip_suffix(twake_call::SCREEN_SUFFIX)
        .unwrap_or(&tile.identity);
    let user_id = person
        .rsplit_once(':')
        .map_or(person, |(user, _)| user)
        .to_string();
    CallTile {
        user_id,
        identity: tile.identity,
        is_local: tile.is_local,
        is_screen: tile.is_screen,
        has_video: tile.has_video,
        is_muted: tile.is_muted,
        is_speaking: tile.is_speaking,
    }
}

/// The membership that ends the call of this device unless renewed (MSC4140): the
/// homeserver sends it if the app dies. None when the homeserver has no delayed events.
async fn delay_leave(client: &Client, room_id: &RoomId, state_key: &str) -> Option<String> {
    let request = delayed_state_event::unstable::Request::new_raw(
        room_id.to_owned(),
        state_key.to_string(),
        StateEventType::from(CALL_MEMBER_TYPE),
        DelayParameters::Timeout {
            timeout: LEAVE_DELAY,
        },
        Raw::from_json_string("{}".into()).ok()?,
    );
    match client.send(request).await {
        Ok(response) => Some(response.delay_id),
        Err(e) => {
            tracing::info!("no delayed leave: {e}");
            None
        }
    }
}

/// Pushes back the delayed leave while the call goes on.
async fn renew_leave(client: Client, delay_id: String) {
    let mut every = tokio::time::interval(LEAVE_RESTART);
    loop {
        every.tick().await;
        let request = update_delayed_event::unstable_v1::Request::new(
            delay_id.clone(),
            update_delayed_event::UpdateAction::Restart,
        );
        if let Err(e) = client.send(request).await {
            tracing::debug!("delayed leave not renewed: {e}");
        }
    }
}

impl Joined {
    /// Who is in the call changed: someone else in it answers the ring (the first time
    /// counts), and the keys go to the devices that joined.
    async fn members_changed(&mut self) {
        if let Some(call) = self.outgoing.as_mut().filter(|c| c.answered_at.is_none())
            && members(&self.room)
                .await
                .iter()
                .any(|m| m.user_id != self.own.0)
        {
            call.answered_at = Some(now_ms());
        }
        self.share_keys().await;
    }

    /// Hands the key to the devices that joined, or a new one to all when a device
    /// left (or the key is old): `RTCEncryptionManager.ensureKeyDistribution`.
    async fn share_keys(&mut self) {
        if !self.encrypted {
            return;
        }
        let now: HashSet<(String, String)> = members(&self.room)
            .await
            .into_iter()
            .map(|m| (m.user_id, m.device_id))
            .filter(|d| *d != self.own)
            .collect();
        let joined: Vec<_> = now.difference(&self.others).cloned().collect();
        let left = self.others.difference(&now).count();
        self.others = now.clone();
        let fresh = self
            .key
            .as_ref()
            .is_some_and(|k| k.made.elapsed() < KEY_GRACE);
        match &self.key {
            Some(key) if left == 0 && (joined.is_empty() || fresh) => {
                if !joined.is_empty() {
                    let (index, key) = (key.index, key.key.clone());
                    self.send_key(index, &key, &joined).await;
                }
            }
            old => {
                let index = old.as_ref().map_or(0, |k| k.index.wrapping_add(1));
                let first = old.is_none();
                let key: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();
                let all: Vec<_> = now.into_iter().collect();
                self.send_key(index, &key, &all).await;
                // A new key is used once the others have it (not the first: nobody yet)
                if !first && !all.is_empty() {
                    tokio::time::sleep(USE_KEY_DELAY).await;
                }
                let identity = format!("{}:{}", self.own.0, self.own.1);
                self.media.set_key(&identity, index, key.clone());
                self.key = Some(OwnKey {
                    index,
                    key,
                    made: Instant::now(),
                });
            }
        }
    }

    async fn send_key(&self, index: u8, key: &[u8], to: &[(String, String)]) {
        if to.is_empty() {
            return;
        }
        let content = json!({
            "keys": {"index": index, "key": STANDARD.encode(key)},
            "room_id": self.room.room_id(),
            "member": {"claimed_device_id": self.own.1, "id": format!("{}:{}", self.own.0, self.own.1)},
            "session": {"call_id": "", "application": "m.call", "scope": "m.room"},
            "sent_ts": now_ms(),
        });
        let encryption = self.client.encryption();
        let mut devices = Vec::new();
        for (user, device) in to {
            let Ok(user) = UserId::parse(user.as_str()) else {
                continue;
            };
            let device = matrix_sdk::ruma::OwnedDeviceId::from(device.as_str());
            if let Ok(Some(found)) = encryption.get_device(&user, &device).await {
                devices.push(found);
            }
        }
        let Ok(raw) = Raw::from_json_string(content.to_string()) else {
            return;
        };
        match encryption
            .encrypt_and_send_raw_to_device(
                devices.iter().collect(),
                KEYS_TYPE,
                raw,
                CollectStrategy::AllDevices,
            )
            .await
        {
            Ok(failed) if !failed.is_empty() => tracing::info!("call key not sent to {failed:?}"),
            Ok(_) => {}
            Err(e) => tracing::warn!("call key not sent: {e}"),
        }
    }

    async fn leave(self) {
        let sent = match &self.delay_id {
            Some(delay_id) => self
                .client
                .send(update_delayed_event::unstable_v1::Request::new(
                    delay_id.clone(),
                    update_delayed_event::UpdateAction::Send,
                ))
                .await
                .is_ok(),
            None => false,
        };
        if !sent
            && let Err(e) = self
                .room
                .send_state_event_raw(CALL_MEMBER_TYPE, &self.state_key, json!({}))
                .await
        {
            tracing::warn!("call membership not ended: {e}");
        }
        self.media.leave().await;
        // The caller alone writes the line of the call, once, in the clear (D108)
        if let Some(call) = &self.outgoing {
            let mut line = json!({
                "intent": if call.is_video { "video" } else { "audio" },
                "started_ts": call.started_at,
                "ended_ts": now_ms(),
                "m.relates_to": {"rel_type": "m.reference", "event_id": call.ring_id},
            });
            if let Some(answered) = call.answered_at {
                line["answered_ts"] = json!(answered);
            }
            send_in_clear(
                &self.client,
                self.room.room_id(),
                twake_domain::call::CALL_LINE_TYPE,
                line,
            )
            .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(content: Value, ts: u64) -> Value {
        json!({"sender": "@a:x", "origin_server_ts": ts, "content": content})
    }

    #[test]
    fn reads_the_live_members_as_the_web() {
        let live = event(
            json!({"device_id": "D", "application": "m.call", "call_id": "",
            "expires": 1000, "foci_preferred": [{"type": "livekit", "livekit_service_url": "https://f"}]}),
            500,
        );
        let m = live_member(&live, 1000).unwrap();
        assert_eq!((m.device_id.as_str(), m.joined_at), ("D", 500));
        assert_eq!(m.focus.as_deref(), Some("https://f"));
        // Expired, left (no device), another call, the older form without application
        assert!(live_member(&live, 1500).is_none());
        assert!(live_member(&event(json!({}), 1), 2).is_none());
        assert!(live_member(&event(json!({"device_id": "D", "call_id": "x"}), 1), 2).is_none());
        let created = event(
            json!({"device_id": "D", "created_ts": 100, "expires": 1000}),
            900,
        );
        assert!(live_member(&created, 1200).is_none());
        assert!(live_member(&event(json!({"device_id": "D"}), 1), u64::MAX).is_some());
    }

    #[test]
    fn counts_people_not_devices() {
        let m = |u: &str, d: &str| CallMember {
            user_id: u.into(),
            device_id: d.into(),
            joined_at: 0,
            focus: None,
        };
        assert_eq!(
            people(&[m("@a:x", "1"), m("@b:x", "1"), m("@a:x", "2")]),
            ["@a:x", "@b:x"]
        );
    }

    #[test]
    fn reads_the_keys_of_the_others() {
        let content = json!({"room_id": "!r:x", "keys": {"index": 3, "key": "AAECAwQFBgcICQoLDA0ODw=="},
            "member": {"claimed_device_id": "D", "id": "@b:x:D"}});
        let (identity, index, key) = read_key("@b:x", &content, "!r:x").unwrap();
        assert_eq!((identity.as_str(), index, key.len()), ("@b:x:D", 3, 16));
        assert!(read_key("@b:x", &content, "!other:x").is_none());
        // Unpadded too
        assert_eq!(
            decode_key("AAECAwQFBgcICQoLDA0ODw").map(|k| k.len()),
            Some(16)
        );
    }
}
