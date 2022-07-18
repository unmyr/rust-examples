/// # Examples
/// ```
/// use primes::primes;
/// assert_eq!(primes(1, 10), vec![2, 3, 5, 7]);
/// ```
pub fn primes(range_from: u32, range_to: u32) -> Vec<u32> {
    let mut v: Vec<u32> = vec![];
    let odd_numbers = match range_from {
        range_from if range_from > 2 => {
            if range_from % 2 == 0 {
                ((range_from + 1)..=range_to).step_by(2)
            } else {
                (range_from..=range_to).step_by(2)
            }
        },
        _ => {
            v.push(2);
            (3..=range_to).step_by(2)
        }
    };

    // find odd primes
    for p in odd_numbers {
        let mut found = true;
        for k in (3..=((p + 1)/2)).step_by(2) {
            if p % k == 0 {
                found = false;
                break;
            }
        }
        if found {
            v.push(p);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        assert_eq!(primes(1, 10), vec![2, 3, 5, 7]);
        assert_eq!(primes(2, 10), vec![2, 3, 5, 7]);
        assert_eq!(primes(3, 10), vec![3, 5, 7]);
        assert_eq!(primes(4, 10), vec![5, 7]);
    }
}
