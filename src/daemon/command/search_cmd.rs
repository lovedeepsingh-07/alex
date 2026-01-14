use crate::{daemon::player, error, response};

pub fn handle(
    response_sender: crossbeam::channel::Sender::<response::Response>,
    player: &mut player::Player,
    search_term: Option<String>,
) -> Result<(), error::Error> {
    match search_term {
        Some(_search_term) => {
            log::warn!("searching with a term is not implemented yet");
            response_sender.send(response::Response {
                data: vec![
                    "ERROR".to_string(),
                    "SEARCH".to_string(),
                ],
            })?;
        }
        None => {
            let mut response = response::Response::new();
            response.data.push("OK".to_string());
            response.data.push("SEARCH".to_string());
            for (label, _) in &player.audio_index {
                response.data.push(label.clone());
            }
            response_sender.send(response)?;
        }
    }
    Ok(())
}
