use crate::{player, protocol};

pub fn handle(player: &mut player::Player) -> protocol::Response {
    return protocol::Response::StatusData(protocol::StatusData {
        current_audio: player.get_current_audio().clone(),
        is_paused: player.is_paused(),
        is_queue_empty: player.is_queue_empty(),
        queue: player.get_queue().clone(),
    });
}
