use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::{Byte, Word};

/// Logical Inclusive OR - Performs a bitwise ORA operation between the value in the Accumulator
/// and the specified byte, storing the result in the Accumulator.
pub struct ORA(pub Addr);

impl ORA {
    fn set_flags(cpu: &mut CPU) {
        // Set zero flag if X = 0
        cpu.flags.z = cpu.reg.x == 0;
        // Set negative flag if bit 7 of X is set
        cpu.flags.n = (cpu.reg.x & 0b10000000) > 0;
    }
}

impl Instruction for ORA {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 2B, 2C
            ORA(Addr::Immediate) => {
                let value = cpu.read_byte(cpu.pc + 1);
                cpu.reg.acc |= value;
                cpu.pc += 2;
            },
            // 2B, 3C
            ORA(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                let value = cpu.read_byte(zp_addr as Word);
                cpu.reg.acc |= value;
                cpu.pc += 2;
            },
            // 2B, 4C
            ORA(Addr::ZeroPageX) => {
                let mut zp_addr = cpu.read_byte(cpu.pc + 1);
                zp_addr = zp_addr.wrapping_add(cpu.reg.x);
                let value = cpu.read_byte(zp_addr as Word);
                cpu.reg.acc |= value;
                cpu.pc += 2;
            },
            // 3B, 4C
            ORA(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                let value = cpu.read_byte(addr);
                cpu.reg.acc |= value;
                cpu.pc += 3;
            },
            // 3B, 4C (+1 if page crossed)
            ORA(Addr::AbsoluteX) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.x as Word;
                let value = cpu.read_byte(addr);
                cpu.reg.acc |= value;
                cpu.pc += 3;
            },
            // 3B, 4C (+1 if page crossed)
            ORA(Addr::AbsoluteY) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.y as Word;
                let value = cpu.read_byte(addr);
                cpu.reg.acc |= value;
                cpu.pc += 3;
            },
            // 2B, 6C
            ORA(Addr::XIndirect) => {
                let mut ptr = cpu.read_byte(cpu.pc + 1);
                ptr = ptr.wrapping_add(cpu.reg.x);
                let addr = cpu.read_word(ptr as Word);
                let value = cpu.read_byte(addr);
                cpu.reg.acc |= value;
                cpu.pc += 2;
            },
            // 2B, 5C (+1 if page crossed)
            ORA(Addr::IndirectY) => {
                let ptr = cpu.read_byte(cpu.pc + 1);
                let mut addr = cpu.read_word(ptr as Word);
                addr += cpu.reg.y as Word;
                let value = cpu.read_byte(addr);
                cpu.reg.acc |= value;
                cpu.pc += 2;
            },
            _ => panic!("Operation not supported!")
        }
        Self::set_flags(cpu);
    }

    fn code(&self) -> Byte {
        match self {
            ORA(Addr::Immediate) => 0x09,
            ORA(Addr::ZeroPage) => 0x05,
            ORA(Addr::ZeroPageX) => 0x15,
            ORA(Addr::Absolute) => 0x0D,
            ORA(Addr::AbsoluteX) => 0x1D,
            ORA(Addr::AbsoluteY) => 0x19,
            ORA(Addr::XIndirect) => 0x01,
            ORA(Addr::IndirectY) => 0x11,
            _ => panic!("Operation not supported!")
        }
    }
}
