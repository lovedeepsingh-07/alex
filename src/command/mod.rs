mod player_cmd;
mod reload_cmd;
mod search_cmd;
mod status_cmd;

use crate::{error, player, protocol};

#[derive(Debug)]
pub enum Command {
    Status {
        sub_command: Option<StatusSubCommand>,
    },
    Reload,
    Search {
        search_term: Option<String>,
    },
    Player {
        sub_command: PlayerSubCommand,
    },
}

#[derive(Debug)]
pub enum StatusSubCommand {
    CurrentAudio,
    IsPaused,
    IsQueueEmpty,
}
#[derive(Debug)]
pub enum PlayerSubCommand {
    Play { audio_label: String },
    Pause,
    Resume,
    Clear,
}

pub async fn handle(
    command: Command,
    player: &mut player::Player,
) -> Result<protocol::Response, error::Error> {
    match command {
        Command::Status { sub_command } => status_cmd::handle(player, sub_command).await,
        Command::Reload => reload_cmd::handle(player).await,
        Command::Search { search_term } => search_cmd::handle(player, search_term).await,
        Command::Player { sub_command } => player_cmd::handle(player, sub_command).await,
    }
}
