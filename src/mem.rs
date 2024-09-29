use crate::{Byte, Word};

// This is a `usize` since it refers to memory representation on the host machine (we
// assume that `usize` is greater than `Word`). Every other type referring to a logical
// memory location should be a `Word`.
const MAX_MEM: usize = 1024 * 64;

pub struct Mem {
    data: [Byte; MAX_MEM]
}

impl Mem {
    pub fn new() -> Self {
        Mem {
            data: [0; MAX_MEM]
        }
    }

    pub fn init(&mut self) {
        self.data = [0; MAX_MEM]
    }

    pub fn read_byte(&self, address: Word) -> Byte {
        let value = self.data[address as usize];
        value
    }

    pub fn write_byte(&mut self, address: Word, value: Byte) {
        self.data[address as usize] = value;
    }

    pub fn read_word(&self, address: Word) -> Word {
        let mut data = self.read_byte(address) as Word;
        data |= (self.read_byte(address + 1) as Word) << 8;
        data
    }

    pub fn write_word(&self, address: Word, value: Word) {
        todo!()
    }
}
