//! The engine: one tokio runtime on its own thread, driven by commands.

use std::sync::Arc;

use futures_util::StreamExt;
use matrix_sdk::ruma::api::client::receipt::create_receipt::v3::ReceiptType;
use matrix_sdk::ruma::events::poll::{
    unstable_end::UnstablePollEndEventContent, unstable_response::UnstablePollResponseEventContent,
};
use matrix_sdk::{
    Client,
    encryption::{VerificationState, recovery::RecoveryState},
    room::edit::EditedContent,
    ruma::{
        OwnedEventId, OwnedRoomId, OwnedTransactionId,
        events::room::message::{RoomMessageEventContent, RoomMessageEventContentWithoutRelation},
    },
};
use matrix_sdk_ui::{
    sync_service::SyncService,
    timeline::{RoomExt, Timeline, TimelineEventItemId, TimelineFocus},
};
use tokio::{sync::mpsc, task::JoinHandle};
use twake_domain::{RoomSummary, TimelineEntry};

use crate::{rooms, session, timeline};

/// Messages fetched per back-pagination.
const HISTORY_PAGE: u16 = 40;

#[derive(Debug)]
pub enum Command {
    SignInPassword {
        homeserver: String,
        username: String,
        password: String,
    },
    SignInSso {
        homeserver: String,
    },
    OpenRoom {
        room_id: String,
    },
    LoadOlder,
    /// The newest message of the open room is on screen.
    MarkRead,
    /// The newest reply of the open thread is on screen: its own read receipt (MSC3771),
    /// so the room stops counting it unread.
    MarkThreadRead,
    /// The user is typing in the open room (or stopped).
    Typing {
        typing: bool,
    },
    /// Send `text` in the open room, or in its open thread when `thread`; as a reply to the
    /// message `reply_to` when given.
    Send {
        text: String,
        thread: bool,
        reply_to: Option<String>,
        /// The people and rooms the text names (`prepareOutgoing` keeps those still there).
        mentions: Vec<twake_domain::mention::Mention>,
        /// `/me`: sent as `m.emote`.
        emote: bool,
        /// The bot command the text runs (MSC4332).
        command: Option<twake_domain::composer::BotCommand>,
    },
    /// Replace the text of one's own message.
    Edit {
        id: String,
        text: String,
        thread: bool,
    },
    /// Delete a message for everyone.
    Delete {
        id: String,
        thread: bool,
        /// Why, shown to the members (web `ReasonDialog`).
        reason: Option<String>,
    },
    /// Add the reaction `key`, or take it back when already given.
    React {
        id: String,
        key: String,
        thread: bool,
    },
    /// Open the thread of the message `root_id` beside the conversation.
    OpenThread {
        root_id: String,
    },
    CloseThread,
    LoadOlderInThread,
    Recover {
        key: String,
    },
    SignOut,
    /// Decode the image of `key` (see [`crate::mxc_key`] and `Attachment::source`) at this
    /// size in pixels, cropped to fill it or scaled to fit.
    FetchImage {
        key: String,
        width: u32,
        height: u32,
        crop: bool,
    },
    /// Save an attachment to the Downloads folder.
    SaveAttachment {
        key: String,
        name: String,
    },
    /// Open an attachment with the system's application for it.
    OpenAttachment {
        key: String,
        name: String,
    },
    /// Send a file of the computer in the open room (or its thread).
    SendFile {
        path: std::path::PathBuf,
        thread: bool,
        reply_to: Option<String>,
        /// Text sent with the file (Markdown).
        caption: Option<String>,
    },
    /// A recorded voice message (Ogg Opus at `path`), sent to the room or the thread.
    SendVoice {
        path: std::path::PathBuf,
        thread: bool,
        reply_to: Option<String>,
        duration_ms: u64,
        /// Values from 0 to 1.
        waveform: Vec<f32>,
    },
    /// The edit history of a message of the open room ([`Event::MessageVersions`]).
    MessageVersions {
        event_id: String,
    },
    /// Send messages of the open room again to these rooms, in this order
    /// ([`Event::Forwarded`]).
    Forward {
        event_ids: Vec<String>,
        to_rooms: Vec<String>,
    },
    /// Delete a chat for the user alone up to `before_ts`, or bring it back with None
    /// ([`Event::DeletedChats`], D186).
    DeleteChat {
        room_id: String,
        before_ts: Option<f64>,
    },
    /// Save a message of the open room for later, or forget it ([`Event::Saved`]).
    SaveMessage {
        event_id: String,
        saved: bool,
    },
    /// The bytes of a file, for the audio player ([`Event::FileData`]).
    FetchFile {
        key: String,
    },
    /// Open the PDF of `key` in the viewer ([`Event::PdfOpened`] or [`Event::PdfFailed`]).
    OpenPdf {
        key: String,
    },
    /// Draw a page of the PDF open, `width` pixels wide ([`Event::PdfPage`]).
    RenderPdfPage {
        key: String,
        page: u16,
        width: u32,
    },
    /// The viewer closed: the PDF open is dropped.
    ClosePdf,
    /// Join the call of the room (start it when none goes on), with the camera on or
    /// not ([`Event::CallStatus`]). Leaves the call this device is in first.
    JoinCall {
        room_id: String,
        video: bool,
        /// The text of the message that opens the thread of the call (« Meeting started
        /// at … »), sent when this device starts the call; None in a direct room (D105).
        thread_root: Option<String>,
    },
    LeaveCall,
    /// Decline the call that rings in the room (MSC4310).
    DeclineCall {
        room_id: String,
    },
    /// Turn the microphone of the call on or off.
    CallMicrophone(bool),
    /// Turn the camera of the call on or off.
    CallCamera(bool),
    /// Share a screen (the system's picker chooses it) or stop sharing it.
    CallScreen(bool),
    /// The devices of these kinds, read now ([`Event::CallDevices`]).
    CallDevices(Vec<crate::calls::DeviceKind>),
    /// Use this device of the call from now on.
    CallSwitchDevice(crate::calls::DeviceKind, String),
    /// The frames of an animated image (GIF) of `key`, scaled to fit this size in pixels
    /// ([`Event::Animation`]).
    FetchAnimation {
        key: String,
        width: u32,
        height: u32,
    },
    /// Vote in a poll: the chosen answers (empty to take the vote back).
    Vote {
        poll_id: String,
        answers: Vec<String>,
        thread: bool,
    },
    EndPoll {
        poll_id: String,
        thread: bool,
    },
    CreatePoll {
        question: String,
        answers: Vec<String>,
        multiple: bool,
        thread: bool,
    },
    /// Pin or unpin messages of the open room, in one update of the pinned list.
    Pin {
        ids: Vec<String>,
        pinned: bool,
    },
    /// Open the pinned messages of the open room in the side panel ([`Event::Thread`] with
    /// the root id `pinned`).
    OpenPinned,
    /// Search the open room ([`Event::SearchResults`]); `older`: read one more page of its
    /// history first.
    Search {
        filter: twake_domain::search::SearchFilter,
        older: bool,
    },
    /// Open the messages around this one in the side panel ([`Event::Thread`] with
    /// `root_id` "around").
    OpenAround {
        event_id: String,
    },
    /// « Go to a date »: the first message of the open room from `ts` (epoch ms, the
    /// start of a local day; 0 is the beginning) on, or the last before it, asked of the
    /// server (`/timestamp_to_event`); [`Event::DateFound`] names it.
    JumpToDate {
        ts: i64,
    },
    /// What the homeserver says of a web link ([`Event::LinkPreview`]); never asked in an
    /// encrypted room (the caller's rule, web D211).
    FetchLinkPreview {
        url: String,
    },
    /// `/discardsession`: drop the outgoing encryption session of the open room, so the
    /// next message starts a new one (the assistant asks for it when it lacks the keys).
    DiscardSession,
    /// Load the details of a room for its info pane ([`Event::RoomInfo`]).
    RoomInfo {
        room_id: String,
    },
    /// Change a room or its members; the info pane is refreshed after.
    Room {
        room_id: String,
        action: RoomAction,
    },
    /// Create a room and invite people to it; opens it ([`Event::RoomCreated`]).
    CreateRoom {
        name: String,
        topic: String,
        invite: Vec<String>,
        encrypted: bool,
    },
    /// Open the direct message with someone, created when there is none (in the clear, as
    /// the web: encryption is a switch of the room).
    OpenDirect {
        user_id: String,
    },
}

