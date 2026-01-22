use crate::{error, player, protocol::response};

pub(crate) async fn handle(
    player: &mut player::Player,
) -> Result<response::Response, error::Error> {
    Ok(response::Response {
        data: vec!["OK".to_string(), "STATUS".to_string(), serde_json::to_string(&player.state).unwrap()],
    })
}
