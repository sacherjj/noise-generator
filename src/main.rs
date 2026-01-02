mod fft;
mod noise;
mod playback;
mod wave;

use clap::Parser;
use std::path::PathBuf;

use crate::noise::{NoiseGenerator, NoiseType};
use crate::playback::AudioSystem;
use crate::wave::write_wav;

#[derive(Parser)]
#[command(name = "Noise Generator")]
#[command(about = "Generate various types of noise audio files", long_about = None)]
struct Args {
    /// Type of noise to generate
    #[arg(value_enum)]
    noise_type: NoiseType,

    /// Output file path (optional if --play or --fft is used)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Play the audio (optional if --output or --fft is used)
    #[arg(short, long)]
    play: bool,

    /// Show FFT of audio (optional if --output or --play is used)
    #[arg(long)]
    fft: bool,

    /// Duration in seconds
    #[arg(short, long, default_value_t = 5.0)]
    duration: f32,

    /// Sample rate in Hz
    #[arg(short, long, default_value_t = 44100)]
    sample_rate: u32,

    /// Custom frequency for gray noise (optional)
    #[arg(short, long)]
    frequency: Option<f32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let audio: Option<AudioSystem> = {
        if args.play {
            // Initialize audio early if used as takes time.
            Some(AudioSystem::init())
        } else {
            None
        }
    };

    // Validate arguments
    if !args.play && !args.fft && args.output.is_none() {
        eprintln!("Error: Either --output, --play, or --fft must be specified");
        std::process::exit(1);
    }

    println!("Generating {:?} noise...", args.noise_type);
    println!("Duration: {:.2}s", args.duration);
    println!("Sample rate: {}Hz", args.sample_rate);
    if let Some(freq) = args.frequency {
        println!("Custom frequency: {}Hz", freq);
    }

    let mut generator = NoiseGenerator::new(args.sample_rate);
    let samples = generator.generate(args.noise_type, args.duration, args.frequency);

    if args.play {
        audio.unwrap().play_audio(&samples, args.sample_rate)?;
    }

    if let Some(output_path) = args.output {
        println!("Writing to {:?}...", output_path);
        write_wav(&output_path, &samples, args.sample_rate)?;
        println!("Done! Generated {} samples.", samples.len());
    }

    if args.fft {
        fft::show_fft(&samples, args.sample_rate);
    }

    Ok(())
}
