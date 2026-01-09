pub struct Player {
    output_stream: rodio::OutputStream,
    sink: rodio::Sink,
    curr_audio: Option<std::path::PathBuf>,
    pub music_files: Vec<std::path::PathBuf>
}

impl Player {
    pub fn new() -> Self {
        let output_stream = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(&output_stream.mixer());

        let home_dir = std::env::home_dir().unwrap();
        let music_folder_path = home_dir.join("Music");
        let mut music_files: Vec<std::path::PathBuf> = Vec::new();
        for entry in walkdir::WalkDir::new(music_folder_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry = entry.path();
            if entry.is_file() {
                music_files.push(entry.to_path_buf());
            };
        }

        Player {
            output_stream,
            sink,
            curr_audio: None,
            music_files,
        }
    }
    pub fn play(&mut self, audio_path: std::path::PathBuf) {
        self.clear();
        let audio_file = std::fs::File::open(&audio_path).unwrap();
        let audio_source = rodio::Decoder::try_from(audio_file).unwrap();
        self.sink.append(audio_source);
        self.curr_audio = Some(audio_path);
        // player.output_stream.mixer().add(source);
    }
    pub fn clear(&mut self) {
        self.sink.stop();
    }
    pub fn resume(&self) {
        self.sink.play();
    }
    pub fn pause(&self) {
        self.sink.pause();
    }
}
