pub fn sum_of_digits(n: i32) -> i32 {
    let mut sum = 0;
    let mut divisor = n;
    while divisor != 0 {
        sum += divisor % 10;
        divisor /= 10;
    }
    sum
}

pub fn check_nth_roots(n: i32, base: i32) -> (i32, i32) {
    match base {
        1 => {
            (1, 1)
        }
        _ => {
            let mut exponent = 1;
            let mut accum = base;
            while accum<n {
                accum *= base;
                exponent += 1;
            }
            (accum, exponent)
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sum_of_digits() {
        assert_eq!(sum_of_digits(0), 0);
        assert_eq!(sum_of_digits(1), 1);
        assert_eq!(sum_of_digits(1000), 1);
        assert_eq!(sum_of_digits(1234), 10);
    }

    #[test]
    fn test_check_nth_roots() {
        assert_eq!(check_nth_roots(1, 1), (1, 1));
        assert_eq!(check_nth_roots(1000, 1), (1, 1));
        assert_eq!(check_nth_roots(2401, 7), (2401, 4));
        assert_eq!(check_nth_roots(4913, 17), (4913, 3));
        assert_eq!(check_nth_roots(5832, 18), (5832, 3));
    }
}