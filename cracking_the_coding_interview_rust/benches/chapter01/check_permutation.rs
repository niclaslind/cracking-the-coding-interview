use criterion::{Criterion, criterion_group};
use cracking_the_coding_interview_rust::chapter01::check_permutation;

fn permutation_benchmark(c: &mut Criterion) {
    c.bench_function("Check permutation", |b|
        b.iter(|| check_permutation::check_permutation("hello", "olleh")));
}

fn permutation_efficient_benchmark(c: &mut Criterion) {
    c.bench_function("Check permutation efficient", |b|
        b.iter(|| check_permutation::check_permutation_efficient("hello", "olleh")));
}

criterion_group!(permuatation_bencmarks, permutation_benchmark, permutation_efficient_benchmark);
