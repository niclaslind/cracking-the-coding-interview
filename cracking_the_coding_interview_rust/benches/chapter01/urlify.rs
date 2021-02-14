use criterion::{Criterion, criterion_group};
use cracking_the_coding_interview_rust::chapter01::urlify;

fn replace_spaces_benchmark(c: &mut Criterion) {
    c.bench_function("Is unique chars bit", |b|
        b.iter(|| urlify::replace_spaces("Mr John Smith          ", 11)));
}

criterion_group!(replace_spaces_benchmarks, replace_spaces_benchmark);