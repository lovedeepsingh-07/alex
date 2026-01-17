mod player_cmd;
mod reload_cmd;
mod search_cmd;

use crate::{daemon, error, player};

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

pub async fn handle(
    command_response_handle: daemon::CR_Handle,
    player: &mut player::Player,
) -> Result<(), error::Error> {
    match command_response_handle.command {
        Command::Reload => reload_cmd::handle(command_response_handle.response_tx, player).await?,
        Command::Search(search_term) => {
            search_cmd::handle(command_response_handle.response_tx, player, search_term).await?
        }
        Command::Player(player_sub_command) => {
            player_cmd::handle(
                command_response_handle.response_tx,
                player,
                player_sub_command,
            )
            .await?
        }
    }
    Ok(())
}
