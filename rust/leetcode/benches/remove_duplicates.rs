use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::remove_duplicates::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simple", |b| {
        b.iter(|| {
            let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
            remove_duplicates(&mut nums)
        })
    });

    c.bench_function("inplace", |b| {
        b.iter(|| {
            let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
            remove_duplicates_inplace(&mut nums)
        })
    });

    c.bench_function("inplace proc", |b| {
        b.iter(|| {
            let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
            remove_duplicates_inplace_proc(&mut nums)
        })
    });

    c.bench_function("simple double_dup", |b| {
        b.iter(|| {
            let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
            remove_double_duplicates(&mut nums)
        })
    });

    c.bench_function("built_in double_dup", |b| {
        b.iter(|| {
            let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
            remove_double_duplicates_builtin(&mut nums)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
