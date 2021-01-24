#[macro_use]
extern crate criterion;
extern crate verifier;

use criterion::Criterion;
use verifier::{fast_fibonacci, slow_fibonacci};

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 8",
                     |b| b.iter(|| slow_fibonacci(8)));
}

criterion_group!(fib_bench, fibonacci_benchmark);
criterion_main!(fib_bench);