#[derive(Clone, Debug)]
pub enum RoomAction {
    Invite(String),
    /// A member and the reason given, if any.
    Remove(String, Option<String>),
    Ban(String, Option<String>),
    SetRole(String, twake_domain::room_management::MemberRole),
    Rename(String),
    SetTopic(String),
    Encrypt,
    /// Accept an invitation.
    Join,
    /// Leave the room, or decline an invitation.
    Leave,
    /// Report the room to the server's administrators, with a reason (web `reportRoom`).
    Report(Option<String>),
    /// Report a message of the room, with a reason (web `reportMessage`).
    ReportMessage(String, Option<String>),
    /// Block a person: their messages no longer come (`m.ignored_user_list`, web
    /// `ignorePerson`).
    Block(String),
    /// Who may join the room (web `changeRoomAccess`, D516).
    SetJoinRule(twake_domain::room_management::JoinRuleChoice),
    /// List the room in the public directory, or take it off.
    SetListed(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SessionState {
    /// Looking for a saved session at start.
    Restoring,
    SignedOut,
    SigningIn,
    SignedIn {
        user_id: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct EncryptionStatus {
    /// This device is verified: encrypted history can be read.
    pub verified: bool,
    /// The account has recovery set up (on this device or another one): a recovery key
    /// can unlock the encrypted history here.
    pub recovery_available: bool,
}

#[derive(Clone, Debug)]
pub enum Event {
    Session(SessionState),
    Rooms(Vec<RoomSummary>),
    RoomOpened {
        room_id: String,
        name: String,
        is_encrypted: bool,
    },
    Timeline {
        room_id: String,
        entries: Vec<TimelineEntry>,
        /// Messages pinned in the room.
        pinned_count: usize,
    },
    /// The open thread: its root first, then the replies.
    Thread {
        room_id: String,
        root_id: String,
        entries: Vec<TimelineEntry>,
    },
    /// Back-pagination state of the open room.
    History {
        loading: bool,
        at_start: bool,
    },
    Encryption(EncryptionStatus),
    Image {
        key: String,
        width: u32,
        height: u32,
        picture: crate::media::Picture,
    },
    ImageFailed {
        key: String,
        width: u32,
        height: u32,
    },
    RoomInfo(twake_domain::RoomDetails),
    /// The messages of the open room a search found, newest first. `is_complete`: the
    /// history is read to its start, so nothing is missing.
    SearchResults {
        room_id: String,
        query: String,
        results: Vec<twake_domain::search::IndexedMessage>,
        is_complete: bool,
    },
    /// The saved messages, newest first, after each change.
    Saved(Vec<SavedEntry>),
    /// The versions of a message, oldest first; empty when it could not be read.
    MessageVersions {
        event_id: String,
        versions: Vec<crate::versions::MessageVersion>,
    },
    /// A message went to these rooms ([`Command::Forward`]).
    Forwarded {
        to_rooms: Vec<String>,
    },
    /// The chats deleted for the user alone, after each change (D186).
    DeletedChats(twake_domain::deleted_chats::DeletedChats),
    /// The bytes of a file asked with [`Command::FetchFile`]; None when it failed.
    FileData {
        key: String,
        bytes: Option<Arc<Vec<u8>>>,
    },
    /// The users in the call of a room now (MatrixRTC), one per person; empty: no call.
    CallMembers {
        room_id: String,
        user_ids: Vec<String>,
    },
    /// The calls that ring for the user now (empty: none).
    Ringing(Vec<crate::calls::IncomingCall>),
    /// Where the call of this device stands.
    CallStatus {
        room_id: String,
        status: crate::calls::CallStatus,
    },
    /// The people of the call grid, the user first.
    CallTiles(Vec<crate::calls::CallTile>),
    /// The devices of a device menu ([`Command::CallDevices`]).
    CallDevices(Vec<crate::calls::CallDeviceList>),
    /// A new picture of the video of `identity` (a [`crate::calls::CallTile`]).
    CallFrame {
        identity: String,
        picture: crate::media::Picture,
    },
    /// The PDF of a [`Command::OpenPdf`] is open: the size of each page, in points.
    PdfOpened {
        key: String,
        pages: Vec<(f32, f32)>,
    },
    PdfFailed {
        key: String,
        error: String,
    },
    /// A page drawn for [`Command::RenderPdfPage`].
    PdfPage {
        key: String,
        page: u16,
        width: u32,
        picture: crate::media::Picture,
    },
    /// The frames of a [`Command::FetchAnimation`] and how long each shows, in ms; empty
    /// when the image does not move, is too big to keep, or failed.
    Animation {
        key: String,
        width: u32,
        height: u32,
        frames: Vec<(crate::media::Picture, u32)>,
    },
    /// Whether animated images play by themselves (account data `app.twake.chat.autoplay`,
    /// shared with the web); None when the user never chose.
    Autoplay(Option<bool>),
    /// The preview of a web link; None when the homeserver found nothing to show.
    LinkPreview {
        url: String,
        preview: Option<twake_domain::link_preview::LinkPreview>,
    },
    /// The message a [`Command::JumpToDate`] reached, in the room `room_id`.
    DateFound {
        room_id: String,
        event_id: String,
    },
    /// Mentions, reactions on the user's messages and followed threads, after each change.
    Activity(twake_domain::activity::Activity),
    /// What the composer of the open room offers: its members to mention, whether the
    /// user may notify everyone, and the commands of its bots.
    Composer {
        room_id: String,
        members: Vec<twake_domain::room_management::RoomMemberDetail>,
        can_mention_everyone: bool,
        commands: Vec<twake_domain::composer::BotCommand>,
    },
    /// The top-level spaces with their rooms.
    Spaces(Vec<twake_domain::SpaceSummary>),
    /// The people typing in the open room, by name (the user excluded).
    Typing {
        room_id: String,
        names: Vec<String>,
    },
    /// A room created or a direct message found: open it.
    RoomCreated {
        room_id: String,
    },
    /// The user left the room (or declined it).
    RoomLeft {
        room_id: String,
    },
    /// A message that notifies (push rules), for an OS notification.
    Notify {
        room_id: String,
        title: String,
        body: String,
    },
    Notice(String),
    Error(String),
}

/// A saved message: its ids, when it was saved, and what this device read of it (sender
/// and text, from the search index), None when it is not in the history held here.
#[derive(Clone, Debug)]
pub struct SavedEntry {
    pub room_id: String,
    pub event_id: String,
    pub saved_at: f64,
    pub quote: Option<(String, String)>,
}

pub(crate) type Emit = Arc<dyn Fn(Event) + Send + Sync>;

pub struct Engine {
    commands: mpsc::UnboundedSender<Command>,
}

impl Engine {
    /// Starts the engine; `on_event` is called from the engine's threads.
    pub fn start(on_event: impl Fn(Event) + Send + Sync + 'static) -> Engine {
        let (commands, receiver) = mpsc::unbounded_channel();
        let emit: Emit = Arc::new(on_event);
        std::thread::Builder::new()
            .name("twake-engine".into())
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(2)
                    .enable_all()
                    .build();
                match runtime {
                    Ok(runtime) => runtime.block_on(run(receiver, emit)),
                    Err(e) => emit(Event::Error(format!("cannot start: {e}"))),
                }
            })
            .ok();
        Engine { commands }
    }

    pub fn send(&self, command: Command) {
        let _ = self.commands.send(command);
    }
}

#[derive(Default)]
struct State {
    client: Option<Client>,
    images: crate::media::InFlight,
    sync: Option<SyncService>,
    tasks: Vec<JoinHandle<()>>,
    open: Option<OpenRoom>,
    /// The local search index of the session.
    index: Option<Arc<crate::search_index::SearchIndex>>,
    /// The call this device is in.
    call: Option<crate::calls::CallHandle>,
    /// The rooms whose ring the user declines, for the call watcher.
    declines: Option<mpsc::UnboundedSender<String>>,
}

struct OpenRoom {
    timeline: Arc<Timeline>,
    task: JoinHandle<()>,
    /// Follows who is typing in the room.
    typing: JoinHandle<()>,
    thread: Option<OpenThread>,
    /// The history of the room is read to its start: a search misses nothing.
    complete: Arc<std::sync::atomic::AtomicBool>,
}

struct OpenThread {
    timeline: Arc<Timeline>,
    task: JoinHandle<()>,
}

impl Drop for OpenRoom {
    fn drop(&mut self) {
        self.task.abort();
        self.typing.abort();
    }
}

impl Drop for OpenThread {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl State {
    fn room(&self, room_id: &str) -> Option<matrix_sdk::Room> {
        let id = OwnedRoomId::try_from(room_id).ok()?;
        self.client.as_ref()?.get_room(&id)
    }

    /// The timeline a command acts on: the open thread's, or the room's.
    fn pane(&self, thread: bool) -> Option<Arc<Timeline>> {
        let open = self.open.as_ref()?;
        if thread {
            open.thread.as_ref().map(|t| t.timeline.clone())
        } else {
            Some(open.timeline.clone())
        }
    }
}

/// The timeline id of a message: an event id (`$...`) or the transaction id of a local echo.
fn item_id(id: &str) -> Option<TimelineEventItemId> {
    if id.starts_with('$') {
        OwnedEventId::try_from(id)
            .ok()
            .map(TimelineEventItemId::EventId)
    } else if id.is_empty() {
        None
    } else {
        Some(TimelineEventItemId::TransactionId(
            OwnedTransactionId::from(id),
        ))
    }
}

async fn run(mut commands: mpsc::UnboundedReceiver<Command>, emit: Emit) {
    let mut state = State::default();
    emit(Event::Session(SessionState::Restoring));
    match session::restore().await {
        Some(Ok((client, _))) => start_session(&mut state, client, &emit).await,
        Some(Err(e)) => {
            emit(Event::Error(e));
            emit(Event::Session(SessionState::SignedOut));
        }
        None => emit(Event::Session(SessionState::SignedOut)),
    }

    while let Some(command) = commands.recv().await {
        match command {
            Command::SignInPassword {
                homeserver,
                username,
                password,
            } => {
                emit(Event::Session(SessionState::SigningIn));
                match session::sign_in_password(&homeserver, &username, &password).await {
                    Ok(client) => start_session(&mut state, client, &emit).await,
                    Err(e) => {
                        emit(Event::Error(e));
                        emit(Event::Session(SessionState::SignedOut));
                    }
                }
            }
            Command::SignInSso { homeserver } => {
                emit(Event::Session(SessionState::SigningIn));
                match session::sign_in_sso(&homeserver).await {
                    Ok(client) => start_session(&mut state, client, &emit).await,
                    Err(e) => {
                        emit(Event::Error(e));
                        emit(Event::Session(SessionState::SignedOut));
                    }
                }
            }
            Command::OpenRoom { room_id } => open_room(&mut state, &room_id, &emit).await,
            Command::LoadOlder => {
                if let Some(open) = &state.open {
                    let timeline = open.timeline.clone();
                    let complete = open.complete.clone();
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        emit(Event::History {
                            loading: true,
                            at_start: false,
                        });
                        let at_start = timeline
                            .paginate_backwards(HISTORY_PAGE)
                            .await
                            .unwrap_or(false);
                        complete.fetch_or(at_start, std::sync::atomic::Ordering::Relaxed);
                        emit(Event::History {
                            loading: false,
                            at_start,
                        });
                    });
                }
            }
            Command::Search { filter, older } => {
                if let (Some(open), Some(index)) = (&state.open, &state.index) {
                    tokio::spawn(search(
                        open.timeline.clone(),
                        open.complete.clone(),
                        index.clone(),
                        filter,
                        older,
                        emit.clone(),
                    ));
                }
            }
            Command::OpenAround { event_id } => {
                open_side(&mut state, Side::Around(event_id), &emit).await
            }
            Command::FetchLinkPreview { url } => {
                if let Some(client) = state.client.clone() {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let raw = client.media().get_media_preview(&url, None).await;
                        let preview = raw
                            .ok()
                            .flatten()
                            .and_then(|r| serde_json::from_str(r.get()).ok())
                            .and_then(|v: serde_json::Value| {
                                twake_domain::link_preview::read_link_preview(&v)
                            });
                        emit(Event::LinkPreview { url, preview });
                    });
                }
            }
            Command::JumpToDate { ts } => {
                if let Some(open) = &state.open {
                    let room = open.timeline.room().clone();
                    let client = state.client.clone();
                    let emit = emit.clone();
                    tokio::spawn(async move { jump_to_date(client, room, ts, emit).await });
                }
            }
            Command::Typing { typing } => {
                if let Some(open) = &state.open {
                    let room = open.timeline.room().clone();
                    tokio::spawn(async move {
                        let _ = room.typing_notice(typing).await;
                    });
                }
            }
            Command::MarkRead => {
                if let Some(open) = &state.open {
                    let timeline = open.timeline.clone();
                    tokio::spawn(async move {
                        let _ = timeline.mark_as_read(ReceiptType::Read).await;
                    });
                }
            }
            Command::MarkThreadRead => {
                if let Some(thread) = state.open.as_ref().and_then(|o| o.thread.as_ref())
                    && thread.timeline.is_threaded()
                {
                    let timeline = thread.timeline.clone();
                    tokio::spawn(async move {
                        let _ = timeline.mark_as_read(ReceiptType::Read).await;
                    });
                }
            }
            Command::Edit { id, text, thread } => {
                let text = text.trim().to_string();
                if let (Some(timeline), Some(item), false) =
                    (state.pane(thread), item_id(&id), text.is_empty())
                {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let content = RoomMessageEventContentWithoutRelation::text_markdown(text);
                        if let Err(e) = timeline
                            .edit(&item, EditedContent::RoomMessage(content))
                            .await
                        {
                            emit(Event::Error(format!("not edited: {e}")));
                        }
                    });
                }
            }
            Command::Delete { id, thread, reason } => {
                if let (Some(timeline), Some(item)) = (state.pane(thread), item_id(&id)) {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        if let Err(e) = timeline.redact(&item, reason.as_deref()).await {
                            emit(Event::Error(format!("not deleted: {e}")));
                        }
                    });
                }
            }
            Command::React { id, key, thread } => {
                if let (Some(timeline), Some(item)) = (state.pane(thread), item_id(&id)) {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        if let Err(e) = timeline.toggle_reaction(&item, &key).await {
                            emit(Event::Error(format!("reaction not sent: {e}")));
                        }
                    });
                }
            }
            Command::SendFile {
                path,
                thread,
                reply_to,
                caption,
            } => {
                if let Some(timeline) = state.pane(thread) {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let name = path
                            .file_name()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_default();
                        let reply_to = reply_to.and_then(|id| OwnedEventId::try_from(id).ok());
                        if let Err(e) =
                            crate::media::send_file(&timeline, path, reply_to, caption).await
                        {
                            emit(Event::Error(format!("{name} not sent: {e}")));
                        }
                    });
                }
            }
            Command::SendVoice {
                path,
                thread,
                reply_to,
                duration_ms,
                waveform,
            } => {
                if let Some(timeline) = state.pane(thread) {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let reply_to = reply_to.and_then(|id| OwnedEventId::try_from(id).ok());
                        let sent = crate::media::send_voice(
                            &timeline,
                            path.clone(),
                            duration_ms,
                            waveform,
                            reply_to,
                        )
                        .await;
                        let _ = tokio::fs::remove_file(&path).await;
                        if let Err(e) = sent {
                            emit(Event::Error(format!("voice message not sent: {e}")));
                        }
                    });
                }
            }
            Command::MessageVersions { event_id } => {
                if let Some(open) = &state.open {
                    let room = open.timeline.room().clone();
                    let mut timelines = vec![open.timeline.clone()];
                    timelines.extend(open.thread.as_ref().map(|t| t.timeline.clone()));
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let versions =
                            crate::versions::versions(&room, &timelines, &event_id).await;
                        emit(Event::MessageVersions { event_id, versions });
                    });
                }
            }
            Command::Forward {
                event_ids,
                to_rooms,
            } => {
                if let (Some(client), Some(open)) = (state.client.clone(), &state.open) {
                    // A reply of the open thread is in the thread's timeline only
                    let mut timelines = vec![open.timeline.clone()];
                    timelines.extend(open.thread.as_ref().map(|t| t.timeline.clone()));
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        match crate::forward::forward(&client, &timelines, &event_ids, &to_rooms)
                            .await
                        {
                            Ok(to_rooms) => emit(Event::Forwarded { to_rooms }),
                            Err(e) => emit(Event::Error(format!("not forwarded: {e}"))),
                        }
                    });
                }
            }
            Command::DeleteChat { room_id, before_ts } => {
                if let Some(client) = state.client.clone() {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let done =
                            crate::deleted_chats::set(&client, &emit, &room_id, before_ts).await;
                        if let Err(e) = done {
                            emit(Event::Error(format!("chat not deleted: {e}")));
                        }
                    });
                }
            }
            Command::SaveMessage { event_id, saved } => {
                if let (Some(client), Some(index), Some(open)) =
                    (state.client.clone(), state.index.clone(), &state.open)
                {
                    let room_id = open.timeline.room().room_id().to_string();
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let done =
                            crate::saved::set(&client, &index, &emit, &room_id, &event_id, saved)
                                .await;
                        if let Err(e) = done {
                            emit(Event::Error(format!("not saved: {e}")));
                        }
                    });
                }
            }
            Command::FetchFile { key } => {
                if let Some(client) = state.client.clone() {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let bytes = crate::media::file_bytes(&client, &key).await;
                        if let Err(e) = &bytes {
                            tracing::debug!("file {key}: {e}");
                        }
                        emit(Event::FileData {
                            key,
                            bytes: bytes.ok().map(Arc::new),
                        });
                    });
                }
            }
            Command::OpenPdf { key } => {
                if let Some(client) = state.client.clone() {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        match crate::media::file_bytes(&client, &key).await {
                            Ok(bytes) => crate::pdf::open(key, bytes, emit),
                            Err(error) => emit(Event::PdfFailed { key, error }),
                        }
                    });
                }
            }
            Command::RenderPdfPage { key, page, width } => {
                crate::pdf::render(key, page, width, emit.clone());
            }
            Command::ClosePdf => crate::pdf::close(),
            Command::JoinCall {
                room_id,
                video,
                thread_root,
            } => {
                if let Some(call) = state.call.take() {
                    call.leave().await;
                }
                if let Some(client) = state.client.clone() {
                    state.call = Some(crate::calls::start(
                        client,
                        room_id,
                        video,
                        thread_root,
                        emit.clone(),
                    ));
                }
            }
            Command::LeaveCall => {
                if let Some(call) = state.call.take() {
                    tokio::spawn(call.leave());
                }
            }
            Command::DeclineCall { room_id } => {
                if let Some(declines) = &state.declines {
                    let _ = declines.send(room_id);
                }
            }
            Command::CallMicrophone(on) => {
                if let Some(call) = &state.call {
                    call.send(crate::calls::CallCommand::Microphone(on));
                }
            }
            Command::CallCamera(on) => {
                if let Some(call) = &state.call {
                    call.send(crate::calls::CallCommand::Camera(on));
                }
            }
            Command::CallScreen(on) => {
                if let Some(call) = &state.call {
                    call.send(crate::calls::CallCommand::Screen(on));
                }
            }
            Command::CallDevices(kinds) => {
                if let Some(call) = &state.call {
                    call.send(crate::calls::CallCommand::Devices(kinds));
                }
            }
            Command::CallSwitchDevice(kind, id) => {
                if let Some(call) = &state.call {
                    call.send(crate::calls::CallCommand::SwitchDevice(kind, id));
                }
            }
            Command::FetchAnimation { key, width, height } => {
                if let Some(client) = state.client.clone() {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let frames = crate::media::load_animation(&client, &key, width, height)
                            .await
                            .unwrap_or_else(|e| {
                                tracing::debug!("animation {key}: {e}");
                                Vec::new()
                            });
                        emit(Event::Animation {
                            key,
                            width,
                            height,
                            frames,
                        });
                    });
                }
            }
            Command::Vote {
                poll_id,
                answers,
                thread,
            } => {
                if let (Some(timeline), Ok(id)) =
                    (state.pane(thread), OwnedEventId::try_from(poll_id))
                {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let content = UnstablePollResponseEventContent::new(answers, id);
                        if let Err(e) = timeline.send(content.into()).await {
                            emit(Event::Error(format!("vote not sent: {e}")));
                        }
                    });
                }
            }
            Command::EndPoll { poll_id, thread } => {
                if let (Some(timeline), Ok(id)) =
                    (state.pane(thread), OwnedEventId::try_from(poll_id))
                {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        let content = UnstablePollEndEventContent::new("The poll has ended", id);
                        if let Err(e) = timeline.send(content.into()).await {
                            emit(Event::Error(format!("poll not ended: {e}")));
                        }
                    });
                }
            }
            Command::CreatePoll {
                question,
                answers,
                multiple,
                thread,
            } => {
                if let Some(timeline) = state.pane(thread) {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        if let Err(e) = send_poll(&timeline, question, answers, multiple).await {
                            emit(Event::Error(format!("poll not sent: {e}")));
                        }
                    });
                }
            }
            Command::Pin { ids, pinned } => {
                if let Some(open) = &state.open {
                    let room = open.timeline.room().clone();
                    let event_ids: Vec<OwnedEventId> = ids
                        .into_iter()
                        .filter_map(|id| OwnedEventId::try_from(id).ok())
                        .collect();
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        // One update: pinning one by one would each read the list before the
                        // other's change came back (web `setPinned`)
                        let done = set_pinned(&room, &event_ids, pinned).await;
                        if let Err(e) = done {
                            emit(Event::Error(format!(
                                "not {}: {e}",
                                if pinned { "pinned" } else { "unpinned" }
                            )));
                        }
                    });
                }
            }
            Command::OpenPinned => open_side(&mut state, Side::Pinned, &emit).await,
            Command::DiscardSession => {
                if let Some(open) = &state.open {
                    let room = open.timeline.room().clone();
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        match room.discard_room_key().await {
                            Ok(()) => emit(Event::Notice("Encryption session discarded".into())),
                            Err(e) => emit(Event::Error(format!("discard session: {e}"))),
                        }
                    });
                }
            }
            Command::RoomInfo { room_id } => {
                if let Some(room) = state.room(&room_id) {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        match crate::room_admin::details(&room).await {
                            Ok(details) => emit(Event::RoomInfo(details)),
                            Err(e) => emit(Event::Error(format!("room info: {e}"))),
                        }
                    });
                }
            }
            Command::Room { room_id, action } => {
                if let Some(room) = state.room(&room_id) {
                    let emit = emit.clone();
                    tokio::spawn(room_action(room, action, emit));
                }
            }
            Command::CreateRoom {
                name,
                topic,
                invite,
                encrypted,
            } => {
                if let Some(client) = state.client.clone() {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        match create_room(&client, name, topic, invite, encrypted).await {
                            Ok(room_id) => emit(Event::RoomCreated { room_id }),
                            Err(e) => emit(Event::Error(format!("room not created: {e}"))),
                        }
                    });
                }
            }
            Command::OpenDirect { user_id } => {
                if let Some(client) = state.client.clone() {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        match open_direct(&client, &user_id).await {
                            Ok(room_id) => emit(Event::RoomCreated { room_id }),
                            Err(e) => emit(Event::Error(format!("cannot write to {user_id}: {e}"))),
                        }
                    });
                }
            }
            Command::OpenThread { root_id } => {
                open_side(&mut state, Side::Thread(&root_id), &emit).await
            }
            Command::CloseThread => {
                if let Some(open) = state.open.as_mut() {
                    open.thread = None;
                }
            }
            Command::LoadOlderInThread => {
                if let Some(timeline) = state.pane(true) {
                    tokio::spawn(async move {
                        let _ = timeline.paginate_backwards(HISTORY_PAGE).await;
                    });
                }
            }
            Command::Send {
                text,
                thread,
                reply_to,
                mentions,
                emote,
                command,
            } => {
                let text = text.trim().to_string();
                if let (Some(timeline), false) = (state.pane(thread), text.is_empty()) {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        if let Err(e) =
                            send_text(&timeline, text, reply_to, &mentions, emote, command).await
                        {
                            emit(Event::Error(format!("not sent: {e}")));
                        }
                    });
                }
            }
            Command::Recover { key } => {
                if let Some(client) = &state.client {
                    match client.encryption().recovery().recover(key.trim()).await {
                        Ok(()) => {
                            emit(Event::Notice("Encrypted history unlocked".into()));
                            // Messages that failed before the key was known: fetch their
                            // keys now; the timeline decrypts them again when they arrive.
                            if let Some(open) = &state.open {
                                let room_id = open.timeline.room().room_id().to_owned();
                                let client = client.clone();
                                tokio::spawn(async move {
                                    let _ = client
                                        .encryption()
                                        .backups()
                                        .download_room_keys_for_room(&room_id)
                                        .await;
                                });
                            }
                        }
                        Err(e) => emit(Event::Error(format!("recovery key refused: {e}"))),
                    }
                }
            }
            Command::FetchImage {
                key,
                width,
                height,
                crop,
            } => {
                if let Some(client) = &state.client {
                    crate::media::fetch_image(
                        client.clone(),
                        state.images.clone(),
                        key,
                        width,
                        height,
                        crop,
                        emit.clone(),
                    );
                }
            }
            Command::SaveAttachment { key, name } => {
                if let Some(client) = state.client.clone() {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        match crate::media::save_file(&client, &key, &name).await {
                            Ok(path) => emit(Event::Notice(format!("Saved to {}", path.display()))),
                            Err(e) => emit(Event::Error(format!("not saved: {e}"))),
                        }
                    });
                }
            }
            Command::OpenAttachment { key, name } => {
                if let Some(client) = state.client.clone() {
                    let emit = emit.clone();
                    tokio::spawn(async move {
                        match crate::media::save_file(&client, &key, &name).await {
                            Ok(path) => {
                                if let Err(e) = open::that(&path) {
                                    emit(Event::Error(format!(
                                        "cannot open {}: {e}",
                                        path.display()
                                    )));
                                }
                            }
                            Err(e) => emit(Event::Error(format!("cannot open: {e}"))),
                        }
                    });
                }
            }
            Command::SignOut => {
                stop_session(&mut state).await;
                if let Some(client) = state.client.take() {
                    let _ = client.matrix_auth().logout().await;
                }
                session::forget();
                emit(Event::Session(SessionState::SignedOut));
            }
        }
    }
}

