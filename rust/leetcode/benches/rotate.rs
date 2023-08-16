use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::rotate::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    c.bench_function("std", |b| b.iter(|| rotate(&mut v, 3)));

    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    c.bench_function("reverse", |b| b.iter(|| rotate_reversing(&mut v, 3)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
