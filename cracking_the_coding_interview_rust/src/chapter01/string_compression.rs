/// Implement a method to perform basic string compression using the counts
/// of repeted characters. For example, the string aabcccccaaa would become a2b1c5a3.
/// If the "compressed" string would not become smaller then the original string,
/// your method should retuyrn the original string. You can assume the string has
/// only uppercase and lowercase letters (a-z).
/// Hints:
///     - Do the easy things first. Compress the thring, then compare the length.
///     - Be careful that you aren't repeatedly concatenating strings toghether. This can be very inefficient.

pub fn string_compression(input: &str) -> String {
    let mut first_run = true;
    let mut counter = 0;
    let mut last_char = '_';
    let mut new_string = String::new();
    input.chars().for_each(|c| {
        if first_run {
            last_char = c;
            counter += 1;
            first_run = false;
        } else {
            if c != last_char {
                new_string.push(last_char);
                new_string.push_str(&counter.to_string());
                counter = 0;
            }
            last_char = c;
            counter += 1;
        }
    });

    new_string.push(last_char);
    new_string.push_str(&counter.to_string());

    // Compere the string lengths
    if new_string.len() > input.len() {
        return input.to_string();
    }
    new_string
}

#[test]
fn test_string_compression() {
    assert_eq!(string_compression("a"), "a".to_string());
    assert_eq!(string_compression("aaaaabb"), "a5b2".to_string());
    assert_eq!(string_compression("aabcccccaaa"), "a2b1c5a3".to_string());
    assert_eq!(string_compression("yyffssqqaavvff"), "y2f2s2q2a2v2f2".to_string());
}
