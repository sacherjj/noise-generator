use rasciichart::plot_sized;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

pub fn show_fft(noise: &Vec<f32>, sample_rate: u32) {
    let len = noise.len();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(len);

    let mut buffer: Vec<Complex<f32>> = noise
        .into_iter()
        .map(|&x| Complex { re: x, im: 0.0 })
        .collect();
    fft.process(&mut buffer);

    // Calculate magnitudes and normalize
    let magnitudes: Vec<f32> = buffer[0..=len / 2]
        .into_iter()
        .map(|c| (c.re * c.re + c.im * c.im).sqrt() * 1.0 / len as f32)
        .collect();

    const GRAPH_WIDTH: usize = 60;
    let chunk_size = magnitudes.len() / GRAPH_WIDTH;
    let mut max_bin: f64 = 0.0;

    let linear_bins = {
        let mut smaller_bins: Vec<f64> = Vec::new();
        for chunks in magnitudes.chunks(chunk_size) {
            let bin_mag = chunks.iter().map(|&n| n as f64).sum();
            if bin_mag > max_bin {
                max_bin = bin_mag;
            }
            smaller_bins.push(bin_mag);
        }
        smaller_bins
    };

    println!("\nLinear FFT:");
    println!("{}", plot_sized(&linear_bins, 20, GRAPH_WIDTH));
    println!("Frequency 0 to {} - linear", sample_rate / 2);
}
