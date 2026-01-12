use crate::{daemon::player, error};
use colored::Colorize;

pub fn handle(player: &mut player::Player, search_term: Option<String>) -> Result<(), error::Error> {
    match search_term {
        Some(search_term) => {
            println!("searching with term: {}", search_term);
        },
        None => {
            for (label, _) in &player.audio_index {
                println!("{} {}", "|".blue(), label);
            }
        }
    }
    // println!("{} {}", "> playing".green(), audio_label.blue());
    Ok(())
}
