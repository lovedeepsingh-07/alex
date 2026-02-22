use crate::{error, player};

#[allow(dead_code)]
pub struct Engine {
    output_stream: rodio::OutputStream,
    sink: rodio::Sink,
}
impl std::fmt::Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("output_stream", &"OutputStream")
            .field("sink", &"Sink")
            .finish()
    }
}

impl Engine {
    pub fn new() -> Result<Self, error::Error> {
        let mut output_stream = rodio::OutputStreamBuilder::open_default_stream()?;
        output_stream.log_on_drop(false);
        let sink = rodio::Sink::connect_new(&output_stream.mixer());

        Ok(Self {
            output_stream,
            sink,
        })
    }
    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }
    pub fn is_sink_empty(&self) -> bool {
        self.sink.empty()
    }
    pub fn play(&mut self, audio: &player::Audio) -> Result<(), error::Error> {
        let audio_file = std::fs::File::open(audio.get_path())?;
        let audio_source = rodio::Decoder::builder().with_data(audio_file).build()?;
        self.clear_sink();
        self.sink.append(audio_source);
        self.resume();
        Ok(())
    }
    pub fn pause(&mut self) {
        self.sink.pause();
    }
    pub fn resume(&mut self) {
        self.sink.play();
    }
    pub fn clear_sink(&mut self) {
        self.sink.clear();
    }
}
