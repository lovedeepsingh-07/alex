use crate::{daemon::player, error};
use colored::Colorize;

pub fn handle(player: &mut player::Player) -> Result<(), error::Error> {
    println!("{}", "> reloading player...".green());
    player.index_audio_files()?;
    Ok(())
}
