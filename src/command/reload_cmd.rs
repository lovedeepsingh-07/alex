use crate::{error, player, response};
use tokio::sync::mpsc;

pub async fn handle(
    respnose_tx: mpsc::Sender<response::Response>,
    player: &mut player::Player,
) -> Result<(), error::Error> {
    log::info!("reloading player");
    player.index_audio_files()?;
    let mut response = response::Response::new();
    response.data.push("OK".to_string());
    response.data.push("SEARCH".to_string());
    Ok(())
}
