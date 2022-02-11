use criterion::{criterion_group, criterion_main, Criterion};
use luhnr::generate;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate", |b| b.iter(|| generate(16)));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
