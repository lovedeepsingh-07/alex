use crate::{error, player, protocol};

pub async fn handle(
    player: &mut player::Player,
    search_term: Option<String>,
) -> Result<protocol::Response, error::Error> {
    match search_term {
        Some(_search_term) => {
            log::warn!("searching with a term is not implemented yet");
            return Ok(protocol::Response {
                data: vec!["ERROR".to_string(), "SEARCH".to_string()],
                packet: Vec::new(),
            });
        }
        None => {
            log::debug!("Searching for audio files");
            let mut response = protocol::Response::new();
            response.data.push("OK".to_string());
            response.data.push("SEARCH".to_string());
            for (label, _) in &player.index {
                response.data.push(label.clone());
            }
            return Ok(response);
        }
    }
}
