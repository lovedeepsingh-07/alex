mod player_cmd;
mod reload_cmd;
mod search_cmd;

use crate::{daemon::player, error};

#[derive(Debug)]
pub enum Command {
    Reload,
    Search(Option<String>),
    Player(PlayerSubCommand),
}
#[derive(Debug)]
pub enum PlayerSubCommand {
    Play(String),
    Pause,
    Resume,
    Clear,
}

pub fn handle(cmd: Command, player: &mut player::Player) -> Result<(), error::Error> {
    match cmd {
        Command::Reload => reload_cmd::handle(player)?,
        Command::Search(search_term) => search_cmd::handle(player, search_term)?,
        Command::Player(player_sub_command) => player_cmd::handle(player, player_sub_command)?,
    }
    Ok(())
}
