/// # Examples
/// ```
/// use collatz::collatz_m;
/// assert_eq!(collatz_m(3), vec![3, 10, 5, 16, 8, 4, 2, 1]);
/// ```
pub fn collatz_m(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![1];
    }

    let mut v: Vec<u64> = vec![n];
    match n % 2 {
        0 => v.append(&mut collatz_m(n / 2)),
        _ => v.append(&mut collatz_m(n * 3 + 1))
    }
    v
}

/// # Examples
/// ```
/// use collatz::collatz_im;
/// assert_eq!(collatz_im(3), vec![3, 10, 5, 16, 8, 4, 2, 1]);
/// ```
pub fn collatz_im(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![1];
    }

    match n % 2 {
        0 => vec![n].into_iter().chain(collatz_im(n / 2)).collect(),
        _ => vec![n].into_iter().chain(collatz_im(n * 3 + 1)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_m() {
        assert_eq!(collatz_m(1), vec![1]);
        assert_eq!(collatz_m(2), vec![2, 1]);
        assert_eq!(collatz_m(3), vec![3, 10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(collatz_m(4), vec![4, 2, 1]);
        assert_eq!(collatz_m(5), vec![5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_collatz_im() {
        assert_eq!(collatz_im(1), vec![1]);
        assert_eq!(collatz_im(2), vec![2, 1]);
        assert_eq!(collatz_im(3), vec![3, 10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(collatz_im(4), vec![4, 2, 1]);
        assert_eq!(collatz_im(5), vec![5, 16, 8, 4, 2, 1]);
    }
}
