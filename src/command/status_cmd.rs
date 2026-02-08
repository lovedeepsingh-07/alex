use crate::{
    command, error, player, protocol,
};

pub async fn handle(
    player: &mut player::Player,
    sub_command: Option<command::StatusSubCommand>,
) -> Result<protocol::Response, error::Error> {
    let mut status_sub_command: Option<protocol::R_StatusSubCommand> = None;

    if let Some(sub_command) = sub_command {
        match sub_command {
            command::StatusSubCommand::CurrentAudio => {
                let mut output = None;
                if let Some(current_audio) = &player.state.current_audio {
                    output = Some(current_audio.to_string());
                }
                status_sub_command = Some(protocol::R_StatusSubCommand::CurrentAudio { output });
            }
            command::StatusSubCommand::IsPaused => {
                status_sub_command = Some(protocol::R_StatusSubCommand::IsPaused {
                    output: player.state.is_paused.to_string(),
                });
            }
            command::StatusSubCommand::IsQueueEmpty => {
                status_sub_command = Some(protocol::R_StatusSubCommand::IsQueueEmpty {
                    output: player.state.is_queue_empty.to_string(),
                });
            }
        }
    } else {
        status_sub_command = Some(protocol::R_StatusSubCommand::ALL {
            output: serde_json::to_string(&player.state)?,
        });
    }

    let status_sub_command = match status_sub_command {
        Some(out) => out,
        None => {
            return Err(error::Error::ProtocolError("Failed to somehow get status sub command out of an option".to_string()));
        }
    };
    let status_command = protocol::R_Command::Status { sub_command: status_sub_command };
    return Ok(protocol::Response {
        data: Vec::new(),
        packet: bitcode::encode(&protocol::R_Packet {
            result: protocol::R_Result::OK,
            command: status_command,
        })
    });
}
