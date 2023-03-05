use criterion::{criterion_group, criterion_main, Criterion};
use luhnr::{generate, generate_str};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate", |b| b.iter(|| generate(16)));
    c.bench_function("generate_str", |b| b.iter(|| generate_str(16)));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
