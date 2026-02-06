use crate::{error, protocol, constants};

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct CliArgs {
    #[arg(long, default_value=constants::DEFAULT_SERVER_PORT, global=true)]
    /// Server port
    pub port: u16,
    #[arg(long, global=true)]
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
    Search {
        search_term: Option<String>,
    },
    /// Play an audio
    Play {
        audio_label: String,
    },
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
impl StatusSubCommand {
    pub fn to_request_key(&self) -> String {
        match self {
            StatusSubCommand::CurrentAudio => "CURRENT_AUDIO",
            StatusSubCommand::IsPaused => "IS_PAUSED",
            StatusSubCommand::IsQueueEmpty => "IS_QUEUE_EMPTY",
        }
        .to_string()
    }
}

pub fn generate_request(sub_command: SubCommand) -> Result<protocol::Request, error::Error> {
    let mut request = protocol::Request::new();
    request.data.push(request.private_key_hash.clone());

    match sub_command {
        SubCommand::Daemon => {}
        SubCommand::Status { sub_command } => {
            request.data.push("STATUS".to_string());
            if let Some(status_sub_command) = sub_command {
                request.data.push(status_sub_command.to_request_key());
            }
        }
        SubCommand::Reload => {
            request.data.push("RELOAD".to_string());
        }
        SubCommand::Search { search_term } => {
            request.data.push("SEARCH".to_string());
            if let Some(search_term) = search_term {
                request.data.push(search_term);
            }
        }
        SubCommand::Play { audio_label } => {
            request
                .data
                .extend(vec!["PLAYER".to_string(), "PLAY".to_string(), audio_label]);
        }
        SubCommand::Resume => {
            request
                .data
                .extend(vec!["PLAYER".to_string(), "RESUME".to_string()]);
        }
        SubCommand::Pause => {
            request
                .data
                .extend(vec!["PLAYER".to_string(), "PAUSE".to_string()]);
        }
        SubCommand::Clear => {
            request
                .data
                .extend(vec!["PLAYER".to_string(), "CLEAR".to_string()]);
        }
    }
    Ok(request)
}
