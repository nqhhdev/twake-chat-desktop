//! Application layer and adapters of Twake Chat (no UI types).
//!
//! The UI talks to an [`Engine`]: it sends [`Command`]s and receives [`Event`]s on its
//! callback, called from the engine's own threads (the UI hops to its event loop).
//! Matrix runs on matrix-rust-sdk (ADR-003).

mod activity;
mod autoplay;
mod calls;
mod compose;
mod engine;
mod media;
mod pdf;
mod room_admin;
mod rooms;
mod saved;
mod search_index;
mod session;
mod spaces;
mod timeline;

pub use calls::{CallDeviceList, CallStatus, CallTile, DeviceKind, IncomingCall};
pub use engine::{Command, EncryptionStatus, Engine, Event, RoomAction, SavedEntry, SessionState};
pub use media::{Picture, mxc_key};
pub use pdf::available as pdf_available;
