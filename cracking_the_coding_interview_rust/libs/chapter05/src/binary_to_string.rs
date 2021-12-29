pub fn binary_to_string(num: f32) -> String {
    if num >= 1.0 || num <= 0.0 {
        return "ERROR".to_string()
    }
    
    let mut num = num;
    let mut binary = String::new();

    while num > 0.0 {
        if binary.len() > 32 {
            return "ERROR".to_string();
        }

        let r = num * 2.0;
        if r >= 1.0 {
            binary.push('1');
            num = r - 1.0
        } else {
            binary.push('0');
            num = r;
        }
    }

    binary
}

#[cfg(test)]
mod tests{ 
    use super::*;

    #[test]
    fn test_binary_to_string(){
        assert_eq!(binary_to_string(0.72), "1011100001010001111011");
    }

    #[test]
    fn test_binary_to_string_value_out_of_range(){
        assert_eq!(binary_to_string(1.01), "ERROR");
        assert_eq!(binary_to_string(0.00), "ERROR");
    }
}