use crate::{Byte, Word};
use crate::mem::Memory;
use crate::ins::DecodeIns;

/// All internal data structures of the 6502 CPU.
#[derive(Clone)]
pub struct CPU {
    /// Program counter.
    pub pc: Word,
    /// Stack pointer (should only be `Byte`, not a `Word`).
    ///
    /// The stack pointer (S) points to a byte on Page 1, that is, to a byte whose address
    /// is from 0100 to 01FF, where the last two digits are supplied by S. When a byte is
    /// pushed on the stack, it is written at the address in S, and then S is decremented.
    pub sp: Byte,
    /// Cycle count.
    pub cycles: u32,
    /// Memory.
    pub mem: Memory,
    /// Registers.
    pub reg: Registers,
    /// Status flags.
    pub flags: StatusFlags,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0xFFFC,
            // TODO: Check the initial sp address.
            sp: 0xFF,
            cycles: 0,
            mem: Memory::new(),
            reg: Registers::new(),
            flags: StatusFlags::new(),
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0xFFFC;
        self.sp = 0xFF;
        self.cycles = 0;
        self.reg.clear();
        self.flags.clear();
        self.mem.init();
    }

    /// Fetch the next instruction from memory.
    pub fn fetch(&mut self) -> Byte {
        self.read_byte(self.pc)
    }

    /// Read a byte from the specified address.
    pub fn read_byte(&self, address: Word) -> Byte {
        self.mem.read_byte(address)
    }

    /// Read a word from the specified address.
    pub fn read_word(&self, address: Word) -> Word {
        self.mem.read_word(address)
    }

    /// Write a byte of data to the specified address.
    pub fn write_byte(&mut self, address: Word, data: Byte) {
        self.mem.write_byte(address, data);
    }

    /// Write a word of data to the specified address.
    pub fn write_word(&mut self, address: Word, data: Word) {
        self.mem.write_word(address, data);
    }

    pub fn stack_address(addr: Byte) -> Word {
        0x0100 + addr as Word
    }

    /// Starts the fetch-decode-execute cycle.
    pub fn start(&mut self) {
        // loop {
            self
                // Fetch the next instruction code from memory.
                .fetch()
                // Identify the instruction from the code retrieved.
                .decode()
                // Execute the instruction in our CPU.
                .execute(self)
        // }
    }
}

/// Storage location that holds inputs and outputs for the ALU.
#[derive(Clone)]
pub struct Registers {
    /// The 8-bit accumulator is used for all arithmetic and logical operations except
    /// increments and decrements. The contents of the accumulator can be stored and
    /// retrieved either from memory or the stack.
    pub acc: Byte,
    /// The 8-bit index register (X) is most commonly used to hold counters or offsets for
    /// accessing memory. The value of the X register can be loaded and saved in memory,
    /// compared with values held in memory, or incremented and decremented. This register
    /// has one special function; it can be used to get a copy of the stack pointer or
    /// change its value.
    pub x: Byte,
    /// The Y register is similar to the X register in that it is available for holding
    /// counter or offsets memory access and supports the same set of memory load, save
    /// and compare operations, and increments and decrements. Unlike X, it has no special
    /// functions.
    pub y: Byte,
}

impl Registers {
    fn new() -> Self {
        Registers {
            acc: 0,
            x: 0,
            y: 0,
        }
    }

    fn clear(&mut self) {
        self.acc = 0;
        self.x = 0;
        self.y = 0;
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct StatusFlags {
    /// Carry Flag.
    pub c: bool,
    /// Zero Flag.
    pub z: bool,
    /// Interrupt Disable.
    pub i: bool,
    /// Decimal Mode Flag.
    pub d: bool,
    /// Break Command.
    pub b: bool,
    /// Overflow Flag.
    pub v: bool,
    /// Negative Flag.
    pub n: bool,
}

impl StatusFlags {
    fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

impl Into<Byte> for StatusFlags {
    fn into(self) -> u8 {
        (if self.c { 0b00000001 } else { 0 }) |
        (if self.z { 0b00000010 } else { 0 }) |
        (if self.i { 0b00000100 } else { 0 }) |
        (if self.d { 0b00001000 } else { 0 }) |
        (if self.b { 0b00010000 } else { 0 }) |
        // empty
        (if self.v { 0b01000000 } else { 0 }) |
        (if self.n { 0b10000000 } else { 0 })
    }
}

impl From<Byte> for StatusFlags {
    fn from(value: u8) -> Self {
        let c = (value & 0b00000001) > 0;
        let z = (value & 0b00000010) > 0;
        let i = (value & 0b00000100) > 0;
        let d = (value & 0b00001000) > 0;
        let b = (value & 0b00010000) > 0;
        // empty
        let v = (value & 0b01000000) > 0;
        let n = (value & 0b10000000) > 0;

        Self { c, z, i, d, b, v, n }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_flags_into_byte() {
        let flags = StatusFlags {
            c: false,
            z: false,
            i: true,
            d: false,
            b: true,
            v: true,
            n: false,
        };
        let flags_byte: Byte = flags.into();
        assert_eq!(flags_byte, 0b01010100);
    }

    #[test]
    fn test_byte_into_status_flags() {
        let flags = StatusFlags {
            c: false,
            z: false,
            i: true,
            d: false,
            b: true,
            v: true,
            n: false,
        };
        assert_eq!(StatusFlags::from(0b01010100), flags);
        // We should never reach this value since bit 5 is unused, but the conversion
        // should still be valid.
        assert_eq!(StatusFlags::from(0b01110100), flags);

    }
}
