use crate::{player, protocol};

pub async fn handle(
    player: &mut player::Player,
    search_term: Option<String>,
) -> protocol::Response {
    match search_term {
        Some(search_term) => {
            let mut search_results: Vec<String> = Vec::new();
            let keywords = tokenize_string(search_term.as_str());

            for (slug, _) in player.index.iter() {
                for curr_keyword in keywords.iter() {
                    if slug.contains(curr_keyword.as_str()) && !search_results.contains(slug) {
                        search_results.push(slug.clone());
                    }
                }
            }

            return protocol::Response::SearchResults(search_results);
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

fn tokenize_string(input: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut push_string = String::new();

    let input_chars_iter = input.chars();

    for c in input_chars_iter {
        match c {
            'a'..'z' | 'A'..'Z' | '0'..'9' => {
                push_string.push(c);
            }
            _ => {
                if push_string.trim().len() != 0 {
                    output.push(push_string);
                }
                push_string = String::new();
            }
        }
    }
    if push_string.trim().len() != 0 {
        output.push(push_string)
    }

    output
}
