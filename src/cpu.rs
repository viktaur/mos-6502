use crate::{Byte, Word};
use crate::mem::{Addr, Mem};
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
    // Registers.
    pub reg: Registers,
    // Status flags.
    pub flags: StatusFlags,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0xFFFC,
            // TODO: Check the initial sp address.
            sp: 0xFF,
            cycles: 0,
            reg: Registers::new(),
            flags: StatusFlags::new(),
        }
    }

    pub fn reset(&mut self, mem: &mut Mem) {
        self.pc = 0xFFFC;
        self.sp = 0xFF;
        self.cycles = 0;
        self.reg.clear();
        self.flags.clear();
        mem.init()
    }

    pub fn fetch_byte(&mut self, mem: &mut Mem) -> Byte {
        let data: Byte = mem.read_byte(self.pc);
        self.pc += 1;
        data
    }

    pub fn fetch_word(&mut self, mem: &mut Mem) -> Word {
        let data = mem.read_word(self.pc);
        self.pc += 2;
        data
    }

    pub fn execute(&mut self, mem: &mut Mem) {
        let ins_code = self.fetch_byte(mem);
        Ins::from_byte(ins_code).execute(self, mem);
    }
}

// TODO segregate functions into these to better represent the fetch-decode-execute cycle.
/// Moves data and instructions between main memory and registers.
pub struct ControlUnit;

/// Arithmetic and Logic Unit. Performs all computation and comparison operations.
pub struct ALU;

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

pub enum Ins {
    /// Add with Carry. This instruction adds the contents of a memory location to the
    /// accumulator together with the carry bit. If overflow occurs, the carry bit is set,
    /// enabling multiple byte addition.
    ADC(Addr),
    /// Load Accummulator. Loads a byte of memory into the accumulator, setting the zero
    /// and negative flags as appropriate.
    LDA(Addr),
    /// Load X Register. Loads a byte of memory into the X register, setting the zero and
    /// negative flags as appropriate.
    LDX(Addr),
    /// Load Y Register. Loads a byte of memory into the Y register, setting the zero and
    /// negative flags as appropriate.
    LDY(Addr),
    /// Jump to Subroutine.
    JSR(Addr),
}

impl Ins {
    pub fn from_byte(code: Byte) -> Self {
        match code {
            0x49 => Ins::LDA(Addr::Immediate),
            0xA5 => Ins::LDA(Addr::ZeroPage),
            0xB5 => Ins::LDA(Addr::ZeroPageX),
            0xAD => Ins::LDA(Addr::Absolute),
            0xBD => Ins::LDA(Addr::AbsoluteX),
            0xB9 => Ins::LDA(Addr::AbsoluteY),
            0xA1 => Ins::LDA(Addr::XIndirect),
            0xB1 => Ins::LDA(Addr::IndirectY),
            0xA2 => Ins::LDA(Addr::Immediate),
            0x20 => Ins::JSR(Addr::Absolute),
            _ => panic!("Unable to identify instruction.")
        }
    }

    pub fn code(&self) -> Byte {
        match self {
            Ins::LDA(Addr::Immediate) => 0x49,
            Ins::LDA(Addr::ZeroPage) => 0xA5,
            Ins::LDA(Addr::ZeroPageX) => 0xB5,
            Ins::LDA(Addr::Absolute) => 0xAD,
            Ins::LDA(Addr::AbsoluteX) => 0xBD,
            Ins::LDA(Addr::AbsoluteY) => 0xB9,
            Ins::LDA(Addr::XIndirect) => 0xA1,
            Ins::LDA(Addr::IndirectY) => 0xB1,
            Ins::LDX(Addr::Immediate) => 0xA2,
            Ins::JSR(Addr::Absolute) => 0x20,
            _ => panic!("Instruction not supported.")
        }
    }

    pub fn execute(&self, cpu: &mut CPU, mem: &mut Mem) {
        match self {
            Ins::ADC(Addr::Immediate) => {
                todo!()
            },
            Ins::LDA(Addr::Immediate) => {
                let data = cpu.fetch_byte(mem);
                cpu.reg.acc = data;
                self.set_flags(cpu);
            },
            Ins::LDA(Addr::ZeroPage) => {
                let zero_page_addr = cpu.fetch_byte(mem);
                cpu.reg.acc = mem.read_byte(zero_page_addr as Word);
                self.set_flags(cpu);
            },
            Ins::LDA(Addr::ZeroPageX) => {
                let mut zero_page_addr = cpu.fetch_byte(mem);
                zero_page_addr += cpu.reg.x;
                cpu.reg.acc = mem.read_byte(zero_page_addr as Word);
                self.set_flags(cpu);
            },
            Ins::LDA(Addr::Absolute) => {
                let address = cpu.fetch_word(mem);
                cpu.reg.acc = mem.read_byte(address);
                self.set_flags(cpu);
            },
            Ins::LDA(Addr::AbsoluteX) => {
                let mut address = cpu.fetch_word(mem);
                address += cpu.reg.x as Word;
                cpu.reg.acc = mem.read_byte(address);
                self.set_flags(cpu);
            },
            Ins::LDA(Addr::AbsoluteY) => {
                let mut address = cpu.fetch_word(mem);
                address += cpu.reg.y as Word;
                cpu.reg.acc = mem.read_byte(address);
                self.set_flags(cpu);
            },
            Ins::LDA(Addr::XIndirect) => {
                let mut ptr = cpu.fetch_byte(mem);
                ptr += cpu.reg.x;
                let address = mem.read_word(ptr as Word);
                cpu.reg.acc = mem.read_byte(address);
                self.set_flags(cpu);
            },
            Ins::LDA(Addr::IndirectY) => {
                let ptr = cpu.fetch_byte(mem);
                let mut address = mem.read_word(ptr as Word);
                address += cpu.reg.y as Word;
                cpu.reg.acc = mem.read_byte(address);
                self.set_flags(cpu);
            }
            Ins::JSR(Addr::Absolute) => {
                let sub_addr = cpu.fetch_word(mem);
                mem.write_word(cpu.sp as Word, cpu.pc - 1);
                cpu.pc = sub_addr;
                // TODO set flags
            },
            _ => panic!("Instruction not supported.")
        }
    }

    pub fn set_flags(&self, cpu: &mut CPU) {
        match self {
            Ins::ADC(_) => {
                todo!()
            }
            Ins::LDA(_) => {
                // Set if A = 0
                cpu.flags.z = cpu.reg.acc == 0;
                // Set if bit 7 of A is set
                cpu.flags.n = (cpu.reg.acc & 0b10000000) > 0;
            },
            Ins::LDX(_) => todo!(),
            Ins::LDY(_) => todo!(),
            Ins::JSR(_) => todo!(),
        }
    }
}
