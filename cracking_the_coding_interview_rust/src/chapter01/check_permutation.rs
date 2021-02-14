use itertools::Itertools;

/// First of all, find out these things: Should the comparison be case sensitive?
/// Example is God a permutation of dog?
///
/// Second: Is whitespaces significant?
/// Example "god     " is different from "dog"
pub fn check_permutation(word: &str, permutation: &str) -> bool {
    if word.len() != permutation.len() {
        return false;
    }

    word.chars().sorted().rev().collect::<String>() == permutation.chars().sorted().rev().collect::<String>()
}

/// This function is better if efficient is important, maybe not so readable as the first function
/// regarding the assumption, always check about he size of the character set, here we assuming it's ASCII
pub fn check_permutation_efficient(word: &str, permutation: &str) -> bool {
    if word.len() != permutation.len() {
        return false;
    }

    let mut letters: [u8; 128] = [0; 128];

    word.chars().for_each(|ch| {
        letters[ch as usize] += 1;
    });

    for i in 0..permutation.len() {
        let c = permutation.chars().nth(i).unwrap();
        letters[c as usize] -= 1;
    }

    true
}

#[test]
fn test_permutation() {
    assert_eq!(check_permutation("hello", "olleh"), true);
    assert_eq!(check_permutation("west", "hest"), false);
}

#[test]
fn test_permutation_efficient() {
    assert_eq!(check_permutation_efficient("hello", "olleh"), true)
}