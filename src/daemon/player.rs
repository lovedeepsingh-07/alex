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
        let home_dir = std::env::home_dir().ok_or_else(|| {
            error::Error::FSError("Failed to get the home directory path".to_string())
        })?;
        let music_folder_path = home_dir.join("Music");
        for entry in walkdir::WalkDir::new(music_folder_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry = entry.path();
            if entry.is_file() {
                let file_name = entry
                    .file_name()
                    .ok_or_else(|| {
                        error::Error::FSError(
                            "Failed to get the filename for a file entry".to_string(),
                        )
                    })?
                    .to_string_lossy()
                    .to_string();
                let extension = entry
                    .extension()
                    .ok_or_else(|| {
                        error::Error::FSError(
                            "Failed to get the extension for a file entry".to_string(),
                        )
                    })?
                    .to_string_lossy()
                    .to_string();
                if !["ogg"].contains(&extension.as_str()) {
                    continue;
                }
                let label = match file_name.strip_suffix(&format!(".{}", extension)) {
                    Some(out) => out,
                    None => file_name.as_str(),
                };
                self.audio_index
                    .insert(label.to_string(), entry.to_path_buf());
            };
        }
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
