use criterion::{Criterion, criterion_group};
use cracking_the_coding_interview_rust::chapter01::is_unique::{is_unique, is_unique_chars_bit};

fn is_unique_benchmark(c: &mut Criterion) {
    c.bench_function("Is unique", |b|
        b.iter(|| is_unique("hello")));
}

fn is_unique_chars_bit_benchmark(c: &mut Criterion) {
    c.bench_function("Is unique chars bit", |b|
        b.iter(|| is_unique_chars_bit("hello")));
}

criterion_group!(is_unique_benchmarks, is_unique_benchmark, is_unique_chars_bit_benchmark);