async fn start_session(state: &mut State, client: Client, emit: &Emit) {
    stop_session(state).await;
    let user_id = client.user_id().map(|u| u.to_string()).unwrap_or_default();
    emit(Event::Session(SessionState::SignedIn { user_id }));

    // Latest events (room previews) need the event cache.
    if let Err(e) = client.event_cache().subscribe() {
        emit(Event::Error(format!("event cache: {e}")));
    }
    let sync = match SyncService::builder(client.clone()).build().await {
        Ok(sync) => sync,
        Err(e) => {
            emit(Event::Error(format!("sync: {e}")));
            return;
        }
    };
    sync.start().await;
    let service = sync.room_list_service();
    state.tasks.push(tokio::spawn(rooms::run(
        client.clone(),
        service,
        emit.clone(),
    )));
    let (index, writer) = crate::search_index::SearchIndex::start(client.clone());
    state.tasks.push(writer);
    let (_activity, activity_writer) =
        crate::activity::ActivityLog::start(client.clone(), index.clone(), emit.clone()).await;
    state.tasks.push(activity_writer);
    state.tasks.push(tokio::spawn(crate::saved::start(
        client.clone(),
        index.clone(),
        emit.clone(),
    )));
    state.tasks.push(tokio::spawn(crate::autoplay::start(
        client.clone(),
        emit.clone(),
    )));
    state.tasks.push(tokio::spawn(crate::deleted_chats::start(
        client.clone(),
        emit.clone(),
    )));
    let (declines, declined) = mpsc::unbounded_channel();
    state.declines = Some(declines);
    state.tasks.push(tokio::spawn(crate::calls::watch_calls(
        client.clone(),
        emit.clone(),
        declined,
    )));
    state.index = Some(index);
    state
        .tasks
        .push(tokio::spawn(watch_encryption(client.clone(), emit.clone())));
    state.tasks.push(tokio::spawn(watch_tokens(client.clone())));
    state.tasks.push(tokio::spawn(crate::spaces::run(
        client.clone(),
        emit.clone(),
    )));
    state.sync = Some(sync);
    state.client = Some(client);
}

