use crate::{error, player, protocol};

pub async fn handle(player: &mut player::Player) -> Result<protocol::Response, error::Error> {
    log::debug!("Reloading player audio index");
    player.reload()?;

    Ok(protocol::Response::OK(None))
}
