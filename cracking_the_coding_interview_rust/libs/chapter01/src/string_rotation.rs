fn is_rotated_string(s1: &str, s2: &str) -> bool {
    let tres_str = format!("{}{}{}", s1, s1, s1);

    tres_str.contains(s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_rotation() {
        assert_eq!(is_rotated_string("waterbottle", "erbottlewat"), true);
    }
}
