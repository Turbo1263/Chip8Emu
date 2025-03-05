pub struct SysMem {
    memory: [u8; 4096],
    indexRegister: u16,
    programCounter: u16,
    stack: Vec<u8>,
    stackPointer: u8,
    delayTimer: u8,
    soundTimer: u8,
    

}

impl SysMem {

    pub fn new() -> SysMem {
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

    
}