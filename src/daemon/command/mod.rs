mod player_cmd;
mod reload_cmd;
mod search_cmd;

use crate::{daemon::{self, player}, error};

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

pub fn handle(command_response_handle: daemon::CommandResponseHandle, player: &mut player::Player) -> Result<(), error::Error> {
    match command_response_handle.command {
        Command::Reload => reload_cmd::handle(player)?,
        Command::Search(search_term) => search_cmd::handle(command_response_handle.response_sender, player, search_term)?,
        Command::Player(player_sub_command) => player_cmd::handle(player, player_sub_command)?,
    }
    Ok(())
}
