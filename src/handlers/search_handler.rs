use crate::{player, protocol, utils};
use std::collections::HashSet;

pub async fn handle(
    player: &mut player::Player,
    search_term: Option<String>,
) -> protocol::Response {
    match search_term {
        Some(search_term) => {
            log::debug!(
                "Searching for audio files with term: {:#?}",
                search_term.as_str()
            );
            let query_tokens =
                utils::remove_stop_words(utils::tokenize_string(search_term.as_str()));
            let candidates = get_search_candidates(player, &query_tokens);

            let results: Vec<protocol::SearchResult> = candidates
                .iter()
                .filter_map(|slug| {
                    let score = score_audio(slug.as_str(), &search_term, &query_tokens);
                    if score > 0.15 {
                        Some(protocol::SearchResult {
                            slug: slug.clone(),
                            score,
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
            let mut results: Vec<protocol::SearchResult> = Vec::new();
            for (slug, _) in &player.storage.audios {
                results.push(protocol::SearchResult {
                    slug: slug.clone(),
                    score: 0.0,
                });
            }
            return protocol::Response::SearchResults(results);
        }
    }
}

fn get_search_candidates(player: &mut player::Player, query_tokens: &[String]) -> HashSet<String> {
    let mut candidates = HashSet::new();
    for query_tok in query_tokens {
        for (index_tok, audio_slugs) in player.storage.index.iter() {
            if index_tok.starts_with(query_tok.as_str()) {
                candidates.extend(audio_slugs.clone());
                continue;
            }
            if query_tok.len() >= 3 {
                let dist = strsim::levenshtein(query_tok, index_tok);
                let threshold = match query_tok.len() {
                    3..=4 => 1,
                    5..=7 => 2,
                    _ => 3,
                };
                if dist < threshold {
                    candidates.extend(audio_slugs.clone());
                }
            }
        }
    }
    return candidates;
}

fn score_audio(slug: &str, query: &str, query_tokens: &[String]) -> f64 {
    let mut output_score = 0.0;
    let slug_tokens = utils::tokenize_string(slug);

    let full_match = strsim::jaro_winkler(query, slug);
    if full_match > 0.85 {
        output_score += full_match * 2.0;
    }

    for query_tok in query_tokens {
        let best_token_score = slug_tokens
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
