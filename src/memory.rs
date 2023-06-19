use std::fs::File;
use std::io::Read;

pub struct Memory {
    stack: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            stack: vec![0; 0x200],
        }
    }

    pub fn load(&mut self, mut file: File) {
        file.read_to_end(&mut self.stack).unwrap();
    }

    pub fn get_byte(&self, idx: usize) -> u8 {
        self.stack[idx]
    }

    pub fn set_byte(&mut self, idx: usize, new_byte: u8) {
        self.stack[idx] = new_byte;
    }
}