async fn stop_session(state: &mut State) {
    if let Some(call) = state.call.take() {
        call.leave().await;
    }
    if let Some(open) = state.open.take() {
        open.task.abort();
    }
    if let Some(index) = state.index.take() {
        index.flush().await;
    }
    for task in state.tasks.drain(..) {
        task.abort();
    }
    if let Some(sync) = state.sync.take() {
        sync.stop().await;
    }
}

async fn open_room(state: &mut State, room_id: &str, emit: &Emit) {
    let Some(client) = &state.client else { return };
    let Ok(id) = OwnedRoomId::try_from(room_id) else {
        return;
    };
    let Some(room) = client.get_room(&id) else {
        emit(Event::Error(format!("unknown room {room_id}")));
        return;
    };
    if let Some(open) = state.open.take() {
        open.task.abort();
    }
    if let Some(sync) = &state.sync {
        sync.room_list_service()
            .set_room_subscriptions(&[&id])
            .await;
    }
    let name = room
        .cached_display_name()
        .map(|n| n.to_string())
        .unwrap_or_else(|| room_id.to_string());
    emit(Event::RoomOpened {
        room_id: room_id.to_string(),
        name,
        is_encrypted: room.encryption_state().is_encrypted(),
    });
    // Thread replies stay in their thread, as in the web app (Discord-like threads).
    let built = room
        .timeline_builder()
        .with_focus(TimelineFocus::Live {
            hide_threaded_events: true,
        })
        .event_filter(crate::timeline::shown_event)
        .build()
        .await;
    let timeline = match built {
        Ok(t) => Arc::new(t),
        Err(e) => {
            emit(Event::Error(format!("timeline: {e}")));
            return;
        }
    };
    let task = tokio::spawn(timeline::run(
        room_id.to_string(),
        timeline::Pane::Main,
        timeline.clone(),
        state.index.clone(),
        emit.clone(),
    ));
    let complete = Arc::new(std::sync::atomic::AtomicBool::new(false));
    // A first page of history so the screen is never empty.
    {
        let timeline = timeline.clone();
        let complete = complete.clone();
        let emit = emit.clone();
        tokio::spawn(async move {
            emit(Event::History {
                loading: true,
                at_start: false,
            });
            let at_start = timeline
                .paginate_backwards(HISTORY_PAGE)
                .await
                .unwrap_or(false);
            complete.fetch_or(at_start, std::sync::atomic::Ordering::Relaxed);
            emit(Event::History {
                loading: false,
                at_start,
            });
        });
    }
    let typing = tokio::spawn(watch_typing(room.clone(), emit.clone()));
    tokio::spawn(composer_context(room.clone(), emit.clone()));
    state.open = Some(OpenRoom {
        timeline,
        task,
        typing,
        thread: None,
        complete,
    });
}

