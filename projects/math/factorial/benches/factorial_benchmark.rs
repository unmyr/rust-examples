use criterion::{black_box, criterion_group, criterion_main, Criterion};
use factorial::factorial_recursive_if_else;
use factorial::factorial_recursive_use_match;
use factorial::factorial_iterative;

pub fn bm_factorial(c: &mut Criterion) {
    c.bench_function("factorial_recursive_if_else(20)", |b| b.iter(
        || factorial_recursive_if_else(black_box(20)))
    );

    c.bench_function("factorial_recursive_use_match(20)", |b| b.iter(
        || factorial_recursive_use_match(black_box(20)))
    );

    c.bench_function("factorial_iterative(20)", |b| b.iter(
        || factorial_iterative(black_box(20)))
    );
}

criterion_group!(benches, bm_factorial);
criterion_main!(benches);
