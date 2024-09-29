use crate::{Byte, Word};
use crate::mem::{Addressing, Mem};
use deku::prelude::*;

/// All internal data structures of the 6502 CPU.
pub struct CPU {
    /// Program counter.
    pub pc: Word,
    /// Stack pointer (should only be `Byte`, not a `Word`).
    pub sp: Byte,
    // Registers.
    pub registers: Registers,
    // Status flags.
    pub flags: StatusFlags,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0xFFFC,
            // TODO: Check the initial sp address.
            sp: 0xFF,
            registers: Registers::new(),
            flags: StatusFlags::new(),
        }
    }

    pub fn reset(&mut self, mem: &mut Mem) {
        self.pc = 0xFFFC;
        self.sp = 0xFF;
        self.registers.clear();
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

    pub fn lda_set_flags(&mut self) {
        // Set if A = 0
        self.flags.z = self.registers.a == 0;
        // Set if bit 7 of A is set
        self.flags.n = (self.registers.a & 0b10000000) > 0;
    }

    pub fn execute(&mut self, mem: &mut Mem) {
        let ins_code = self.fetch_byte(mem);
        Ins::from_byte(ins_code).execute(self, mem);
    }
}

pub struct Registers {
    pub a: Byte,
    pub x: Byte,
    pub y: Byte,
}

impl Registers {
    fn new() -> Self {
        Registers {
            a: 0,
            x: 0,
            y: 0,
        }
    }

    fn clear(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
    }
}

#[derive(Debug, PartialEq, Default, DekuRead, DekuWrite)]
#[deku(endian = "little")]
struct StatusFlags {
    /// Carry Flag.
    #[deku(bits = 1)]
    c: bool,
    /// Zero Flag.
    #[deku(bits = 1)]
    z: bool,
    /// Interrupt Disable.
    #[deku(bits = 1)]
    i: bool,
    /// Decimal Mode Flag.
    #[deku(bits = 1)]
    d: bool,
    /// Break Command.
    #[deku(bits = 1)]
    b: bool,
    /// Overflow Flag.
    #[deku(bits = 1)]
    v: bool,
    /// Negative Flag.
    #[deku(bits = 1)]
    n: bool,
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
    LDA(Addressing),
    JSR(Addressing)
}

impl Ins {
    pub fn from_byte(code: Byte) -> Self {
        match code {
            0x49 => Ins::LDA(Addressing::Immediate),
            0xA5 => Ins::LDA(Addressing::ZeroPage),
            0xB5 => Ins::LDA(Addressing::ZeroPageX),
            0x20 => Ins::JSR(Addressing::Absolute),
            _ => panic!("Unable to identify instruction.")
        }
    }

    pub fn code(&self) -> Byte {
        match self {
            Ins::LDA(a) => {
                match a {
                    Addressing::Immediate => 0x49,
                    Addressing::ZeroPage => 0xA5,
                    Addressing::ZeroPageX => 0xB5,
                    _ => panic!("Instruction not supported.")
                }
            },
            Ins::JSR(a) => {
                match a {
                    Addressing::Absolute => 0x20,
                    _ => panic!("Instruction not supported.")
                }
            },
        }
    }

    pub fn execute(&self, cpu: &mut CPU, mem: &mut Mem) {
        match self {
            Ins::LDA(a) => {
                match a {
                    Addressing::Immediate => {
                        let current_addr = cpu.fetch_byte(mem);
                        cpu.registers.a = current_addr;
                        cpu.lda_set_flags();
                    },
                    Addressing::ZeroPage => {
                        // TODO: Check conversions between Byte and Word.
                        let zero_page_addr = cpu.fetch_byte(mem);
                        cpu.registers.a = mem.read_byte(zero_page_addr as Word);
                        cpu.lda_set_flags();
                    },
                    Addressing::ZeroPageX => {
                        let mut zero_page_addr = cpu.fetch_word(mem);
                        zero_page_addr += cpu.registers.x as Word;
                        cpu.registers.a = mem.read_byte(zero_page_addr);
                        cpu.lda_set_flags();
                    },
                    _ => panic!("Instruction not supported.")
                }
            },
            Ins::JSR(a) => {
                match a {
                    Addressing::Absolute => {
                        let sub_addr = cpu.fetch_word(mem);
                        mem.write_word(cpu.sp as Word, cpu.pc - 1);
                        cpu.pc = sub_addr;
                    },
                    _ => panic!("Instruction not supported.")
                }
            },
        }
    }
}
