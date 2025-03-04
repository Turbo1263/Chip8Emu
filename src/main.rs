struct SysMem {
    memory: [u8; 4096],
    indexRegister: u16,
    programCounter: u16,
    stack: Vec<u8>,
    stackPointer: u8,
    delayTimer: u8,
    soundTimer: u8,
    

}

impl SysMem {

    fn new() -> SysMem {
        SysMem {
            memory: [0; 4096], //PrePopulate with Font later
            indexRegister: 0,
            programCounter: 0,
            stack: Vec::new(),
            stackPointer: 0,
            delayTimer: 0,
            soundTimer: 0
        }
    }

    fn hex_string_to_int(hex_string: &str) -> Option<u64> {
        // Attempt to parse the hexadecimal string into a u64 integer
        match u64::from_str_radix(hex_string, 16) {
            // If parsing succeeds, return the parsed integer wrapped in Some
            Ok(parsed_int) => Some(parsed_int),
            // If parsing fails (invalid input), return None
            Err(_) => None,
        }
    }
}

fn main() {
    let memory = SysMem::new();
    let number = SysMem::hex_string_to_int("F");
    let mut newnumber: u64 = 5;
    if number.is_some() {
        newnumber = number.unwrap();
    }
    println!("{}", newnumber.to_string());
}
