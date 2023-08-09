use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::remove_element::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simple", |b| {
        b.iter(|| {
            let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
            remove_element(&mut nums, 2)
        })
    });

    c.bench_function("for_loop", |b| {
        b.iter(|| {
            let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
            remove_element_for(&mut nums, 2)
        })
    });

    c.bench_function("fp", |b| {
        b.iter(|| {
            let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
            remove_element_fp(&mut nums, 2)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
