pub mod lib;

fn main() {
    for n in 1000..10000 {
        let base = lib::sum_of_digits(n);
        let (accum, exponent) = lib::check_nth_roots(n, base);
        if n == accum {
            println!("{} ({}^{})", n, base, exponent);
        }
    }
}
