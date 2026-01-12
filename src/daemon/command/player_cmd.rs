use crate::{
    daemon::{command, player},
    error,
};
use colored::Colorize;

pub fn handle(
    player: &mut player::Player,
    player_sub_command: command::PlayerSubCommand,
) -> Result<(), error::Error> {
    match player_sub_command {
        command::PlayerSubCommand::Play(audio_label) => {
            match player.play(&audio_label) {
                Ok(_) => {
                    println!("{} {}", "> playing".green(), audio_label.blue());
                }
                Err(e) => {
                    log::error!("failed to play the song: {}", e.to_string());
                }
            };
        }
        command::PlayerSubCommand::Pause => {
            player.pause();
        }
        command::PlayerSubCommand::Resume => {
            player.resume();
        }
        command::PlayerSubCommand::Clear => {
            player.clear();
        }
    }
    Ok(())
}
