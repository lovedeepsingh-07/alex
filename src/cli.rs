use crate::{constants, error, flatbuffers_gen::request_packet_ as request_packet, protocol};

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
impl Into<request_packet::StatusSubCommand> for Option<StatusSubCommand> {
    fn into(self) -> request_packet::StatusSubCommand {
        match self {
            Some(StatusSubCommand::CurrentAudio) => request_packet::StatusSubCommand::CurrentAudio,
            Some(StatusSubCommand::IsPaused) => request_packet::StatusSubCommand::IsPaused,
            Some(StatusSubCommand::IsQueueEmpty) => request_packet::StatusSubCommand::IsQueueEmpty,
            None => request_packet::StatusSubCommand::NONE,
        }
    }
}

pub fn generate_request(sub_command: SubCommand) -> Result<protocol::Request, error::Error> {
    let mut request = protocol::Request::new();
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let mut request_packet_args = request_packet::RequestPacketArgs::default();

    match sub_command {
        SubCommand::Daemon => {
            // NOTE: the program never reaches this branch
        }
        SubCommand::Status { sub_command } => {
            let status_command = request_packet::Status::create(
                &mut builder,
                &request_packet::StatusArgs {
                    sub_command: sub_command.into(),
                },
            );
            request_packet_args = request_packet::RequestPacketArgs {
                command_type: request_packet::Command::Status,
                command: Some(status_command.as_union_value()),
            };
        }
        SubCommand::Reload => {
            let reload_command =
                request_packet::Reload::create(&mut builder, &request_packet::ReloadArgs {});
            request_packet_args = request_packet::RequestPacketArgs {
                command_type: request_packet::Command::Reload,
                command: Some(reload_command.as_union_value()),
            };
        }
        SubCommand::Search { search_term } => {
            let search_term = builder.create_string(&search_term.unwrap_or_else(|| String::new()));
            let search_command = request_packet::Search::create(
                &mut builder,
                &request_packet::SearchArgs {
                    search_term: Some(search_term),
                },
            );
            request_packet_args = request_packet::RequestPacketArgs {
                command_type: request_packet::Command::Search,
                command: Some(search_command.as_union_value()),
            };
        }
        SubCommand::Play { audio_label } => {
            let audio_label = builder.create_string(&audio_label);
            let play_sub_command = request_packet::Play::create(
                &mut builder,
                &request_packet::PlayArgs {
                    audio_label: Some(audio_label),
                },
            );
            let player_command = request_packet::Player::create(
                &mut builder,
                &request_packet::PlayerArgs {
                    sub_command_type: request_packet::PlayerSubCommand::Play,
                    sub_command: Some(play_sub_command.as_union_value()),
                },
            );
            request_packet_args = request_packet::RequestPacketArgs {
                command_type: request_packet::Command::Player,
                command: Some(player_command.as_union_value()),
            };
        }
        SubCommand::Resume => {
            let resume_sub_command =
                request_packet::Resume::create(&mut builder, &request_packet::ResumeArgs {});
            let player_command = request_packet::Player::create(
                &mut builder,
                &request_packet::PlayerArgs {
                    sub_command_type: request_packet::PlayerSubCommand::Resume,
                    sub_command: Some(resume_sub_command.as_union_value()),
                },
            );
            request_packet_args = request_packet::RequestPacketArgs {
                command_type: request_packet::Command::Player,
                command: Some(player_command.as_union_value()),
            };
        }
        SubCommand::Pause => {
            let pause_sub_command =
                request_packet::Pause::create(&mut builder, &request_packet::PauseArgs {});
            let player_command = request_packet::Player::create(
                &mut builder,
                &request_packet::PlayerArgs {
                    sub_command_type: request_packet::PlayerSubCommand::Pause,
                    sub_command: Some(pause_sub_command.as_union_value()),
                },
            );
            request_packet_args = request_packet::RequestPacketArgs {
                command_type: request_packet::Command::Player,
                command: Some(player_command.as_union_value()),
            };
        }
        SubCommand::Clear => {
            let clear_sub_command =
                request_packet::Clear::create(&mut builder, &request_packet::ClearArgs {});
            let player_command = request_packet::Player::create(
                &mut builder,
                &request_packet::PlayerArgs {
                    sub_command_type: request_packet::PlayerSubCommand::Clear,
                    sub_command: Some(clear_sub_command.as_union_value()),
                },
            );
            request_packet_args = request_packet::RequestPacketArgs {
                command_type: request_packet::Command::Player,
                command: Some(player_command.as_union_value()),
            };
        }
    }

    let request_packet = request_packet::RequestPacket::create(&mut builder, &request_packet_args);
    builder.finish(request_packet, None);
    request.packet = builder.finished_data().to_vec();
    Ok(request)
}
