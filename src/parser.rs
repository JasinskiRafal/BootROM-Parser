/// Converts a hex dump string into a vector of 32-bit words
///
/// This function parses a string containing space-separated hexadecimal bytes
/// and converts them into 32-bit unsigned integers (u32). The input should
/// be formatted as a memory dump with bytes in little-endian order.
///
/// # Arguments
///
/// * `contents` - A string containing space-separated hexadecimal bytes
///
/// # Returns
///
/// A vector of u32 values, where each value represents 4 bytes from the input
///
/// # Examples
///
/// ```
/// let input = String::from("00 10 20 30 78 56 34 12");
/// let result = from_hex_dump_file(input);
/// // Returns vec![0x30201000, 0x12345678]
/// ```
///
/// # Panics
///
/// This function will panic if any of the hexadecimal strings cannot be parsed
/// as valid base-16 values.
pub fn from_hex_dump_file(contents: String) -> Vec<u32> {
    /// Byte shift values for converting 4 bytes into a u32
    ///
    /// These values correspond to the positions of each byte in a 32-bit word:
    /// - Byte 0: No shift (position 0)
    /// - Byte 1: Shift by 8 bits (position 1)
    /// - Byte 2: Shift by 16 bits (position 2)
    /// - Byte 3: Shift by 24 bits (position 3)
    const SHIFT_VALUES: [u32; 4] = [0, 8, 16, 24];

    // Split the input string by whitespace and collect hex byte strings
    let hex_strings: Vec<&str> = contents.split_whitespace().collect();
    let mut result: Vec<u32> = Vec::new();

    // Process the hex strings in chunks of 4 bytes (32 bits)
    for chunk in hex_strings.chunks(4) {
        if chunk.len() != 4 {
            // If we don't have a complete 4-byte chunk, stop processing
            break;
        }

        let mut parsed_value: u32 = 0;
        // Convert each byte string to its numeric value and combine into u32
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
