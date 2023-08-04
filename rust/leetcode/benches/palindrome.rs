use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::palindrome::is_palindrome;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("is_pal", |b| {
        b.iter(|| is_palindrome("A man, a plan, a canal: Panama"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
