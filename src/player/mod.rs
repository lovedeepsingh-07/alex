pub(crate) mod indexer;

use crate::error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct PlayerState {
    pub(crate) current_audio: Option<String>,
    pub(crate) is_paused: bool,
    pub(crate) is_queue_empty: bool,
}

pub(crate) struct Player {
    pub(crate) state: PlayerState,
    #[allow(dead_code)]
    output: rodio::OutputStream,
    sink: rodio::Sink,
    pub(crate) index: indexer::AudioIndex,
}

impl Player {
    pub(crate) fn new() -> Result<Self, error::Error> {
        let state = PlayerState {
            current_audio: None,
            is_paused: false,
            is_queue_empty: true,
        };

        let output = rodio::OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(&output.mixer());
        let index = indexer::index_audio_files()?;

        Ok(Player {
            output,
            sink,
            index,
            state,
        })
    }
    pub(crate) fn update_state(&mut self) -> Result<(), error::Error> {
        self.state.is_queue_empty = self.sink.empty();
        if self.state.is_queue_empty {
            self.state.current_audio = None;
        }
        Ok(())
    }
    pub(crate) fn play(&mut self, audio_label: &str) -> Result<(), error::Error> {
        let audio = self.index.get(audio_label).ok_or_else(|| {
            error::Error::NotFoundError("The requested audio filese does not exist".to_string())
        })?;

        let audio_path = audio.path.clone();
        let audio_file = std::fs::File::open(&audio_path)?;
        let audio_source = rodio::Decoder::builder().with_data(audio_file).build()?;

        self.clear();
        self.sink.append(audio_source);
        self.resume();
        self.state.current_audio = Some(audio_label.to_string());
        Ok(())
    }
    pub(crate) fn resume(&mut self) {
        if !self.state.is_paused {
            return;
        }
        self.state.is_paused = false;
        self.sink.play();
    }
    pub(crate) fn pause(&mut self) {
        if self.state.is_paused {
            return;
        }
        self.state.is_paused = true;
        self.sink.pause();
    }
    pub(crate) fn clear(&mut self) {
        if self.state.is_queue_empty {
            return;
        }
        self.state.is_queue_empty = true;
        self.sink.stop();
    }
    pub(crate) fn reload(&mut self) -> Result<(), error::Error> {
        self.index = indexer::index_audio_files()?;
        Ok(())
    }
}
