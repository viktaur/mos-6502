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
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 2B, 2C
            LDY(Addr::Immediate) => {
                cpu.reg.y = cpu.read_byte(cpu.pc + 1);
                cpu.pc += 2;
            },
            // 2B, 3C
            LDY(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                cpu.reg.y = cpu.read_byte(zp_addr as Word);
                cpu.pc += 2;
            },
            // 2B, 4C
            LDY(Addr::ZeroPageX) => {
                let mut zp_addr = cpu.read_byte(cpu.pc + 1);
                zp_addr = zp_addr.wrapping_add(cpu.reg.x);
                cpu.reg.y = cpu.read_byte(zp_addr as Word);
                cpu.pc += 2;
            },
            // 3B, 4C
            LDY(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                cpu.reg.y = cpu.read_byte(addr);
                cpu.pc += 3;
            },
            // 3B, 4C (+1 if page crossed)
            LDY(Addr::AbsoluteX) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.x as Word;
                cpu.reg.y = cpu.read_byte(addr);
                cpu.pc += 3;
            },
            _ => panic!("Operation not supported!")
        }
        self.set_flags(cpu);
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
