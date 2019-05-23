#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use nsubset::{f, f_iter};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rec f 5 3", |b| {
        b.iter(|| f(black_box(5), black_box(3), black_box(6)))
    });
    c.bench_function("iter f 5 3", |b| {
        b.iter(|| f_iter(black_box(5), black_box(3), black_box(6)))
    });
    c.bench_function("iter f 148 17", |b| {
        b.iter(|| f_iter(black_box(67), black_box(20), black_box(99)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
