use std::f64::consts::PI;
use rand::{Rng};

/// # Examples
/// ```
/// use monte_carlo::calc_pi;
/// assert_eq!((calc_pi(1000) - 3.141).abs() < 0.11, true);
/// ```
pub fn calc_pi(n_samples: u64) -> f64 {
    let mut rng = rand::rng();
    let mut inside_area_count = 0;

    // Area D: x^2 + y^2 < 1
    for _ in 1..=n_samples {
        let x: f64 = rng.random_range(0.0 .. 1.0);
        let y: f64 = rng.random_range(0.0 .. 1.0);
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
    use super::*;

    #[test]
    fn test_calc_pi() {
        let n_samples = 1000;
        let error = (calc_pi(n_samples) - PI).abs();
        assert_eq!(error < 0.11, true);
    }
}
