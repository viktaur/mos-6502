use crate::mem::{Mem, Addr};
use crate::cpu::CPU;
use crate::{Byte, Word};

pub mod arithmetic;
pub mod branches;
pub mod inc_dec;
pub mod jumps_calls;
pub mod load_store;
pub mod logical;
pub mod shifts;
pub mod stack_ops;
pub mod status_flags;
pub mod sys_funcs;
pub mod transfers;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, mem: &mut Mem);

    fn code(&self) -> Byte;

    fn from_byte(code: Byte) -> Self;
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
            0xA2 => Ins::LDX(Addr::Immediate),
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
