use monte_carlo::calc_pi;
use std::f64::consts::PI;

fn main() {
    let num_trials = vec![
        u64::pow(10, 0),
        u64::pow(10, 1),
        u64::pow(10, 2),
        u64::pow(10, 3),
        u64::pow(10, 4),
        u64::pow(10, 5),
        u64::pow(10, 6),
        u64::pow(10, 7),
    ];
    // let seed: [u8; 32] = [13; 32];
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_os_rng();
    let results = num_trials
        .iter()
        .map(|n| (n, calc_pi(*n, &mut rng)))
        .collect::<Vec<_>>();

    let sigma = |n_samples: u64| {
        let p = std::f64::consts::FRAC_PI_4; // true probability
        4.0 * (p * (1.0 - p) / (n_samples as f64)).sqrt()
    };

    for pair in results {
        let (n, monte_carlo_pi) = pair;
        println!(
            "trial={:10}, result={:.10}, error={:.10}, 2 sigma={:.10}, within_95%_conf={}",
            n,
            monte_carlo_pi,
            (monte_carlo_pi - PI).abs(),
            2.0 * sigma(*n),
            if (monte_carlo_pi - PI).abs() < 2.0 * sigma(*n) {
                "PASS"
            } else {
                "FAIL"
            }
        );
    }
}
