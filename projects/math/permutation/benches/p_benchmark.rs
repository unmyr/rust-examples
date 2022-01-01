use criterion::{black_box, criterion_group, criterion_main, Criterion};
use permutation::v1_recursive_ordered_u32::gen_perm_with_tmp_vec_u32;
use permutation::v2_recursive_unordered_u32::gen_perm_with_depth as gen_perm_with_depth_u32;
use permutation::v3_recursive_unordered_gen::gen_perm_with_depth as gen_perm_with_depth_gen;
use permutation::v4_iterative_unordered::gen_perm as gen_perm_unordered;
use permutation::v5_iterative_ordered::gen_perm as gen_perm_ordered;

pub fn bm_perm(c: &mut Criterion) {
    c.bench_function("v1 recursive tmp_vec u32(6)", |b| b.iter(
        || {
            let mut result = Vec::<Vec<u32>>::new();
            gen_perm_with_tmp_vec_u32(
                black_box(vec![1, 2, 4, 3, 4, 5, 6]), &mut vec![], &mut result
            )
        })
    );

    c.bench_function("v2 recursive swap u32(6)", |b| b.iter(
        || {
            let mut result = Vec::<Vec<u32>>::new();
            gen_perm_with_depth_u32(
                black_box(vec![1, 2, 4, 3, 4, 5, 6]),
                black_box(0),
                &mut result
            )
        })
    );

    c.bench_function("v3 recursive swap generics(6)", |b| b.iter(
        || {
            let mut result = Vec::<Vec<u32>>::new();
            gen_perm_with_depth_gen(
                black_box(vec![1, 2, 4, 3, 4, 5, 6]),
                black_box(0),
                &mut result
            )
        })
    );

    c.bench_function("v4 iterative unordered(6)",
        |b| b.iter(
            || gen_perm_unordered(
                black_box(vec![1, 2, 4, 3, 4, 5, 6])
            )
        )
    );

    c.bench_function("v5 iterative ordered sorting(6)",
        |b| b.iter(
            || gen_perm_ordered(
                black_box(vec![1, 2, 4, 3, 4, 5, 6])
            )
        )
    );
}

criterion_group!(benches, bm_perm);
criterion_main!(benches);
