use crate::{error, protocol::request};

#[derive(Debug, clap::Parser)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub(crate) sub_command: SubCommand,
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub(crate) enum SubCommand {
    Daemon,
    Reload,
    Search { search_term: Option<String> },
    Play { audio_label: String },
    Pause,
    Resume,
    Clear,
}

pub(crate) fn generate_request(sub_command: SubCommand) -> Result<request::Request, error::Error> {
    let mut request = request::Request::new();
    request.data.push(request.private_key_hash.clone());

    match sub_command {
        SubCommand::Daemon => {}
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
