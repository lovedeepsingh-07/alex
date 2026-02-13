use crate::{player, protocol};
use colored::Colorize;

pub async fn handle(
    player: &mut player::Player,
    sub_command: protocol::PlayerSubCommand,
) -> protocol::Response {
    match sub_command {
        protocol::PlayerSubCommand::Play { audio_label } => {
            match player.play(&audio_label) {
                Ok(_) => {
                    log::debug!(
                        "Playing {quote}{}{quote}",
                        audio_label.purple(),
                        quote = "\"".purple()
                    );
                    return protocol::Response::PlaybackStarted {
                        audio_label: audio_label.to_string(),
                    };
                }
                Err(e) => {
                    log::error!("Failed to play the audio: {}", e.to_string());
                    return protocol::Response::ERROR {
                        message: String::from("Failed to play the audio"),
                    };
                }
            };
        }
        protocol::PlayerSubCommand::Pause => {
            log::debug!("Pausing playback");
            player.pause();
            return protocol::Response::Paused;
        }
        protocol::PlayerSubCommand::Resume => {
            log::debug!("Resuming playback");
            player.resume();
            return protocol::Response::Resumed;
        }
        protocol::PlayerSubCommand::Clear => {
            log::debug!("Clearing player queue");
            player.clear();
            return protocol::Response::Cleared;
        }
    }
}
