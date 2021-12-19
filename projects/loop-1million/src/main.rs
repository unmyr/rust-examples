use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut total = 0;
    for _ in 0..(10i64.pow(6)) {
        total += 1;
    }
    let duration = start.elapsed();

    println!(
        "Total: {}, Duration: {:?}",
        total, duration
    );
}
