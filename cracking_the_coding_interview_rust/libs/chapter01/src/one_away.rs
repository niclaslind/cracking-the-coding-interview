/// There are three types of edits that can be performed on strings:
/// insert a character, remove a character, or replace a character
/// Given two strings, write a function to check if they are one edit (or zero edits away)
/// Both [one_edit_away] and [one_edit_replace] takes O(n) time,
/// where n is the length of the shorter string

/// Compare two strings and see if they match
pub fn one_edit_away(s1: &str, s2: &str) -> bool {
    if s1.len() == s2.len() {
        return one_edit_replace(s1, s2);
    } else if s1.len() + 1 == s2.len() {
        return one_edit_insert(s1, s2);
    } else if s1.len() - 1 == s2.len() {
        return one_edit_insert(s2, s1);
    }

    false
}

fn one_edit_replace(s1: &str, s2: &str) -> bool {
    let mut found_diff = false;

    for i in 0..s1.len() {
        if s1.as_bytes()[i] != s2.as_bytes()[i] {
            if found_diff {
                return false;
            }
            found_diff = true;
        }
    }
    true
}

/// Check if you can insert a character into s1 to make s2
fn one_edit_insert(s1: &str, s2: &str) -> bool {
    let mut index1 = 0;
    let mut index2 = 0;

    while index2 < s2.len() && index1 < s1.len() {
        if s1.as_bytes()[index1] != s2.as_bytes()[index2] {
            if index1 != index2 {
                return false;
            }
            index2 += 1;
        } else {
            index1 += 1;
            index2 += 1;
        }
    }

    true
}

#[test]
fn test_check_strings() {
    assert!(one_edit_away("pale", "ple"));
    assert!(one_edit_away("pales", "pale"));
    assert!(one_edit_away("pale", "bale"));
    assert!(!one_edit_away("pale", "bake"))
}
