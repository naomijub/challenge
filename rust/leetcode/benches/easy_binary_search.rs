use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::binary_search::search;

const TARGET1: i32 = 9;
const TARGET2: i32 = 2;

fn criterion_benchmark(c: &mut Criterion) {
    let nums: Vec<i32> = vec![-1, 0, 3, 5, 9, 12];
    c.bench_function("test target 9", |b| {
        b.iter(|| search(nums.clone(), TARGET1))
    });

    c.bench_function("test target 2", |b| {
        b.iter(|| search(nums.clone(), TARGET2))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
