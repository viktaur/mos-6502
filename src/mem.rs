use crate::{Byte, Word};

// This is a `usize` since it refers to memory representation on the host machine (we
// assume that `usize` is greater than `Word`). Every other type referring to a logical
// memory location should be a `Word`.
const MAX_MEM: usize = 1024 * 64;

#[derive(Clone)]
pub struct Memory {
    data: [Byte; MAX_MEM]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; MAX_MEM]
        }
    }

    pub fn init(&mut self) {
        self.data = [0; MAX_MEM]
    }

    /// Read a byte from memory, either statically or dynamically by the CPU.
    pub fn read_byte(&self, address: Word) -> Byte {
        let value = self.data[address as usize];
        value
    }

    /// Read a word from memory, either statically or dynamically by the CPU.
    pub fn read_word(&self, address: Word) -> Word {
        let mut data = self.read_byte(address) as Word;
        data |= (self.read_byte(address + 1) as Word) << 8;
        data
    }

    /// Write a byte to memory, either statically or dynamically by the CPU.
    pub fn write_byte(&mut self, address: Word, value: Byte) {
        self.data[address as usize] = value;
    }

    /// Write a word to memory, either statically or dynamically by the CPU.
    pub fn write_word(&mut self, address: Word, value: Word) {
        self.write_byte(address, value as Byte);
        self.write_byte(address + 1, (value >> 8) as Byte);
    }
}

/// Addressing type.
pub enum Addr {
    /// For many 6502 instructions the source and destination of the information to be
    /// manipulated is implied directly by the function of the instruction itself and no
    /// further operand needs to be specified. Operations like 'Clear Carry Flag' (CLC)
    /// and 'Return from Subroutine' (RTS) are implicit.
    Implicit,
    /// Some instructions have an option to operate directly upon the accumulator. This
    /// may be specified in code by using a special operand value, `A`.
    Accummulator,
    /// These instructions have their data defined in the next byte after the opcode.
    Immediate,
    /// An instruction using zero page addressing mode has only an 8-bit address operand.
    /// This limits the addressing to the first 256 bytes of memory (i.e. page zero,
    /// `0x0000` to `0x00FF`), where the most significant byte of the address is always
    /// zero.
    ZeroPage,
    /// The address is calculated by taking a Zero Page address and adding the value in
    /// the X register, wrapping if it goes over 0xFF.
    ZeroPageX,
    /// The address is calculated by taking a Zero Page address and adding the value in
    /// the Y register, wrapping if it goes over 0xFF.
    ZeroPageY,
    Relative,
    /// Uses a full 16-bit address to identify the target location.
    Absolute,
    /// The address is computed by taking the 16-bit address from the instruction and
    /// adding the contents of the X register. For example if X contains `0x92`, then the
    /// `STA $2000,X` instruction will store the accumulator at `0x2092` (i.e. `0x2000 +
    /// 0x92`).
    AbsoluteX,
    /// The address is computed by taking the 16-bit address from the instruction and
    /// adding the contents of the Y register. For example if Y contains `0x92`, then the
    /// `STA $2000,Y` instruction will store the accumulator at `0x2092` (i.e. `0x2000 +
    /// 0x92`).
    AbsoluteY,
    /// Data is accessed using a pointer. The 16-bit address of the pointer is given in
    /// the two bytes following the opcode.
    Indirect,
    /// An 8-bit zero-page address and the X register are added, without carry (if the
    /// addition overflows, the address wraps around within page 0). The resulting address
    /// is used as a pointer to the data being accessed. Note that, effectively, this
    /// makes the X register an index into a list of pointers. Also note that pointers
    /// are two bytes long, so the X register should be an even number when accessing a
    /// list of pointers (otherwise you'll get half of one pointer and half of another).
    XIndirect,
    /// An 8-bit address identifies a pointer. The value of the Y register is added to the
    /// address contained in the pointer. Effectively, the pointer is the base address
    /// and the Y register is an index past that base address.
    IndirectY,
}
