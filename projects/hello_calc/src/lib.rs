pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

pub fn factorial_recursive_if_else(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        n * factorial_recursive_if_else(n - 1)
    }
}

pub fn factorial_recursive_use_match(num: u64) -> u64 {
    match num {
        0 | 1 => 1,
        _ => factorial_recursive_use_match(num - 1) * num,
    }
}

pub fn factorial_iterative(num: u64) -> u64 {
    // (1..=num).product()
    (1..=num).fold(1, |acc, v| acc * v)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[ignore]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }

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
