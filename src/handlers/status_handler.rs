use crate::{player, protocol};

pub fn handle(player: &mut player::Player) -> protocol::Response {
    return protocol::Response::StatusData(protocol::StatusData {
        current_audio: player.state.current_audio.clone(),
        is_paused: player.state.is_paused,
        is_queue_empty: player.state.is_queue_empty,
    });
}
