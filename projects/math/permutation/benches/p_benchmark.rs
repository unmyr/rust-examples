use criterion::{black_box, criterion_group, criterion_main, Criterion};
use permutation::perm_tmp_vec_u32;
use permutation::perm_with_swap_u32;
use permutation::perm_with_swap_gen;
use permutation::perm_iterative_no_gen;
use permutation::perm_iterative_ordered_gen;

pub fn bm_perm(c: &mut Criterion) {
    c.bench_function("perm_tmp_vec_u32(6)", |b| b.iter(
        || {
            let mut result = Vec::<Vec<u32>>::new();
            perm_tmp_vec_u32(
                black_box(vec![1, 2, 4, 3, 4, 5, 6]), &mut vec![], &mut result
            )
        })
    );

    c.bench_function("perm_with_swap_u32(6)", |b| b.iter(
        || {
            let mut result = Vec::<Vec<u32>>::new();
            perm_with_swap_u32(
                black_box(vec![1, 2, 4, 3, 4, 5, 6]),
                black_box(0),
                &mut result
            )
        })
    );

    c.bench_function("perm_with_swap_gen(6)", |b| b.iter(
        || {
            let mut result = Vec::<Vec<u32>>::new();
            perm_with_swap_gen(
                black_box(vec![1, 2, 4, 3, 4, 5, 6]),
                black_box(0),
                &mut result
            )
        })
    );

    c.bench_function("perm_iterative_no_gen(6)",
        |b| b.iter(
            || perm_iterative_no_gen(
                black_box(vec![1, 2, 4, 3, 4, 5, 6])
            )
        )
    );

    c.bench_function("perm_iterative_ordered_gen(6)",
        |b| b.iter(
            || perm_iterative_ordered_gen(
                black_box(vec![1, 2, 4, 3, 4, 5, 6])
            )
        )
    );
}

criterion_group!(benches, bm_perm);
criterion_main!(benches);
