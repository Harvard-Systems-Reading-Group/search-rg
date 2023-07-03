use search::search;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search", |b| b.iter(|| search::search("test", "test")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);