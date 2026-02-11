use crate::{player, protocol};

pub async fn handle(
    player: &mut player::Player,
    search_term: Option<String>,
) -> protocol::Response {
    match search_term {
        Some(search_term) => {
            let _ = search_term;
            log::warn!("searching with a term is not implemented yet");

            return protocol::Response::ERROR {
                message: "Searching with a term is not implemented yet".to_string(),
            };
        }
        None => {
            log::debug!("Searching for audio files");
            let search_results = player
                .index
                .iter()
                .map(|(label, _)| label.clone())
                .collect::<Vec<String>>();

            return protocol::Response::SearchResults(search_results);
        }
    }
}
