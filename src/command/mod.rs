mod player_cmd;
mod reload_cmd;
mod search_cmd;
mod status_cmd;

use crate::{error, player, protocol};

pub async fn handle(
    request: protocol::Request,
    player: &mut player::Player,
) -> Result<protocol::Response, error::Error> {
    match request {
        protocol::Request::Status => status_cmd::handle(player).await,
        protocol::Request::Reload => reload_cmd::handle(player).await,
        protocol::Request::Search { search_term } => search_cmd::handle(player, search_term).await,
        protocol::Request::Player { sub_command } => player_cmd::handle(player, sub_command).await,
    }
}
