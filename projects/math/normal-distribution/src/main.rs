use rand::distr::Distribution;

fn main() {
    let mut rng = rand::rng();
    let (mean, std_dev): (f32, f32) = (0., 0.6);
    let normal_dist = rand_distr::Normal::new(mean, std_dev).unwrap();
    let num_samples = 50;
    let samples: Vec<f32> = normal_dist
        .sample_iter(&mut rng)
        .take(num_samples)
        .collect();

    // Draws an ASCII histogram
    let (x_min, x_max): (f32, f32) = (
        *samples
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
        *samples
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
    );

    let divisions = 10;
    let bin_width = (x_max - x_min) / ((divisions - 1) as f32);
    let mut histogram_bins = vec![0usize; divisions];
    for p in &samples {
        let i = ((p - x_min) / bin_width).round() as usize;
        histogram_bins[i] += 1;
    }

    for i in 0..divisions {
        println!(
            "{:+.2}..{:+.2}: {}",
            x_min + bin_width * (i as f32),
            x_min + bin_width * ((i + 1) as f32),
            "*".repeat(histogram_bins[i])
        );
    }
}