/// Opens the side panel: the thread of `root_id`, or the pinned messages without one.
/// What the side panel shows beside the conversation.
enum Side<'a> {
    Thread(&'a str),
    Pinned,
    /// The messages around a search result.
    Around(String),
}

async fn open_side(state: &mut State, side: Side<'_>, emit: &Emit) {
    let index = state.index.clone();
    let Some(open) = state.open.as_mut() else {
        return;
    };
    let (focus, root_id) = match side {
        Side::Thread(id) => match OwnedEventId::try_from(id) {
            Ok(root_event_id) => (TimelineFocus::Thread { root_event_id }, id.to_string()),
            Err(_) => return,
        },
        Side::Pinned => (TimelineFocus::PinnedEvents, "pinned".to_string()),
        Side::Around(id) => match OwnedEventId::try_from(id.as_str()) {
            Ok(target) => (
                TimelineFocus::Event {
                    target,
                    num_context_events: 20,
                    thread_mode: matrix_sdk_ui::timeline::TimelineEventFocusThreadMode::Automatic {
                        hide_threaded_events: false,
                    },
                },
                "around".to_string(),
            ),
            Err(_) => return,
        },
    };
    let room = open.timeline.room().clone();
    let built = room.timeline_builder().with_focus(focus).build().await;
    let timeline = match built {
        Ok(t) => Arc::new(t),
        Err(e) => {
            emit(Event::Error(format!("thread: {e}")));
            return;
        }
    };
    let task = tokio::spawn(timeline::run(
        room.room_id().to_string(),
        timeline::Pane::Thread(root_id.clone()),
        timeline.clone(),
        index,
        emit.clone(),
    ));
    {
        let timeline = timeline.clone();
        tokio::spawn(async move {
            if root_id != "around" {
                let _ = timeline.paginate_backwards(HISTORY_PAGE).await;
            }
        });
    }
    open.thread = Some(OpenThread { timeline, task });
}

