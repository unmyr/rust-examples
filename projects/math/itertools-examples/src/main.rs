use itertools::Itertools;

fn main() {
    let n = 11;
    println!("P({},{})", n, n);
    let mut i = 0;
    let max_iter = (1..=n).fold(1, |acc, v| acc * v);
    for perm in (1..=n).permutations(n) {
        if (i+1) % (max_iter/3) == 0 {
            println!("[{:10}]{:?}", i, perm);
        }
        i += 1;
    }
}
