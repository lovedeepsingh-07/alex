use crate::{constants, error, protocol};

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct CliArgs {
    #[arg(long, default_value=constants::DEFAULT_SERVER_PORT, global=true)]
    /// Server port
    pub port: u16,
    #[arg(long, global = true)]
    /// Pass this argument when calling the `status` subcommand from another program
    pub just_info: bool,
    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub enum SubCommand {
    /// Run the music daemon
    Daemon,
    /// Get information such as which song is playing, whether playback is paused or not etc
    Status {
        #[command(subcommand)]
        sub_command: Option<StatusSubCommand>,
    },
    /// Reload the audio index to reflect any changes to the folder
    Reload,
    /// Search through the audio index
    Search { search_term: Option<String> },
    /// Play an audio
    Play { audio_label: String },
    /// Pause playback (does nothing if already paused)
    Pause,
    /// Resume playback (does nothing if already resumed)
    Resume,
    /// Clear playing queue
    Clear,
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub enum StatusSubCommand {
    /// Current playing audio
    CurrentAudio,
    /// Is the playback paused ?
    IsPaused,
    /// Is the playing queue empty ?
    IsQueueEmpty,
}

pub fn generate_request(sub_command: &SubCommand) -> Result<protocol::Request, error::Error> {
    match sub_command {
        SubCommand::Daemon => {},
        SubCommand::Status { sub_command: _ } => {
            return Ok(protocol::Request::Status);
        },
        SubCommand::Reload => {
            return Ok(protocol::Request::Reload);
        },
        SubCommand::Search { search_term } => {
            return Ok(protocol::Request::Search { search_term: search_term.clone() });
        },
        SubCommand::Play { audio_label } => {
            let audio_label = audio_label.trim().to_string();
            if audio_label.len() == 0 {
                return Err(error::Error::InvalidInputError("You must provide and audio_label with the play command".to_string()));
            }
            return Ok(protocol::Request::Player {
                sub_command: protocol::PlayerSubCommand::Play { audio_label },
            })
        },
        SubCommand::Pause => {
            return Ok(protocol::Request::Player {
                sub_command: protocol::PlayerSubCommand::Pause,
            })
        },
        SubCommand::Resume => {
            return Ok(protocol::Request::Player {
                sub_command: protocol::PlayerSubCommand::Resume,
            })
        },
        SubCommand::Clear => {
            return Ok(protocol::Request::Player {
                sub_command: protocol::PlayerSubCommand::Clear,
            })
        },
    }
    return Err(error::Error::ParseError("Failed to correctly parse CLI arguments".to_string()));
}
