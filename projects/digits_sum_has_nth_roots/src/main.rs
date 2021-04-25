fn main() {
    for n in 1000..10000 {
        let mut sum_of_digits = 0;
        let mut divisor = n;
        while divisor != 0 {
            sum_of_digits += divisor % 10;
            divisor /= 10;
        }

        let base = sum_of_digits;
        let mut accum;
        let mut exponent = 1;
        if base == 1 {
            accum = 1;
        } else {
            accum = base;
            while accum<n {
                accum *= base;
                exponent += 1;
            }
        }

        if n == accum {
            println!("{} ({}^{})", n, base, exponent);
        }
    }
}
