mod reload_h;
mod search_h;
mod status_h;
mod player_h;
use crate::{player, protocol};

pub fn handle(request: protocol::Request, player: &mut player::Player) -> protocol::Response {
    match request {
        protocol::Request::Status => status_h::handle(player),
        protocol::Request::Reload => reload_h::handle(player),
        protocol::Request::Search { search_term } => search_h::handle(player, search_term),
        protocol::Request::Player { sub_command } => player_h::handle(player, sub_command),
    }
}
