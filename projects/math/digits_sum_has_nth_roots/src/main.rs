fn main() {
    for n in 1000..10000 {
        let base = digits_sum_has_nth_roots::sum_of_digits(n);
        let (accum, exponent) = digits_sum_has_nth_roots::check_nth_roots(n, base);
        if n == accum {
            println!("{} ({}^{})", n, base, exponent);
        }
    }
}
