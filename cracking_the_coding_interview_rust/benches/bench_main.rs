use criterion::criterion_main;

mod chapter01;
criterion_main! {
    chapter01::check_permutation::permuatation_bencmarks,
    chapter01::is_unique::is_unique_benchmarks,
    chapter01::urlify::replace_spaces_benchmarks,
}
