use crate::{player, protocol};
use colored::Colorize;

pub fn handle(
    player: &mut player::Player,
    sub_command: protocol::PlayerSubCommand,
) -> protocol::Response {
    match sub_command {
        protocol::PlayerSubCommand::Play { id } => {
            let audio = match player.storage.get_audio(&id) {
                Ok(out) => out,
                Err(e) => {
                    log::error!("Failed to get audio from the storage, {}", e);
                    return protocol::Response::ERROR {
                        message: "Failed to get audio with the corresponding ID from the storage"
                            .to_string(),
                    };
                }
            }
            .clone();
            match player.play(&audio) {
                Ok(_) => {
                    log::debug!(
                        "Playing {quote}{}{quote}",
                        audio.get_title().purple(),
                        quote = "\"".purple()
                    );
                    if player.is_queue_empty() {
                        match player.populate_queue() {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("Failed to populate the queue, {}", e);
                                return protocol::Response::ERROR {
                                    message: "Failed to populate the queue".to_string(),
                                };
                            }
                        }
                    }
                    return protocol::Response::PlaybackStarted {
                        title: audio.get_title().to_string(),
                    };
                }
                Err(e) => {
                    log::error!("Failed to play the audio: {}", e);
                    return protocol::Response::ERROR {
                        message: String::from("Failed to play the audio"),
                    };
                }
            };
        }
        protocol::PlayerSubCommand::Next => {
            match player.next_audio() {
                Ok(_) => {
                    let playing_audio = match player.get_current_audio() {
                        Some(out) => out,
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
                    return protocol::Response::Next {
                        playing_audio: playing_audio.to_string(),
                    };
                }
                Err(e) => {
                    log::error!("Failed to advance the playing queue: {}", e);
                    return protocol::Response::ERROR {
                        message: String::from("Failed to advance the playing queue"),
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
            player.clear_queue();
            return protocol::Response::Cleared;
        }
    }
}
