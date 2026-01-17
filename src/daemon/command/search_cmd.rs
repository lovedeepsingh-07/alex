use crate::{daemon::player, error, response};
use tokio::sync::mpsc;

pub async fn handle(
    response_tx: mpsc::Sender<response::Response>,
    player: &mut player::Player,
    search_term: Option<String>,
) -> Result<(), error::Error> {
    match search_term {
        Some(_search_term) => {
            log::warn!("searching with a term is not implemented yet");
            response_tx
                .send(response::Response {
                    data: vec!["ERROR".to_string(), "SEARCH".to_string()],
                })
                .await?;
        }
        None => {
            let mut response = response::Response::new();
            response.data.push("OK".to_string());
            response.data.push("SEARCH".to_string());
            for (label, _) in &player.audio_index {
                response.data.push(label.clone());
            }
            response_tx.send(response).await?;
        }
    }
    Ok(())
}
