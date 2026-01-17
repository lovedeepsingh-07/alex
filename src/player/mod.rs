pub mod index;

use crate::error;
use std::collections::HashMap;

pub struct Player {
    #[allow(dead_code)]
    output_stream: rodio::OutputStream,
    sink: rodio::Sink,
    pub audio_index: index::AudioIndex,
    #[allow(dead_code)]
    curr_audio: Option<std::path::PathBuf>,
}

impl Player {
    pub fn new() -> Result<Self, error::Error> {
        let output_stream = rodio::OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(&output_stream.mixer());
        let mut player = Player {
            output_stream,
            sink,
            curr_audio: None,
            audio_index: HashMap::new(),
        };
        player.index_audio_files()?;
        Ok(player)
    }
    pub fn index_audio_files(&mut self) -> Result<(), error::Error> {
        self.audio_index = index::index_audio_files()?;
        Ok(())
    }
    pub fn play(&mut self, audio_label: &str) -> Result<(), error::Error> {
        let audio_path = self
            .audio_index
            .get(audio_label)
            .ok_or_else(|| {
                error::Error::NotFoundError("the requested audio filese does not exist".to_string())
            })?
            .clone();
        self.clear();
        let audio_file = std::fs::File::open(&audio_path)?;
        let audio_source = rodio::Decoder::builder().with_data(audio_file).build()?;
        self.sink.play();
        self.sink.append(audio_source);
        Ok(())
    }
    pub fn clear(&mut self) {
        self.sink.stop();
    }
    pub fn resume(&mut self) {
        self.sink.play();
    }
    pub fn pause(&self) {
        self.sink.pause();
    }
}
