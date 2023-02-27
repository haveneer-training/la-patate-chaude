#[cfg(test)]
mod tests {
    fn format_binary(c: char) -> String {
        if c == '0' {
            return "0000".to_string();
        } else if c == '1' {
            return "0001".to_string();
        } else if c == '2' {
            return "0010".to_string();
        } else if c == '3' {
            return "0011".to_string();
        } else if c == '4' {
            return "0100".to_string();
        } else if c == '5' {
            return "0101".to_string();
        } else if c == '6' {
            return "0110".to_string();
        } else if c == '7' {
            return "0111".to_string();
        } else if c == '8' {
            return "1000".to_string();
        } else if c == '9' {
            return "1001".to_string();
        } else if c == 'A' {
            return "1010".to_string();
        } else if c == 'B' {
            return "1011".to_string();
        } else if c == 'C' {
            return "1100".to_string();
        } else if c == 'D' {
            return "1101".to_string();
        } else if c == 'E' {
            return "1110".to_string();
        } else if c == 'F' {
            return "1111".to_string();
        };
        return "1".to_string();
    }

    // converti un hexadécimal en binaire
    fn convert_hex_to_binary(hex: &String) -> String {
        let mut binary = "".to_string();
        for i in hex.chars() {
            binary = binary.to_owned() + &format_binary(i);
        }
        binary
    }

    fn verify_bit_zero(number: u32, binary: String) -> bool {
        for i in 0..number {
            if binary.chars().nth(i as usize) != Some('0') {
                return false;
            }
        }
        return true;
    }

    #[test]
    fn test_convert_hex_to_binary_empty_string() {
        assert_eq!(convert_hex_to_binary(&"".to_string()), "");
    }

    #[test]
    fn test_verify_bit_zero_all_zeroes() {
        let binary = "00000000000000000000000000000000".to_string();
        assert!(verify_bit_zero(32, binary));
    }

    #[test]
    fn test_verify_bit_zero_not_all_zeroes() {
        let binary = "00000000100000000000000000001000".to_string();
        assert!(!verify_bit_zero(31, binary));
    }
}
