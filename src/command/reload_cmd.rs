use crate::{error, player, protocol};

pub async fn handle(player: &mut player::Player) -> Result<protocol::Response, error::Error> {
    log::debug!("Reloading player audio index");
    player.reload()?;
    let response = protocol::Response {
        data: vec!["OK".to_string(), "RELOAD".to_string()],
    };
    Ok(response)
}
