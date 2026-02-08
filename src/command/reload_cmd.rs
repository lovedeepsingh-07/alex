use crate::{error, player, protocol};

pub async fn handle(player: &mut player::Player) -> Result<protocol::Response, error::Error> {
    log::debug!("Reloading player audio index");
    player.reload()?;

    Ok(protocol::Response {
        data: Vec::new(),
        packet: bitcode::encode(&protocol::R_Packet {
            result: protocol::R_Result::OK,
            command: protocol::R_Command::Reload,
        }),
    })
}