async fn watch_encryption(client: Client, emit: Emit) {
    let encryption = client.encryption();
    let mut verification = encryption.verification_state();
    let mut recovery = encryption.recovery().state_stream();
    let mut verified = verification.get() == VerificationState::Verified;
    let available =
        |state: RecoveryState| matches!(state, RecoveryState::Enabled | RecoveryState::Incomplete);
    let mut recovery_available = available(encryption.recovery().state());
    emit(Event::Encryption(EncryptionStatus {
        verified,
        recovery_available,
    }));
    loop {
        tokio::select! {
            Some(v) = verification.next() => verified = v == VerificationState::Verified,
            Some(r) = recovery.next() => recovery_available = available(r),
            else => break,
        }
        emit(Event::Encryption(EncryptionStatus {
            verified,
            recovery_available,
        }));
    }
}

async fn watch_tokens(client: Client) {
    let mut receiver = client.subscribe_to_session_changes();
    while let Ok(change) = receiver.recv().await {
        if matches!(change, matrix_sdk::SessionChange::TokensRefreshed) {
            session::on_tokens_refreshed(&client);
        }
    }
}

/// The pinned list of `room` with `event_ids` added (in order, at its end) or taken off.
async fn set_pinned(
    room: &matrix_sdk::Room,
    event_ids: &[OwnedEventId],
    pinned: bool,
) -> matrix_sdk::Result<()> {
    use matrix_sdk::ruma::events::room::pinned_events::RoomPinnedEventsEventContent;
    let mut list = match room.pinned_event_ids() {
        Some(list) => list,
        None => room.load_pinned_events().await?.unwrap_or_default(),
    };
    let before = list.clone();
    if pinned {
        for id in event_ids {
            if !list.contains(id) {
                list.push(id.clone());
            }
        }
    } else {
        list.retain(|id| !event_ids.contains(id));
    }
    if list != before {
        room.send_state_event(RoomPinnedEventsEventContent::new(list))
            .await?;
    }
    Ok(())
}

