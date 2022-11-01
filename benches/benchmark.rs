use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pretty_bytes_rust::pretty_bytes;

fn all_benchmarks(c: &mut Criterion) {
    c.bench_function("pretty_bytes with default options", |b| {
        b.iter(|| pretty_bytes(black_box(1), None))
    });
}
criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(2000);
    targets = all_benchmarks
);
criterion_main!(benches);
