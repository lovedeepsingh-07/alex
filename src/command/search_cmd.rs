use crate::{error, player, protocol};

pub async fn handle(
    player: &mut player::Player,
    search_term: Option<String>,
) -> Result<protocol::Response, error::Error> {
    match search_term {
        Some(search_term) => {
            let _ = search_term;
            log::warn!("searching with a term is not implemented yet");

            return Ok(protocol::Response {
                data: Vec::new(),
                packet: bitcode::encode(&protocol::R_Packet{
                    result: protocol::R_Result::ERROR {
                        error_message: String::from("Searching with a term is not implemented yet")
                    },
                    command: protocol::R_Command::Search {
                        search_result: Vec::new(),
                    }
                }),
            });
        }
        None => {
            log::debug!("Searching for audio files");
            let search_result = player.index.iter().map(|(label, _)|  label.clone()).collect::<Vec<String>>();

            return Ok(protocol::Response {
                data: Vec::new(),
                packet: bitcode::encode(&protocol::R_Packet {
                    result: protocol::R_Result::OK,
                    command: protocol::R_Command::Search {
                        search_result,
                    }
                })
            });
        }
    }
}
