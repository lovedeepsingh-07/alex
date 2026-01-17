use crate::{error, player, protocol::response};

pub(crate) async fn handle(
    player: &mut player::Player,
    search_term: Option<String>,
) -> Result<response::Response, error::Error> {
    match search_term {
        Some(_search_term) => {
            log::warn!("searching with a term is not implemented yet");
            return Ok(response::Response {
                data: vec!["ERROR".to_string(), "SEARCH".to_string()],
            });
        }
        None => {
            log::debug!("Searching for audio files");
            let mut response = response::Response::new();
            response.data.push("OK".to_string());
            response.data.push("SEARCH".to_string());
            for (label, _) in &player.audio_index {
                response.data.push(label.clone());
            }
            return Ok(response);
        }
    }
}
