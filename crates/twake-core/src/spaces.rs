//! Spaces (Twake Space spaces): the top-level spaces the user joined, with every room
//! below each, for the space rail that filters the room list (web: SpaceChannels).

use futures_util::StreamExt;
use imbl::Vector;
use matrix_sdk::Client;
use matrix_sdk_ui::spaces::{SpaceFilter, SpaceService};
use twake_domain::SpaceSummary;

use crate::engine::{Emit, Event};

pub async fn run(client: Client, emit: Emit) {
    let service = SpaceService::new(client).await;
    // Computes the filters a first time; the subscription then follows the changes.
    let _ = service.space_filters().await;
    let (mut filters, stream) = service.subscribe_to_space_filters().await;
    emit(Event::Spaces(summaries(&filters)));
    futures_util::pin_mut!(stream);
    while let Some(diffs) = stream.next().await {
        for diff in diffs {
            diff.apply(&mut filters);
        }
        emit(Event::Spaces(summaries(&filters)));
    }
}

fn summaries(filters: &Vector<SpaceFilter>) -> Vec<SpaceSummary> {
    filters
        .iter()
        // Top-level spaces only: their descendants include the sub-spaces' rooms.
        .filter(|f| f.level == 0)
        .map(|f| SpaceSummary {
            id: f.space_room.room_id.to_string(),
            name: f.space_room.display_name.clone(),
            avatar_url: f.space_room.avatar_url.as_ref().map(|u| u.to_string()),
            rooms: f.descendants.iter().map(|id| id.to_string()).collect(),
        })
        .collect()
}
