use rand::{thread_rng, Rng};

/// # Examples
/// ```
/// use monte_carlo::calc_pi;
/// assert_eq!((calc_pi(100) - 3.14).abs() < 0.1, true);
/// ```
pub fn calc_pi(n: u64) -> f64 {
    let mut rng = thread_rng();
    let mut success = 0;

    for _ in 1..=n {
        let x: f64 = rng.gen_range(0.0 .. 1.0);
        let y: f64 = rng.gen_range(0.0 .. 1.0);
        if x * x + y * y < 1. {
            success += 1;
        }
    }
    4. * (success as f64) / (n as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_pi() {
        assert_eq!((calc_pi(100) - 3.14).abs() < 0.1, true);
    }
}
