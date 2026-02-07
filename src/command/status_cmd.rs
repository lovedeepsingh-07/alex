use crate::{
    command, error, flatbuffers_gen::response_packet_ as response_packet, player, protocol,
};

pub async fn handle(
    player: &mut player::Player,
    sub_command: Option<command::StatusSubCommand>,
) -> Result<protocol::Response, error::Error> {
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let mut status_args = response_packet::StatusArgs::default();

    if let Some(sub_command) = sub_command {
        match sub_command {
            command::StatusSubCommand::CurrentAudio => {
                status_args.sub_command = Some(response_packet::StatusSubCommand::CurrentAudio);
                if let Some(current_audio) = &player.state.current_audio {
                    status_args.output = Some(builder.create_string(current_audio));
                }
            }
            command::StatusSubCommand::IsPaused => {
                status_args.sub_command = Some(response_packet::StatusSubCommand::IsPaused);
                status_args.output =
                    Some(builder.create_string(player.state.is_paused.to_string().as_str()));
            }
            command::StatusSubCommand::IsQueueEmpty => {
                status_args.sub_command = Some(response_packet::StatusSubCommand::IsQueueEmpty);
                status_args.output =
                    Some(builder.create_string(player.state.is_queue_empty.to_string().as_str()));
            }
        }
    } else {
        status_args.sub_command = None;
        status_args.output =
            Some(builder.create_string(serde_json::to_string(&player.state)?.as_str()));
    }

    let ok_result = response_packet::OK::create(&mut builder, &response_packet::OKArgs {});
    let status_command = response_packet::Status::create(&mut builder, &status_args);
    let response_packet = response_packet::ResponsePacket::create(
        &mut builder,
        &response_packet::ResponsePacketArgs {
            result_type: response_packet::ResponseResult::OK,
            result: Some(ok_result.as_union_value()),
            command_type: response_packet::Command::Status,
            command: Some(status_command.as_union_value()),
        },
    );
    builder.finish(response_packet, None);

    Ok(protocol::Response {
        data: Vec::new(),
        packet: builder.finished_data().to_vec(),
    })
}
