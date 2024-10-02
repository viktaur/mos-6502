use crate::cpu::CPU;
use crate::{ins::Instruction, mem::Addr};
use crate::{Byte, Word};

/// Store X Register - Stores the contents of the X register into memory.
pub struct STX(pub Addr);

impl Instruction for STX {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 2B, 3C
            STX(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                cpu.write_byte(zp_addr as Word, cpu.reg.x);
                cpu.pc += 2;
            },
            // 2B, 4C
            STX(Addr::ZeroPageY) => {
                let mut zp_addr = cpu.read_byte(cpu.pc + 1);
                zp_addr = zp_addr.wrapping_add(cpu.reg.y);
                cpu.write_byte(zp_addr as Word, cpu.reg.x);
                cpu.pc += 2;
            },
            // 3B, 4C
            STX(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                cpu.write_byte(addr, cpu.reg.x);
                cpu.pc += 3;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            STX(Addr::ZeroPage) => 0x86,
            STX(Addr::ZeroPageY) => 0x96,
            STX(Addr::Absolute) => 0x8E,
            _ => panic!("Operation not supported")
        }
    }
}