async fn room_action(room: matrix_sdk::Room, action: RoomAction, emit: Emit) {
    use crate::room_admin as admin;
    let room_id = room.room_id().to_string();
    let (result, done) = match &action {
        RoomAction::Invite(user) => (admin::invite(&room, user).await, format!("{user} invited")),
        RoomAction::Remove(user, reason) => (
            admin::remove(&room, user, reason.as_deref()).await,
            format!("{user} removed"),
        ),
        RoomAction::Ban(user, reason) => (
            admin::ban(&room, user, reason.as_deref()).await,
            format!("{user} banned"),
        ),
        RoomAction::SetRole(user, role) => (
            admin::set_role(&room, user, *role).await,
            format!("{user} is now {}", role.label()),
        ),
        RoomAction::Rename(name) => (admin::rename(&room, name).await, "Room renamed".into()),
        RoomAction::SetTopic(topic) => (
            admin::set_topic(&room, topic).await,
            "Description saved".into(),
        ),
        RoomAction::Encrypt => (admin::encrypt(&room).await, "Encryption turned on".into()),
        RoomAction::Join => (
            room.join().await.map_err(|e| e.to_string()),
            "Joined".into(),
        ),
        RoomAction::Leave => (admin::leave(&room).await, "Left the room".into()),
        RoomAction::Report(reason) => (
            admin::report_room(&room, reason.clone()).await,
            "Room reported".into(),
        ),
        RoomAction::ReportMessage(event_id, reason) => (
            admin::report_message(&room, event_id, reason.clone()).await,
            "Message reported".into(),
        ),
        // Nothing said when it worked (web)
        RoomAction::Block(user) => (admin::block(&room, user).await, String::new()),
        RoomAction::SetJoinRule(rule) => (
            admin::set_join_rule(&room, *rule).await,
            "Access changed".into(),
        ),
        RoomAction::SetListed(listed) => (
            admin::set_listed(&room, *listed).await,
            if *listed {
                "Listed in the directory".into()
            } else {
                "Taken off the directory".into()
            },
        ),
    };
    match result {
        Ok(()) => {
            if !done.is_empty() {
                emit(Event::Notice(done));
            }
            if matches!(
                action,
                RoomAction::Report(_) | RoomAction::ReportMessage(..) | RoomAction::Block(_)
            ) {
                // Nothing of the room changed
            } else if matches!(action, RoomAction::Leave) {
                emit(Event::RoomLeft { room_id });
            } else if matches!(action, RoomAction::Join) {
                emit(Event::RoomCreated { room_id });
            } else if let Ok(details) = admin::details(&room).await {
                emit(Event::RoomInfo(details));
            }
        }
        Err(e) => emit(Event::Error(e)),
    }
}

async fn create_room(
    client: &Client,
    name: String,
    topic: String,
    invite: Vec<String>,
    encrypted: bool,
) -> Result<String, String> {
    use matrix_sdk::ruma::{
        UserId,
        api::client::room::create_room::v3::{Request, RoomPreset},
        events::{InitialStateEvent, room::encryption::RoomEncryptionEventContent},
    };
    let mut request = Request::new();
    request.name = Some(name.trim().to_string()).filter(|n| !n.is_empty());
    request.topic = Some(topic.trim().to_string()).filter(|t| !t.is_empty());
    request.preset = Some(RoomPreset::PrivateChat);
    // Everyone may join a call: the membership is a state event, which the default
    // power levels keep to moderators (web `powerLevelsOverride`)
    request.power_level_content_override = matrix_sdk::ruma::serde::Raw::from_json_string(
        serde_json::json!({ "events": { crate::calls::CALL_MEMBER_TYPE: 0 } }).to_string(),
    )
    .ok();
    request.invite = invite
        .iter()
        .map(|u| UserId::parse(u.trim()).map_err(|_| format!("« {u} » is not a Matrix id")))
        .collect::<Result<_, _>>()?;
    if encrypted {
        request.initial_state = vec![
            InitialStateEvent::with_empty_state_key(
                RoomEncryptionEventContent::with_recommended_defaults(),
            )
            .to_raw_any(),
        ];
    }
    let room = client
        .create_room(request)
        .await
        .map_err(|e| e.to_string())?;
    Ok(room.room_id().to_string())
}

async fn open_direct(client: &Client, user_id: &str) -> Result<String, String> {
    use matrix_sdk::ruma::{
        UserId,
        api::client::room::create_room::v3::{Request, RoomPreset},
    };
    let user =
        UserId::parse(user_id.trim()).map_err(|_| format!("« {user_id} » is not a Matrix id"))?;
    if let Some(room) = client.get_dm_room(&user) {
        return Ok(room.room_id().to_string());
    }
    let mut request = Request::new();
    request.invite = vec![user];
    request.is_direct = true;
    request.preset = Some(RoomPreset::TrustedPrivateChat);
    let room = client
        .create_room(request)
        .await
        .map_err(|e| e.to_string())?;
    Ok(room.room_id().to_string())
}

