use crate::{cpu::CPU, ins::Instruction, mem::{Addr, Memory}};
use crate::{Byte, Word};

/// Load Accummulator. Loads a byte of memory into the accumulator, setting the zero and
/// negative flags as appropriate.
pub struct LDY(pub Addr);

impl LDY {
    fn set_flags(&self, cpu: &mut CPU) {
        todo!()
    }
}

impl Instruction for LDY {
    fn execute(&self, cpu: &mut CPU, mem: &mut Memory) {
        match self {
            LDY(Addr::Immediate) => {
                cpu.reg.y = cpu.read_byte(mem);
                self.set_flags(cpu);
            },
            LDY(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(mem);
                cpu.reg.y = mem.read_byte(zp_addr as Word);
                self.set_flags(cpu);
            },
            LDY(Addr::ZeroPageX) => {
                let mut zp_addr = cpu.read_byte(mem);
                zp_addr += cpu.reg.x;
                cpu.reg.y = mem.read_byte(zp_addr as Word);
                self.set_flags(cpu);
            },
            LDY(Addr::Absolute) => {
                let addr = cpu.read_word(mem);
                cpu.reg.y = mem.read_byte(addr);
                self.set_flags(cpu);
            },
            LDY(Addr::AbsoluteX) => {
                let mut addr = cpu.read_word(mem);
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
}
