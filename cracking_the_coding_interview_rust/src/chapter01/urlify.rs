use std::str;

/// Function to replace spaces with %20 in strings
///
/// You are given a string that have sufficient space at the end to hold teh additional characters
/// You are also given the "true" length of the string
///
/// # Arguments
///
/// * `s` - A string slice that holds the input string
/// * `true_length` - The length of the output string
///
pub fn replace_spaces(s: &str, true_length: usize) -> String {
    let mut space_count = 0;
    // clone the input string into a byte array
    let mut result = s.to_string().into_bytes();

    // count white spaces within the "true" length
    (0..true_length).for_each(|i| {
        if result[i] == b' ' {
            space_count += 1;
        }
    });

    let mut index = true_length + space_count * 2;

    // set a null terminator at the end of the true length
    if true_length < s.len() {
        result[true_length] = b'\0';
    }

    // build the output string and replace the whitespaces with %20
    for i in (0..=true_length - 1).rev() {
        if result[i] == b' ' {
            result[index - 1] = b'0';
            result[index - 2] = b'2';
            result[index - 3] = b'%';
            index -= 3;
        } else {
            result[index - 1] = result[i];
            index -= 1;
        }
    }

    String::from(str::from_utf8(&result).unwrap())
}

#[test]
fn test_replace_spaces() {
    assert_eq!(replace_spaces("Mr John Smith    ", 13), "Mr%20John%20Smith");
}
