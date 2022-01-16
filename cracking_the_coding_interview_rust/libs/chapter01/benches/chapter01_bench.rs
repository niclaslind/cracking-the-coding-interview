use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lib_chapter01::{
    check_permutation::{check_permutation, check_permutation_efficient},
    is_unique::{is_unique, is_unique_chars_bit},
    one_away::one_edit_away,
    palindrome_permutation::{
        is_permutation_of_palindrome, is_permutation_of_palindrome_bit_vector,
        is_permutation_of_palindrome_optimized,
    },
};

fn check_permutation_benchmark(c: &mut Criterion) {
    c.bench_function("check_permutation", |b| {
        b.iter(|| check_permutation(black_box("cat"), black_box("tac")));
    });
}

fn check_permutation_efficient_benchmark(c: &mut Criterion) {
    c.bench_function("check_permutation_efficient", |b| {
        b.iter(|| check_permutation_efficient(black_box("cat"), black_box("tac")));
    });
}

fn is_unique_benchmark(c: &mut Criterion) {
    c.bench_function("is_unique", |b| {
        b.iter(|| is_unique(black_box("Hej")));
    });
}

fn is_unique_chars_bit_benchmark(c: &mut Criterion) {
    c.bench_function("is_unique_chars_bit", |b| {
        b.iter(|| is_unique_chars_bit(black_box("Hej")));
    });
}

fn one_edit_away_benchmark(c: &mut Criterion) {
    c.bench_function("one_edit_away", |b| {
        b.iter(|| one_edit_away(black_box("pale"), black_box("ple")));
    });
}

fn is_permutation_of_palindrome_benchmark(c: &mut Criterion) {
    c.bench_function("is_permutation_of_palindrome", |b| {
        b.iter(|| is_permutation_of_palindrome(black_box("Tact toa")));
    });
}

fn is_permutation_of_palindrome_optimized_benchmark(c: &mut Criterion) {
    c.bench_function("is_permutation_of_palindrome_optimized", |b| {
        b.iter(|| is_permutation_of_palindrome_optimized(black_box("Tact toa")));
    });
}

fn is_permutation_of_palindrome_bit_vector_benchmark(c: &mut Criterion) {
    c.bench_function("is_permutation_of_palindrome_bit_vector", |b| {
        b.iter(|| is_permutation_of_palindrome_bit_vector(black_box("Tact toa")));
    });
}

criterion_group!(
    check_permutation_group,
    check_permutation_benchmark,
    check_permutation_efficient_benchmark
);

criterion_group!(
    is_unique_group,
    is_unique_benchmark,
    is_unique_chars_bit_benchmark
);

criterion_group!(one_away_group, one_edit_away_benchmark);

criterion_group!(
    is_permutation_of_palindrome_group,
    is_permutation_of_palindrome_benchmark,
    is_permutation_of_palindrome_optimized_benchmark,
    is_permutation_of_palindrome_bit_vector_benchmark,
);

criterion_main!(
    check_permutation_group,
    is_unique_group,
    one_away_group,
    is_permutation_of_palindrome_group,
);
