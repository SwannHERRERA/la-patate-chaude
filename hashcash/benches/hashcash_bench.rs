use criterion::{black_box, Criterion, criterion_group, criterion_main};

use hashcash::hashcash::{HASH_CASH, Hashcash};

fn criterion_benchmark(c: &mut Criterion) {
  let message = "hello world".to_string();
  let complexity = 20;
  c.bench_function(
    "bench hashcash solving",
    |b| b.iter(|| HASH_CASH.solve(black_box(message.clone()), black_box(complexity))),
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);