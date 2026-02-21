use crate::{player, protocol};
use colored::Colorize;

pub fn handle(
    player: &mut player::Player,
    sub_command: protocol::PlayerSubCommand,
) -> protocol::Response {
    match sub_command {
        protocol::PlayerSubCommand::Play { input, is_path } => {
            match player.play(&input, is_path) {
                Ok(_) => {
                    log::debug!(
                        "Playing {quote}{}{quote}",
                        input.purple(),
                        quote = "\"".purple()
                    );
                    return protocol::Response::PlaybackStarted {
                        input: input.to_string(),
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
        protocol::PlayerSubCommand::Next => {
            match player.next() {
                Ok(_) => {
                    let playing_audio = match player.state.current_audio{
                        Some(ref out) => out,
                        None => {
                            log::error!("Failed to get the currently playing audio");
                            return protocol::Response::ERROR {
                                message: String::from("Failed to get the currently playing audio"),
                            };
                        }
                    };
                    log::debug!(
                        "Playing {quote}{}{quote}",
                        playing_audio.purple(),
                        quote = "\"".purple()
                    );
                    return protocol::Response::Next { playing_audio: playing_audio.to_string() };
                },
                Err(e) => {
                    log::error!("Failed to advance the playing queue: {}", e.to_string());
                    return protocol::Response::ERROR {
                        message: String::from("Failed to advance the playing queue"),
                    };
                },
            };
        }
        protocol::PlayerSubCommand::Push {
            input,
            is_path,
            next,
        } => {
            log::info!("{} {} {}", input, is_path, next);
            let _ = input;
            let _ = is_path;
            let _ = next;
            return protocol::Response::ERROR {
                message: String::from("PUSH not implemented!"),
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
