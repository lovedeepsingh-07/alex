mod player_handler;
mod reload_handler;
mod search_handler;
mod status_handler;

use crate::{player, protocol};

pub fn handle(request: protocol::Request, player: &mut player::Player) -> protocol::Response {
    match request {
        protocol::Request::Status => status_handler::handle(player),
        protocol::Request::Reload => reload_handler::handle(player),
        protocol::Request::Search { search_term } => search_handler::handle(player, search_term),
        protocol::Request::Player { sub_command } => player_handler::handle(player, sub_command),
    }
}
