pub fn hex_string_to_int(hex_string: &str) -> Option<u64> {
    // Attempt to parse the hexadecimal string into a u64 integer
    match u64::from_str_radix(hex_string, 16) {
        // If parsing succeeds, return the parsed integer wrapped in Some
        Ok(parsed_int) => Some(parsed_int),
        // If parsing fails (invalid input), return None
        Err(_) => None,
    }
}