use crate::{Byte, Word};
use crate::mem::{Addr, Memory};
use crate::ins::{Instruction, InstructionDecoder, DecodeIns};
use deku::prelude::*;

/// All internal data structures of the 6502 CPU.
#[derive(Clone)]
pub struct CPU {
    /// Program counter.
    pub pc: Word,
    /// Stack pointer (should only be `Byte`, not a `Word`).
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
        self.mem.init()
    }

    /// Fetch the next instruction from memory.
    pub fn fetch(&mut self) -> Byte {
        self.read_byte(self.pc)
    }

    /// Read the specified memory cell.
    pub fn read_byte(&self, address: Word) -> Byte {
        // let data: Byte = self.mem.read_byte(self.pc);
        // self.pc += 1;
        self.mem.read_byte(address)
    }

    /// Read the next two memory cells and update the program counter.
    pub fn read_word(&self, address: Word) -> Word {
        // let data = self.mem.read_word(self.pc);
        // self.pc += 2;
        self.mem.read_word(address)
    }

    pub fn write_byte(&mut self, address: Word, data: Byte) {
        self.mem.write_byte(address, data);
        // self.pc += 1;
    }

    pub fn write_word(&mut self, address: Word, data: Word) {
        self.mem.write_word(address, data);
        // self.pc += 2;
    }

    pub fn jump_to(&mut self, address: Word) {
        self.pc = address;
    }

    /// Starts the fetch-decode-execute cycle.
    pub fn start(&mut self) {
        loop {
            self
                // Fetch the next instruction code from memory.
                .fetch()
                // Identify the instruction from the code retrieved.
                .decode()
                // Execute the instruction in our CPU.
                .execute(self)
        }
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

#[derive(Debug, PartialEq, Clone, Default, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct StatusFlags {
    /// Carry Flag.
    #[deku(bits = 1)]
    pub c: bool,
    /// Zero Flag.
    #[deku(bits = 1)]
    pub z: bool,
    /// Interrupt Disable.
    #[deku(bits = 1)]
    pub i: bool,
    /// Decimal Mode Flag.
    #[deku(bits = 1)]
    pub d: bool,
    /// Break Command.
    #[deku(bits = 1)]
    pub b: bool,
    /// Overflow Flag.
    #[deku(bits = 1)]
    pub v: bool,
    /// Negative Flag.
    #[deku(bits = 1)]
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
