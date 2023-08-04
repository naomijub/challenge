use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::profit::max_profit;

fn criterion_benchmark(c: &mut Criterion) {
    let values = vec![7, 1, 5, 3, 6, 4];
    c.bench_function("test 1", |b| b.iter(|| max_profit(values.clone())));

    let values = vec![7, 6, 4, 3, 1];
    c.bench_function("test 2", |b| b.iter(|| max_profit(values.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
