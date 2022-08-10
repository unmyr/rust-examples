
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

    let results = num_trials.iter().map(
        |n| (n, calc_pi(*n))
    ).collect::<Vec<_>>();

    for pair in results {
        let (n, monte_carlo_pi) = pair;
        println!(
            "trial={:10}, result={:.10}, error={:.10}",
            n, monte_carlo_pi, (monte_carlo_pi - PI).abs()
        );
    }
}
