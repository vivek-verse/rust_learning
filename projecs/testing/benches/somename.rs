use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::sploosh;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sploosh", |b| {
        b.iter(|| sploosh(black_box(1), black_box(2), black_box(3)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
