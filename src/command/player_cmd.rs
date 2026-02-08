use crate::{command, error, player, protocol};
use colored::Colorize;

pub async fn handle(
    player: &mut player::Player,
    sub_command: command::PlayerSubCommand,
) -> Result<protocol::Response, error::Error> {
    let mut player_sub_command = protocol::R_PlayerSubCommand::Resume;
    let mut player_result = protocol::R_Result::OK;

    match sub_command {
        command::PlayerSubCommand::Play { audio_label } => {
            match player.play(&audio_label) {
                Ok(_) => {
                    log::debug!(
                        "Playing {quote}{}{quote}",
                        audio_label.purple(),
                        quote = "\"".purple()
                    );
                    player_sub_command = protocol::R_PlayerSubCommand::Play {
                        audio_label: audio_label.to_string(),
                    };
                },
                Err(e) => {
                    log::error!("Failed to play the audio: {}", e.to_string());
                    player_result = protocol::R_Result::ERROR {
                        error_message: String::from("Failed to play the audio")
                    };
                }
            };
        }
        command::PlayerSubCommand::Pause => {
            log::debug!("Pausing playback");
            player.pause();
            player_sub_command = protocol::R_PlayerSubCommand::Pause;
        }
        command::PlayerSubCommand::Resume => {
            log::debug!("Resuming playback");
            player.resume();
            player_sub_command = protocol::R_PlayerSubCommand::Resume;
        }
        command::PlayerSubCommand::Clear => {
            log::debug!("Clearing player queue");
            player.clear();
            player_sub_command = protocol::R_PlayerSubCommand::Clear;
        }
    }

    let player_command = protocol::R_Command::Player { sub_command: player_sub_command };
    return Ok(protocol::Response {
        data: Vec::new(),
        packet: bitcode::encode(&protocol::R_Packet {
            result: player_result,
            command: player_command,
        })
    });
}
