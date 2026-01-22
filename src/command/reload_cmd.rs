use crate::{error, player, protocol::response};

pub(crate) async fn handle(
    player: &mut player::Player,
) -> Result<response::Response, error::Error> {
    log::debug!("Reloading player audio index");
    player.reload()?;
    let response = response::Response {
        data: vec!["OK".to_string(), "RELOAD".to_string()],
    };
    Ok(response)
}
