use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use necklace_permutations::necklace_perm_with_filter;
use necklace_permutations::necklace_perm;

pub fn bm_np(c: &mut Criterion) {
    c.bench_function("necklace_perm_with_filter(11)", |b| b.iter(
        || necklace_perm_with_filter(black_box(
            vec![1, 2, 4, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        )))
    );

    c.bench_function("necklace_perm_iterative(11)", |b| b.iter(
        || necklace_perm(black_box(
            vec![1, 2, 4, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        )))
    );
}

criterion_group!(benches, bm_np);
criterion_main!(benches);
