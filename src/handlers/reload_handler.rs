use crate::{player, protocol};

pub async fn handle(player: &mut player::Player) -> protocol::Response {
    match player.reload() {
        Ok(_) => {
            log::debug!("Reloading player audio index");
            return protocol::Response::Reloaded;
        }
        Err(e) => {
            log::error!("Failed to reload the player audio index, {}", e.to_string());
            return protocol::Response::ERROR {
                message: "Failed to reload the player audio index".to_string(),
            };
        }
    }
}
