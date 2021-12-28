/// # Examples
/// ```
/// use factorial::factorial_recursive_if_else as factorial;
/// assert_eq!(factorial(3), 6);
/// ```
pub fn factorial_recursive_if_else(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        n * factorial_recursive_if_else(n - 1)
    }
}

/// # Examples
/// ```
/// use factorial::factorial_recursive_use_match as factorial;
/// assert_eq!(factorial(3), 6);
/// ```
pub fn factorial_recursive_use_match(num: u64) -> u64 {
    match num {
        0 | 1 => 1,
        _ => factorial_recursive_use_match(num - 1) * num,
    }
}

/// # Examples
/// ```
/// use factorial::factorial_iterative as factorial;
/// assert_eq!(factorial(3), 6);
/// ```
pub fn factorial_iterative(num: u64) -> u64 {
    // (1..=num).product()
    (1..=num).fold(1, |acc, v| acc * v)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_factorial_recursive_if_else() {
        assert_eq!(factorial_recursive_if_else(0), 1);
        assert_eq!(factorial_recursive_if_else(4), 24);
    }

    #[test]
    fn test_factorial_recursive_use_match() {
        assert_eq!(factorial_recursive_use_match(0), 1);
        assert_eq!(factorial_recursive_use_match(4), 24);
    }

    #[test]
    fn test_factorial_iterative() {
        assert_eq!(factorial_iterative(0), 1);
        assert_eq!(factorial_iterative(4), 24);
    }
}
