use crate::cpu::CPU;
use crate::{ins::Instruction, mem::Addr};
use crate::{Byte, Word};

/// Store Y Register - Stores the contents of the Y register into memory.
pub struct STY(pub Addr);

impl Instruction for STY {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 2B, 3C
            STY(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                cpu.write_byte(zp_addr as Word, cpu.reg.y);
                cpu.pc += 2;
            },
            // 2B, 4C
            STY(Addr::ZeroPageX) => {
                let mut zp_addr = cpu.read_byte(cpu.pc + 1);
                zp_addr = zp_addr.wrapping_add(cpu.reg.x);
                cpu.write_byte(zp_addr as Word, cpu.reg.y);
                cpu.pc += 2;
            },
            // 3B, 4C
            STY(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                cpu.write_byte(addr, cpu.reg.y);
                cpu.pc += 3;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            STY(Addr::ZeroPage) => 0x84,
            STY(Addr::ZeroPageX) => 0x94,
            STY(Addr::Absolute) => 0x8C,
            _ => panic!("Operation not supported")
        }
    }
}
