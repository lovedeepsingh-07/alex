use crate::{player, protocol};

pub fn handle(player: &mut player::Player) -> protocol::Response {
    let mut status_data = protocol::StatusData {
        current_audio: None,
        is_paused: player.is_paused(),
        is_queue_empty: player.is_queue_empty(),
        queue: Vec::new(),
    };

    status_data.queue = player
        .get_queue()
        .iter()
        .map(|id| {
            let mut out = protocol::DisplayAudio {
                id: id.clone(),
                title: id.clone(),
            };
            if let Ok(audio) = player.storage.get_audio(id) {
                out.title = audio.get_title().to_string()
            }
            out
        })
        .collect();

    if let Some(current_audio_id) = player.get_current_audio() {
        let current_audio_title = match player.storage.get_audio(current_audio_id) {
            Ok(out) => out.get_title().to_string(),
            Err(e) => {
                log::warn!("Failed to get audio from the storage, {}", e);
                return protocol::Response::ERROR {
                    message: "Failed to get audio from the storage".to_string(),
                };
            }
        };
        status_data.current_audio = Some(protocol::DisplayAudio {
            id: current_audio_id.to_string(),
            title: current_audio_title,
        });
    }

    return protocol::Response::StatusData(status_data);
}
