use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct AudioSystem {
    #[allow(dead_code)]
    stream: OutputStream,
    #[allow(dead_code)]
    stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl AudioSystem {
    pub fn init() -> Self {
        println!("Initializing audio system...");
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        AudioSystem {
            stream,
            stream_handle,
            sink,
        }
    }

    pub fn play_audio(
        &self,
        samples: &[f32],
        sample_rate: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SamplesBuffer::new(1, sample_rate, samples.to_vec());
        self.sink.append(buffer);

        println!("Playing audio... Press Ctrl+C to stop.");
        self.sink.sleep_until_end();

        Ok(())
    }
}
