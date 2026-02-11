pub fn from_hex_dump_file(contents: String) -> Vec<u32> {
    const SHIFT_VALUES: [u32; 4] = [0, 8, 16, 24];
    let hex_strings: Vec<&str> = contents.split_whitespace().collect();
    let mut result: Vec<u32> = Vec::new();
    for chunk in hex_strings.chunks(4) {
        if chunk.len() != 4 {
            break;
        }

        let mut parsed_value: u32 = 0;
        for (idx, byte) in chunk.iter().enumerate() {
            let byte_value =
                u8::from_str_radix(byte, 16).expect("Failed to parse base-16 string to value");
            parsed_value |= (byte_value as u32) << SHIFT_VALUES[idx];
        }
        result.push(parsed_value);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_hex_dump_file_test() {
        let test_string = String::from("00 10 20 30 78 56 34 12 00 00 00");
        let parsed = from_hex_dump_file(test_string);
        let correct_vector = vec![0x30201000, 0x12345678];
        assert_eq!(2, correct_vector.len());
        assert_eq!(parsed, correct_vector);
    }
}