async fn watch_typing(room: matrix_sdk::Room, emit: Emit) {
    // The guard keeps the event handler alive as long as this task.
    let (_guard, mut receiver) = room.subscribe_to_typing_notifications();
    let room_id = room.room_id().to_string();
    while let Ok(user_ids) = receiver.recv().await {
        let mut names = Vec::with_capacity(user_ids.len());
        for user_id in &user_ids {
            let name = match room.get_member_no_sync(user_id).await {
                Ok(Some(member)) => member.name().to_string(),
                _ => crate::room_admin::localpart(user_id.as_str()),
            };
            names.push(name);
        }
        emit(Event::Typing {
            room_id: room_id.clone(),
            names,
        });
    }
}

async fn send_poll(
    timeline: &Timeline,
    question: String,
    answers: Vec<String>,
    multiple: bool,
) -> Result<(), String> {
    use matrix_sdk::ruma::{
        UInt,
        events::poll::unstable_start::{
            NewUnstablePollStartEventContent, UnstablePollAnswer, UnstablePollAnswers,
            UnstablePollStartContentBlock, UnstablePollStartEventContent,
        },
    };
    let answers: Vec<UnstablePollAnswer> = answers
        .iter()
        .map(|a| a.trim())
        .filter(|a| !a.is_empty())
        .enumerate()
        .map(|(i, text)| UnstablePollAnswer::new(format!("answer-{i}"), text))
        .collect();
    let count = answers.len();
    let answers = UnstablePollAnswers::try_from(answers)
        .map_err(|_| "a poll needs 2 to 20 answers".to_string())?;
    let mut block = UnstablePollStartContentBlock::new(question.trim(), answers);
    block.max_selections =
        UInt::try_from(if multiple { count } else { 1 } as u64).unwrap_or(UInt::MIN);
    let content = UnstablePollStartEventContent::New(NewUnstablePollStartEventContent::new(block));
    timeline
        .send(content.into())
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// A message of the composer: text or emote with its mentions, as a reply when
/// `reply_to` is given; a bot command goes out with its MSC4332 key.
async fn send_text(
    timeline: &Timeline,
    text: String,
    reply_to: Option<String>,
    mentions: &[twake_domain::mention::Mention],
    emote: bool,
    command: Option<twake_domain::composer::BotCommand>,
) -> Result<(), String> {
    let (message, set) = crate::compose::text_content(&text, mentions, emote);
    if let Some(command) = command {
        let mut content =
            serde_json::to_value(RoomMessageEventContent::new(message).add_mentions(set))
                .map_err(|e| e.to_string())?;
        content[crate::compose::BOT_COMMAND_KEY] = crate::compose::command_content(&text, &command);
        return timeline
            .room()
            .send_raw("m.room.message", content)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string());
    }
    match reply_to.and_then(|id| OwnedEventId::try_from(id).ok()) {
        Some(event_id) => timeline
            .send_reply(
                RoomMessageEventContentWithoutRelation::new(message).add_mentions(set),
                event_id,
            )
            .await
            .map(|_| ())
            .map_err(|e| e.to_string()),
        None => timeline
            .send(
                RoomMessageEventContent::new(message)
                    .add_mentions(set)
                    .into(),
            )
            .await
            .map(|_| ())
            .map_err(|e| e.to_string()),
    }
}

/// [`Event::Composer`] for the room just opened.
async fn composer_context(room: matrix_sdk::Room, emit: Emit) {
    let Ok(details) = crate::room_admin::details(&room).await else {
        return;
    };
    let joined: Vec<&str> = details
        .members
        .iter()
        .filter(|m| !m.is_invited)
        .map(|m| m.user_id.as_str())
        .collect();
    let mut commands = Vec::new();
    for kind in crate::compose::BOT_COMMANDS_TYPES {
        let Ok(events) = room.get_state_events(kind.into()).await else {
            continue;
        };
        for raw in events {
            let matrix_sdk::deserialized_responses::RawAnySyncOrStrippedState::Sync(raw) = raw
            else {
                continue;
            };
            let Ok(value) = raw.deserialize_as::<serde_json::Value>() else {
                continue;
            };
            let bot = value
                .get("state_key")
                .and_then(|k| k.as_str())
                .unwrap_or("");
            if joined.contains(&bot) {
                commands.extend(crate::compose::read_bot_commands(bot, &value["content"]));
            }
        }
    }
    emit(Event::Composer {
        room_id: room.room_id().to_string(),
        can_mention_everyone: details.permissions.can_mention_everyone,
        members: details.members,
        commands,
    });
}

/// [`Command::Search`]: asks the server too in a room that is not encrypted, then
/// matches the local index.
async fn search(
    timeline: Arc<Timeline>,
    complete: Arc<std::sync::atomic::AtomicBool>,
    index: Arc<crate::search_index::SearchIndex>,
    filter: twake_domain::search::SearchFilter,
    older: bool,
    emit: Emit,
) {
    use std::sync::atomic::Ordering;
    if older && !complete.load(Ordering::Relaxed) {
        let at_start = timeline
            .paginate_backwards(HISTORY_PAGE)
            .await
            .unwrap_or(false);
        complete.fetch_or(at_start, Ordering::Relaxed);
        // The page reaches the index through the timeline task
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }
    let room = timeline.room().clone();
    let query = twake_domain::js::trim(&filter.query).to_string();
    if !room.encryption_state().is_encrypted()
        && !query.is_empty()
        && let Err(e) = index.ask_server(&room, &query).await
    {
        emit(Event::Error(format!("server search: {e}")));
    }
    let results = if twake_domain::search::is_empty_filter(&filter) {
        Vec::new()
    } else {
        index.search(room.room_id().as_str(), &filter).await
    };
    emit(Event::SearchResults {
        room_id: room.room_id().to_string(),
        query: filter.query,
        results,
        is_complete: complete.load(Ordering::Relaxed),
    });
}

/// [`Command::JumpToDate`]: the first message from `ts` on, else the last one before.
async fn jump_to_date(client: Option<Client>, room: matrix_sdk::Room, ts: i64, emit: Emit) {
    use matrix_sdk::ruma::api::Direction;
    use matrix_sdk::ruma::api::client::room::get_event_by_timestamp::v1::Request;
    let Some(client) = client else { return };
    let at = matrix_sdk::ruma::MilliSecondsSinceUnixEpoch(
        matrix_sdk::ruma::UInt::try_from(ts.max(0)).unwrap_or_default(),
    );
    for dir in [Direction::Forward, Direction::Backward] {
        let request = Request::new(room.room_id().to_owned(), at, dir);
        if let Ok(found) = client.send(request).await {
            emit(Event::DateFound {
                room_id: room.room_id().to_string(),
                event_id: found.event_id.to_string(),
            });
            return;
        }
    }
    emit(Event::Notice("No message found around that date.".into()));
}
