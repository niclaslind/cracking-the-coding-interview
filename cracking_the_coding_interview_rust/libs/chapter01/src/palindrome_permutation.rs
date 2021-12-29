use std::ops::{BitAndAssign, BitOrAssign};

/// Given a string, check if if it is a permutation of a palindrome. The palindrome
/// does not need to be limited to just dictionary words.
/// Palindrome: same forward and backward
/// Permutation: A rearrangement of letters

/// This solution takes O(N) time, where N is the length of the string
pub fn is_permutation_of_palindrome(phrase: &str) -> bool {
    let table = build_char_frequency_table(phrase);
    check_max_one_odd(table)
}

/// This function may no necessarily be more optimal than the first one.
/// It has the same big O time and might even be slightly slower.
/// We have eliminated a final iteration through the hash table,
/// but now we have to run a few extra lines of code for each character in the sting.
pub fn is_permutation_of_palindrome_optimized(phrase: &str) -> bool {
    let mut count_odd = 0;

    const Z: i32 = 'z' as i32;
    const A: i32 = 'a' as i32;
    let mut table: Vec<i32> = vec![-1; (Z - A + 1) as usize];

    phrase.chars().for_each(|c| {
        let x = get_char_number(c);
        if x != -1 {
            table[x as usize] += 1;
            if table[x as usize] % 2 == 1 {
                count_odd += 1;
            } else {
                count_odd -= 1;
            }
        }
    });
    count_odd <= 1
}

/// Like the other solution this is O(N)
pub fn is_permutation_of_palindrome_bit_vector(phrase: &str) -> bool {
    let bit_vector = create_bit_vector(phrase);
    bit_vector == 0 || check_exactly_one_bitset(bit_vector)
}

/// Check that no more than one character has an odd count
fn check_max_one_odd(table: Vec<i32>) -> bool {
    let mut found_odd = false;

    for count in table.iter() {
        if count % 2 == 1 {
            if found_odd {
                return false;
            }
            found_odd = true
        }
    }

    true
}

/// Map each character to a number. a -> 0, b -> 1, c -> 2 etc.
fn get_char_number(c: char) -> i32 {
    // change type here from char to i32
    let c = c as i32;
    let a = 'a' as i32;

    if a as i32 <= c && c <= 'z' as i32 {
        return c - a;
    }

    -1
}

/// Count how many times each character appears
fn build_char_frequency_table(phrase: &str) -> Vec<i32> {
    const Z: i32 = 'z' as i32;
    const A: i32 = 'a' as i32;
    let mut table: Vec<i32> = vec![-1; (Z - A + 1) as usize];

    phrase.chars().for_each(|c| {
        let x = get_char_number(c);

        if x != -1 {
            table[x as usize] += 1;
        }
    });

    table.to_vec()
}

/// crate a bit vector for the string. For each letter with value i, toggle the ith bit
fn create_bit_vector(phrase: &str) -> i32 {
    let mut bit_vector = 0;

    phrase.chars().for_each(|c| {
        let x = get_char_number(c);
        bit_vector = toggle(bit_vector, x);
    });

    bit_vector
}

/// Toggle the ith bit in the integer
/// todo: check why the bit_vector grow after the 3rd iteration
fn toggle(mut bit_vector: i32, index: i32) -> i32 {
    if index < 0 {
        return bit_vector;
    }

    let mask = 1 << index;

    if (bit_vector & mask) == 0 {
        bit_vector.bitor_assign(mask);
    } else {
        bit_vector.bitand_assign(!mask);
    }

    bit_vector
}

/// Check that exactly one bit is set by subtracting one from the integer and ANDing it with
/// the original integer
fn check_exactly_one_bitset(bit_vector: i32) -> bool {
    (bit_vector & (bit_vector - 1)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation_of_palindrome() {
        assert!(is_permutation_of_palindrome("Tact Coa"));
    }

    #[test]
    fn test_is_permutation_of_palindrome_optimized() {
        assert!(is_permutation_of_palindrome_optimized("Tact Coa"));
    }

    #[test]
    fn test_is_permutation_of_palindrome_bit_vector() {
        // TODO: Change this, should be true....
        assert!(is_permutation_of_palindrome_bit_vector("Tact Coa"));
    }
}
