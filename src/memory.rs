use crate::constants::FONT;
use std::fs::File;
use std::io::Read;

pub struct Memory {
    pub ram: Vec<u8>,
    pub stack: [u16; 16],
}

impl Memory {
    pub fn new() -> Memory {
        let mut initial_ram: Vec<u8> = FONT.to_vec();
        initial_ram.append(&mut vec![0_u8; 432]);
        Memory {
            ram: initial_ram,
            stack: [0; 16],
        }
    }

    pub fn load(&mut self, mut file: File) {
        file.read_to_end(&mut self.ram).unwrap();
    }
}
