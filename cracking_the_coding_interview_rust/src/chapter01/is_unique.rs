use std::collections::HashSet;

/// Takes a &str and check that all chars is unique
///
/// Time complexity O(n) and Space complexity O(1)
/// The time complexity can from one side also be O(1) because of that we only check strings
/// that is smaller then 128
///
/// Return true if that the case otherwise returns false
pub fn is_unique(word: &str) -> bool {
    if word.len() > 128 {
        return false;
    }
    let mut checker: HashSet<char> = HashSet::new();

    for ch in word.chars() {
        if !checker.contains(&ch) {
            checker.insert(ch);
        } else {
            return false;
        }
    }

    true
}

/// Takes a &str and check that all chars is unique
///
/// Time complexity O(n) and Space complexity O(1)
/// Here we also reduce the space complexity with eight from the previous solution
/// by using a bit-vector instead of hashset
///
/// In this case we assume that word is using only lowercase letters a..z
pub fn is_unique_chars_bit(word: &str) -> bool {
    if word.len() > 128 {
        return false;
    }
    let mut checker = 0;

    for ch in word.chars() {
        let val = ch as i32 - 'a' as i32;
        if (checker & (1 << val)) > 0 {
            return false;
        }
        checker |= 1 << val;
    }

    true
}

#[test]
fn test_is_unique() {
    assert_eq!(is_unique("Hej"), true);
    assert_eq!(is_unique("Hello"), false);
}

#[test]
fn test_is_unique_chars_bit() {
    assert_eq!(is_unique_chars_bit("hej"), true);
    assert_eq!(is_unique_chars_bit("hello"), false);
}