use crate::{player, protocol};

pub fn handle(player: &mut player::Player) -> protocol::Response {
    match player.reload_storage() {
        Ok(_) => {
            log::debug!("Reloading player audio storage");
            return protocol::Response::Reloaded;
        }
        Err(e) => {
            log::error!("Failed to reload the player audio storage, {}", e);
            return protocol::Response::ERROR {
                message: "Failed to reload the player audio storage".to_string(),
            };
        }
    }
}
