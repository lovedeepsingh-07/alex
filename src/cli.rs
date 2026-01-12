use crate::{error, request};

#[derive(Debug, clap::Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub enum SubCommand {
    Daemon,
    Reload,
    Search {
        search_term: Option<String>,
    },
    Player {
        #[command(subcommand)]
        sub_command: PlayerSubCommand,
    },
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub enum PlayerSubCommand {
    Play { audio_label: String },
    Pause,
    Resume,
    Clear,
}

pub fn generate_request(sub_command: SubCommand) -> Result<request::Request, error::Error> {
    let mut request = request::Request::new();
    request.data.push(request.private_key_hash.clone());

    match sub_command {
        SubCommand::Daemon => {}
        SubCommand::Reload => {
            request.data.push("RELOAD".to_string());
        }
        SubCommand::Search { search_term } => {
            println!("{:#?}", search_term);
            request.data.push("SEARCH".to_string());
            if let Some(search_term) = search_term {
                request.data.push(search_term);
            }
        }
        SubCommand::Player { sub_command } => {
            request.data.push("PLAYER".to_string());
            match sub_command {
                PlayerSubCommand::Play { audio_label } => {
                    request.data.push("PLAY".to_string());
                    request.data.push(audio_label);
                }
                PlayerSubCommand::Pause => {
                    request.data.push("PAUSE".to_string());
                }
                PlayerSubCommand::Resume => {
                    request.data.push("RESUME".to_string());
                }
                PlayerSubCommand::Clear => {
                    request.data.push("CLEAR".to_string());
                }
            }
        }
    }
    Ok(request)
}
