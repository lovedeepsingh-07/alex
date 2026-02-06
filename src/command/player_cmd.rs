use crate::{command, error, player, protocol};
use colored::Colorize;

pub async fn handle(
    player: &mut player::Player,
    player_sub_command: command::PlayerSubCommand,
) -> Result<protocol::Response, error::Error> {
    let mut response_data: Vec<String> = vec!["OK".to_string(), "PLAYER".to_string()];
    match player_sub_command {
        command::PlayerSubCommand::Play { audio_label } => {
            match player.play(&audio_label) {
                Ok(_) => {
                    log::debug!(
                        "Playing {quote}{}{quote}",
                        audio_label.purple(),
                        quote = "\"".purple()
                    );
                    response_data.push("PLAY".to_string());
                    response_data.push(audio_label.to_string());
                }
                Err(e) => {
                    log::error!("Failed to play the audio: {}", e.to_string());
                }
            };
        }
        command::PlayerSubCommand::Pause => {
            log::debug!("Pausing playback");
            player.pause();
            response_data.push("PAUSE".to_string());
        }
        command::PlayerSubCommand::Resume => {
            log::debug!("Resuming playback");
            player.resume();
            response_data.push("RESUME".to_string());
        }
        command::PlayerSubCommand::Clear => {
            log::debug!("Clearing player queue");
            player.clear();
            response_data.push("CLEAR".to_string());
        }
    }

    Ok(protocol::Response {
        data: response_data,
    })
}
