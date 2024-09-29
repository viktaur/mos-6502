use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};
use deku::prelude::*;

// These represent the types of the emulated 6502 CPU.
type Byte = u8;
type Word = u16;

// This is a `usize` since it refers to memory representation on the host machine (we
// assume that `usize` is greater than `Word`). Every other type referring to a logical
// memory location should be a `Word`.
const MAX_MEM: usize = 1024 * 64;

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum Instruction {
    /// Load Accumulator (LDA) Intermediate.
    INS_LDA_IM = 0xA9,
    /// Load Accummulator (LDA) Zero Page.
    INS_LDA_ZP = 0xA5,
    /// Load Accummulator (LDA) Zero Page X.
    INS_LDA_ZPX = 0xB5,
    /// Jump to Subroutine
    INS_JSR,
}

struct Mem {
    data: [Byte; MAX_MEM]
}

impl Mem {
    fn new() -> Self {
        Mem {
            data: [0; MAX_MEM]
        }
    }

    fn init(&mut self) {
        self.data = [0; MAX_MEM]
    }

    fn read_byte(&self, address: Word) -> Byte {
        let value = self.data[address as usize];
        value
    }

    fn write_byte(&mut self, address: Word, value: Byte) {
        self.data[address as usize] = value;
    }

    fn read_word(&self, address: Word) -> Word {
        let mut data = self.read_byte(address) as Word;
        data |= (self.read_byte(address + 1) as Word) << 8;
        data
    }

    fn write_word(&self, address: Word, value: Word) {
        todo!()
    }
}

/// All internal data structures of the 6502 CPU.
struct CPU {
    /// Program counter.
    pc: Word,
    /// Stack pointer.
    sp: Word,
    // Registers.
    registers: Registers,
    // Status flags.
    flags: StatusFlags,
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

impl CPU {
    fn new() -> Self {
        Self {
            pc: 0xFFFC,
            sp: 0x0100,
            registers: Registers::new(),
            flags: StatusFlags::new(),
        }
    }

    fn reset(&mut self, mem: &mut Mem) {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.registers.clear();
        self.flags.clear();
        mem.init()
    }

    fn fetch_byte(&mut self, mem: &mut Mem) -> Byte {
        let data: Byte = mem.read_byte(self.pc);
        self.pc += 1;
        data
    }

    fn fetch_word(&mut self, mem: &mut Mem) -> Word {
        let data = mem.read_word(self.pc);
        self.pc += 2;
        data
    }

    fn lda_set_flags(&mut self) {
        // Set if A = 0
        self.flags.z = self.registers.a == 0;
        // Set if bit 7 of A is set
        self.flags.n = (self.registers.a & 0b10000000) > 0;
    }

    fn execute(&mut self, mem: &mut Mem) {
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
                    mem.write_word(self.sp, self.pc - 1);
                }
                _ => {
                    println!("Instruction not handled");
                }
            }
        // }
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
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
        StatusFlags {
            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            v: false,
            n: false,
        }
    }

    fn clear(&mut self) {
        self.c = false;
        self.z = false;
        self.i = false;
        self.d = false;
        self.b = false;
        self.v = false;
        self.n = false;
    }
}

fn main() {
    let mut cpu = CPU::new();
    let mut mem = Mem::new();
    cpu.reset(&mut mem);
    mem.write_byte(0xFFFC, Instruction::INS_LDA_IM.to_u8().unwrap());
    mem.write_byte(0xFFFD, 0x42);
    mem.write_byte(0x0042, 0x84);
    cpu.execute(&mut mem);
    println!("Successfully executed")
}
