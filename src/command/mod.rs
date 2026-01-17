mod player_cmd;
mod reload_cmd;
mod search_cmd;

use crate::{error, player, protocol::response};

#[derive(Debug)]
pub(crate) enum Command {
    Reload,
    Search(Option<String>),
    Player(PlayerSubCommand),
}
#[derive(Debug)]
pub(crate) enum PlayerSubCommand {
    Play(String),
    Pause,
    Resume,
    Clear,
}

pub(crate) async fn handle(
    command: Command,
    player: &mut player::Player,
) -> Result<response::Response, error::Error> {
    match command {
        Command::Reload => reload_cmd::handle(player).await,
        Command::Search(search_term) => search_cmd::handle(player, search_term).await,
        Command::Player(player_sub_command) => player_cmd::handle(player, player_sub_command).await,
    }
}
