use crate::{cpu::CPU, ins::Instruction, mem::{Addr, Mem}};
use crate::{Byte, Word};

/// Load Accummulator. Loads a byte of memory into the accumulator, setting the zero and
/// negative flags as appropriate.
pub struct LDY(Addr);

impl LDY {
    fn set_flags(&self, cpu: &mut CPU) {
        todo!()
    }
}

impl Instruction for LDY {
    fn execute(&self, cpu: &mut CPU, mem: &mut Mem) {
        match self {
            LDY(Addr::Immediate) => {
                cpu.reg.y = cpu.fetch_byte(mem);
                self.set_flags(cpu);
            },
            LDY(Addr::ZeroPage) => {
                let zp_addr = cpu.fetch_byte(mem);
                cpu.reg.y = mem.read_byte(zp_addr as Word);
                self.set_flags(cpu);
            },
            LDY(Addr::ZeroPageX) => {
                let mut zp_addr = cpu.fetch_byte(mem);
                zp_addr += cpu.reg.x;
                cpu.reg.y = mem.read_byte(zp_addr as Word);
                self.set_flags(cpu);
            },
            LDY(Addr::Absolute) => {
                let addr = cpu.fetch_word(mem);
                cpu.reg.y = mem.read_byte(addr);
                self.set_flags(cpu);
            },
            LDY(Addr::AbsoluteX) => {
                let mut addr = cpu.fetch_word(mem);
                addr += cpu.reg.x as Word;
                cpu.reg.y = mem.read_byte(addr);
                self.set_flags(cpu);
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            LDY(Addr::Immediate) => 0xA0,
            LDY(Addr::ZeroPage) => 0xA4,
            LDY(Addr::ZeroPageX) => 0xB4,
            LDY(Addr::Absolute) => 0xAC,
            LDY(Addr::AbsoluteX) => 0xBC,
            _ => panic!("Operation not supported!")
        }
    }

    fn from_byte(code: Byte) -> Self {
        match code {
            0xA0 => LDY(Addr::Immediate),
            0xA4 => LDY(Addr::ZeroPage),
            0xB4 => LDY(Addr::ZeroPageX),
            0xAC => LDY(Addr::Absolute),
            0xBC => LDY(Addr::AbsoluteX),
            _ => panic!("Unable to identify instruction.")
        }
    }
}
