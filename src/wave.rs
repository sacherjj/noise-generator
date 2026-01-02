use hound::{WavSpec, WavWriter};
use std::path::PathBuf;

pub fn write_wav(
    path: &PathBuf,
    samples: &[f32],
    sample_rate: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(path, spec)?;

    for &sample in samples {
        let amplitude = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        writer.write_sample(amplitude)?;
    }

    writer.finalize()?;
    Ok(())
}
