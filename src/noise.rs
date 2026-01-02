use clap::ValueEnum;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Copy, Clone, PartialEq, Debug, ValueEnum)]
pub enum NoiseType {
    White,
    Pink,
    Brown,
    Blue,
    Gray,
}

pub struct NoiseGenerator {
    sample_rate: u32,
    rng: rand::rngs::ThreadRng,
    // State for colored noise filters
    pink_state: [f32; 7],
    value_prev: f32,
}

impl NoiseGenerator {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            rng: rand::rng(),
            pink_state: [0.0; 7],
            value_prev: 0.0,
        }
    }

    fn white_noise(&mut self) -> f32 {
        self.rng.random_range(-1.0..=1.0)
    }

    fn pink_noise(&mut self) -> f32 {
        // Voss-McCartney algorithm
        // https://www.firstpr.com.au/dsp/pink-noise/
        let white = self.white_noise();

        // Coefficients represent how much filter remembers its previous state
        // higher values are slower decay and lower cutoff frequency
        self.pink_state[0] = 0.99886 * self.pink_state[0] + white * 0.0555179;
        self.pink_state[1] = 0.99332 * self.pink_state[1] + white * 0.0750759;
        self.pink_state[2] = 0.96900 * self.pink_state[2] + white * 0.1538520;
        self.pink_state[3] = 0.86650 * self.pink_state[3] + white * 0.3104856;
        self.pink_state[4] = 0.55000 * self.pink_state[4] + white * 0.5329522;
        self.pink_state[5] = -0.7616 * self.pink_state[5] - white * 0.0168980;

        let pink = self.pink_state[0]
            + self.pink_state[1]
            + self.pink_state[2]
            + self.pink_state[3]
            + self.pink_state[4]
            + self.pink_state[5]
            + self.pink_state[6]
            + white * 0.5362;

        // delayed sample for next iteration
        self.pink_state[6] = white * 0.115926;

        // scaling back to within 1
        pink * 0.11
    }

    fn brown_noise(&mut self) -> f32 {
        // Huge energy at low frequency.
        // brown[n] = brown[n - 1] + c * white[n]

        const SCALING_FACTOR: f32 = 0.02;
        let white = self.white_noise();
        self.value_prev = (self.value_prev + SCALING_FACTOR * white).clamp(-1.0, 1.0);
        self.value_prev * 3.5
    }

    fn blue_noise(&mut self) -> f32 {
        // blue[n] = factor * (white[n] - white[n - 1])
        const WEIGHT_FACTOR: f32 = 0.5;
        let white = self.white_noise();
        let prev = self.value_prev;
        self.value_prev = white;
        (white - prev) * WEIGHT_FACTOR
    }

    fn gray_noise(&mut self, center_freq: Option<f32>) -> Vec<f32> {
        // Gray noise uses equal loudness contours
        // Using bandpass around given frequency
        const CENTER_FREQ_DEFAULT: f32 = 1000.0;
        let freq = center_freq.unwrap_or(CENTER_FREQ_DEFAULT);

        let mut result = Vec::new();
        for i in 0..self.sample_rate {
            let white = self.white_noise();
            // Apply bandpass filter weighted by equal loudness
            let t = i as f32 / self.sample_rate as f32;
            let envelope = (2.0 * PI * freq * t).sin().abs();
            result.push(white * envelope * 0.3);
        }
        result
    }

    pub fn generate(
        &mut self,
        noise_type: NoiseType,
        duration: f32,
        frequency: Option<f32>,
    ) -> Vec<f32> {
        let num_samples = (self.sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        match noise_type {
            NoiseType::Gray => {
                // For gray noise, we'll generate in chunks
                let chunks = (duration.ceil() as u32).max(1);
                for _ in 0..chunks {
                    samples.extend(self.gray_noise(frequency));
                }
                samples.truncate(num_samples);
            }
            _ => {
                for _ in 0..num_samples {
                    let sample = match noise_type {
                        NoiseType::White => self.white_noise(),
                        NoiseType::Pink => self.pink_noise(),
                        NoiseType::Brown => self.brown_noise(),
                        NoiseType::Blue => self.blue_noise(),
                        NoiseType::Gray => self.gray_noise(frequency)[0],
                    };
                    samples.push(sample);
                }
            }
        }

        samples
    }
}
