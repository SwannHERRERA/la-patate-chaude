use criterion::{black_box, criterion_group, criterion_main, Criterion};

use recover_secret::challenge_generator::generate_challenge;
use recover_secret::challenge_resolve::solve_secret_string_challenge;

fn criterion_benchmark_1(c: &mut Criterion) {
    c.bench_function("Benchmark recover secret random String", |b| {
        b.iter(|| solve_secret_string_challenge(black_box(&generate_challenge())))
    });
}

criterion_group!(benches, criterion_benchmark_1);
criterion_main!(benches);
