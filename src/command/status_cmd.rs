use crate::{command, error, player, protocol};

pub async fn handle(
    player: &mut player::Player,
    sub_command: Option<command::StatusSubCommand>,
) -> Result<protocol::Response, error::Error> {
    let mut response_data: Vec<String> = vec!["OK".to_string(), "STATUS".to_string()];

    if let Some(sub_command) = sub_command {
        match sub_command {
            command::StatusSubCommand::CurrentAudio => {
                response_data.push("CURRENT_AUDIO".to_string());
                if let Some(current_audio) = &player.state.current_audio {
                    response_data.push(current_audio.to_string());
                }
            }
            command::StatusSubCommand::IsPaused => {
                response_data.push("IS_PAUSED".to_string());
                response_data.push(player.state.is_paused.to_string());
            }
            command::StatusSubCommand::IsQueueEmpty => {
                response_data.push("IS_QUEUE_EMPTY".to_string());
                response_data.push(player.state.is_queue_empty.to_string());
            }
        }
    } else {
        response_data.push("ALL".to_string());
        response_data.push(serde_json::to_string(&player.state).unwrap());
    }

    Ok(protocol::Response {
        data: response_data,
    })
}
