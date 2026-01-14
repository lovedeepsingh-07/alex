use crate::{daemon::player, error};

pub fn handle(player: &mut player::Player) -> Result<(), error::Error> {
    log::info!("reloading player");
    player.index_audio_files()?;
    Ok(())
}
