use crate::{command, error, player, protocol::response};

pub(crate) async fn handle(
    player: &mut player::Player,
    player_sub_command: command::PlayerSubCommand,
) -> Result<response::Response, error::Error> {
    let mut response_data: Vec<String> = vec!["OK".to_string(), "PLAYER".to_string()];
    match player_sub_command {
        command::PlayerSubCommand::Play(audio_label) => {
            match player.play(&audio_label) {
                Ok(_) => {
                    log::debug!("Playing {:#?}", audio_label);
                    response_data.push("PLAY".to_string());
                }
                Err(e) => {
                    log::error!("failed to play the song: {}", e.to_string());
                }
            };
        }
        command::PlayerSubCommand::Pause => {
            log::debug!("Pausing playback");
            player.pause();
            response_data.push("PAUSE".to_string());
        }
        command::PlayerSubCommand::Resume => {
            log::debug!("Resuming playback");
            player.resume();
            response_data.push("RESUME".to_string());
        }
        command::PlayerSubCommand::Clear => {
            log::debug!("Clearing player queue");
            player.clear();
            response_data.push("CLEAR".to_string());
        }
    }

    Ok(response::Response {
        data: response_data,
    })
}
