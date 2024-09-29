use crate::{Byte, Word};
use crate::ins::Instruction;
use crate::mem::Mem;
use num::{FromPrimitive, ToPrimitive};
use deku::prelude::*;

/// All internal data structures of the 6502 CPU.
pub struct CPU {
    /// Program counter.
    pc: Word,
    /// Stack pointer (should only be `Byte`, not a `Word`).
    sp: Byte,
    // Registers.
    registers: Registers,
    // Status flags.
    flags: StatusFlags,
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
        // while cycles > 0 {
            let ins = self.fetch_byte(mem);
            match Instruction::from_u8(ins) {
                Some(Instruction::INS_LDA_IM) => {
                    let current_addr = self.fetch_byte(mem);
                    self.registers.a = current_addr;
                    self.lda_set_flags();
                },
                Some(Instruction::INS_LDA_ZP) => {
                    // TODO: Check conversions between Byte and Word.
                    let zero_page_addr = self.fetch_byte(mem);
                    self.registers.a = mem.read_byte(zero_page_addr.into());
                    self.lda_set_flags();
                },
                Some(Instruction::INS_LDA_ZPX) => {
                    let mut zero_page_addr = self.fetch_word(mem);
                    zero_page_addr += self.registers.x as Word;
                    self.registers.a = mem.read_byte(zero_page_addr);
                    self.lda_set_flags();
                },
                Some(Instruction::INS_JSR) => {
                    let sub_addr = self.fetch_word(mem);
                    mem.write_word(self.sp.into(), self.pc - 1);
                }
                _ => {
                    println!("Instruction not handled");
                }
            }
        // }
    }
}

struct Registers {
    a: Byte,
    x: Byte,
    y: Byte,
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
