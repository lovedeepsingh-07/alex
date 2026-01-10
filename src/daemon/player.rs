use crate::error;
use std::collections::HashMap;

type AudioIndex = HashMap<String, std::path::PathBuf>;

pub struct Player {
    #[allow(dead_code)]
    output_stream: rodio::OutputStream,
    sink: rodio::Sink,
    pub audio_index: AudioIndex,
    #[allow(dead_code)]
    curr_audio: Option<std::path::PathBuf>,
}

pub fn index_audio_files() -> Result<AudioIndex, error::Error> {
    let mut audio_index: AudioIndex = HashMap::new();

    let home_dir = std::env::home_dir().unwrap();
    let music_folder_path = home_dir.join("Music");
    for entry in walkdir::WalkDir::new(music_folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry = entry.path();
        if entry.is_file() {
            let file_name = entry.file_name().unwrap().to_string_lossy().to_string();
            let extension = entry.extension().unwrap().to_string_lossy().to_string();
            let label = file_name.strip_suffix(&format!(".{}", extension)).unwrap();
            audio_index.insert(label.to_string(), entry.to_path_buf());
        };
    }

    Ok(audio_index)
}

impl Player {
    pub fn new() -> Self {
        let output_stream = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(&output_stream.mixer());
        let audio_index = index_audio_files().unwrap();

        Player {
            output_stream,
            sink,
            curr_audio: None,
            audio_index,
        }
    }
    pub fn play(&mut self, audio_label: &str) {
        let audio_path = self.audio_index.get(audio_label).unwrap().clone();
        self.clear();
        let audio_file = std::fs::File::open(&audio_path).unwrap();
        let audio_source = rodio::Decoder::try_from(audio_file).unwrap();
        self.sink.append(audio_source);
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
