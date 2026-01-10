use crate::daemon::player;
use colored::Colorize;

#[derive(Debug)]
pub enum Command {
    Reload,
    Search,
    PlayerPlay(String),
    PlayerPause,
    PlayerResume,
    PlayerClear,
}

pub fn handle_command(cmd: Command, player: &mut player::Player) {
    match cmd {
        Command::Reload => {
            log::info!("reloading player...");
        }
        Command::Search => {
            for (label, _) in &player.audio_index {
                println!("{} {}", "|".blue(), label);
            }
        }
        Command::PlayerPlay(audio_label) => {
            log::info!("playing {:#?}...", audio_label);
            // player.play("back_in_black");
        }
        Command::PlayerPause => {
            player.pause();
        }
        Command::PlayerResume => {
            player.resume();
        }
        Command::PlayerClear => {
            player.clear();
        }
    }
}
