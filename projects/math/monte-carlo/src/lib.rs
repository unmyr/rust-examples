use rand::Rng;
use std::f64::consts::PI;

/// # Examples
/// ```
/// use monte_carlo::calc_pi;
/// let seed: [u8; 32] = [13; 32];
/// let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
/// assert_eq!((calc_pi(1000, &mut rng) - 3.141).abs() < 0.11, true);
/// ```
pub fn calc_pi(n_samples: u64, rng: &mut rand::rngs::StdRng) -> f64 {
    let mut inside_area_count = 0;

    // Area D: x^2 + y^2 < 1
    for _ in 1..=n_samples {
        let x: f64 = rng.random_range(0.0..1.0);
        let y: f64 = rng.random_range(0.0..1.0);
        // Point (x, y) in inside area D
        if x * x + y * y < 1. {
            inside_area_count += 1;
        }
    }
    let pi_estimate = 4. * (inside_area_count as f64) / (n_samples as f64);
    let error = (pi_estimate - PI).abs();
    println!("calc_pi({}): error={}", n_samples, error);
    pi_estimate
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_4;

    use super::*;

    #[test]
    fn test_calc_pi() {
        let p = FRAC_PI_4; // true probability
        let n_samples = 1000;
        let sigma = 4.0 * (p * (1.0 - p) / (n_samples as f64)).sqrt();

        let seed: [u8; 32] = [13; 32];
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
        let error = (calc_pi(n_samples, &mut rng) - PI).abs();

        // verify within 2 sigma (95% confidence interval)
        assert_eq!(error < 2.0 * sigma, true);
    }
}
