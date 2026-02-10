use crate::{
    error, player, protocol,
};

pub async fn handle(
    player: &mut player::Player,
) -> Result<protocol::Response, error::Error> {
    return Ok(protocol::Response::StatusData(protocol::StatusData {
        current_audio: player.state.current_audio.clone(),
        is_paused: player.state.is_paused,
        is_queue_empty: player.state.is_queue_empty,
    }));
}
