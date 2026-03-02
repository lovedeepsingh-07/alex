use crate::{player, protocol, utils};

pub fn handle(player: &mut player::Player, search_term: Option<String>) -> protocol::Response {
    match search_term {
        Some(search_term) => {
            log::debug!(
                "Searching for audio files with term: {:#?}",
                search_term.as_str()
            );
            let query_tokens = utils::tokenize_string(search_term.as_str());
            let candidates = player.storage.get_search_candidates(&query_tokens);

            let results: Vec<protocol::SearchResult> = candidates
                .iter()
                .filter_map(|id| {
                    let score = score_audio(id.as_str(), &search_term, &query_tokens);
                    if score > 0.15 {
                        let audio = match player.storage.get_audio(id) {
                            Ok(out) => out,
                            Err(e) => {
                                log::warn!("Failed to get audio from the storage, {}", e);
                                return None;
                            }
                        };
                        Some(protocol::SearchResult {
                            id: id.clone(),
                            score,
                            title: audio.get_title().to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect();

            return protocol::Response::SearchResults(results);
        }
        None => {
            log::debug!("Searching for audio files");
            let results: Vec<protocol::SearchResult> = player
                .storage
                .get_audio_map()
                .iter()
                .map(|(id, audio)| protocol::SearchResult {
                    id: id.clone(),
                    score: 0.0,
                    title: audio.get_title().to_string(),
                })
                .collect();
            return protocol::Response::SearchResults(results);
        }
    }
}

fn score_audio(id: &str, query: &str, query_tokens: &[String]) -> f64 {
    let mut output_score = 0.0;
    let id_tokens = utils::tokenize_string(id);

    let full_match = strsim::jaro_winkler(query, id);
    if full_match > 0.85 {
        output_score += full_match * 2.0;
    }

    for query_tok in query_tokens {
        let best_token_score = id_tokens
            .iter()
            .map(|ft| {
                let jw = strsim::jaro_winkler(query_tok, ft);
                if ft == query_tok {
                    jw * 1.5
                } else if ft.starts_with(query_tok.as_str()) {
                    jw * 1.2
                } else {
                    jw
                }
            })
            .fold(0.0_f64, f64::max);
        output_score += best_token_score;
    }

    output_score
}
