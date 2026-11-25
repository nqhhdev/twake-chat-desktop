//! Pure domain logic of Twake Chat, ported from `src/domain` of the web app
//! (`reference/twake-space-chat`). No I/O, no UI types.
//!
//! Functions keep the name of their TypeScript original in their doc comment, so
//! `rust/units/domain-rs` (rust-anything) can replay the recorded cases against them.

pub mod activity;
pub mod attachment;
pub mod call;
pub mod call_window;
pub mod composer;
pub mod deleted_chats;
pub mod format;
pub mod js;
pub mod link_preview;
pub mod mention;
pub mod model;
pub mod poll;
pub mod rich_text;
pub mod room_management;
pub mod room_preview;
pub mod saved;
pub mod search;
pub mod shared_content;
pub mod text_format;
pub mod web_links;

pub use model::*;
