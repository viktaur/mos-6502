use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::{Byte, Word};

/// Logical AND - Performs a bitwise AND operation between the value in the Accumulator
/// and the specified byte, storing the result in the Accumulator.
pub struct AND(pub Addr);

impl AND {
    fn set_flags(cpu: &mut CPU) {
        // Set zero flag if acc = 0
        cpu.flags.z = cpu.reg.acc == 0;
        // Set negative flag if bit 7 of acc is set
        cpu.flags.n = (cpu.reg.acc & 0b10000000) > 0;
    }
}

impl Instruction for AND {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 2B, 2C
            AND(Addr::Immediate) => {
                let value = cpu.read_byte(cpu.pc + 1);
                cpu.reg.acc &= value;
                cpu.pc += 2;
            },
            // 2B, 3C
            AND(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                let value = cpu.read_byte(zp_addr as Word);
                cpu.reg.acc &= value;
                cpu.pc += 2;
            },
            // 2B, 4C
            AND(Addr::ZeroPageX) => {
                let mut zp_addr = cpu.read_byte(cpu.pc + 1);
                zp_addr = zp_addr.wrapping_add(cpu.reg.x);
                let value = cpu.read_byte(zp_addr as Word);
                cpu.reg.acc &= value;
                cpu.pc += 2;
            },
            // 3B, 4C
            AND(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                let value = cpu.read_byte(addr);
                cpu.reg.acc &= value;
                cpu.pc += 3;
            },
            // 3B, 4C (+1 if page crossed)
            AND(Addr::AbsoluteX) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.x as Word;
                let value = cpu.read_byte(addr);
                cpu.reg.acc &= value;
                cpu.pc += 3;
            },
            // 3B, 4C (+1 if page crossed)
            AND(Addr::AbsoluteY) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.y as Word;
                let value = cpu.read_byte(addr);
                cpu.reg.acc &= value;
                cpu.pc += 3;
            },
            // 2B, 6C
            AND(Addr::XIndirect) => {
                let mut ptr = cpu.read_byte(cpu.pc + 1);
                ptr = ptr.wrapping_add(cpu.reg.x);
                let addr = cpu.read_word(ptr as Word);
                let value = cpu.read_byte(addr);
                cpu.reg.acc &= value;
                cpu.pc += 2;
            },
            // 2B, 5C (+1 if page crossed)
            AND(Addr::IndirectY) => {
                let ptr = cpu.read_byte(cpu.pc + 1);
                let mut addr = cpu.read_word(ptr as Word);
                addr += cpu.reg.y as Word;
                let value = cpu.read_byte(addr);
                cpu.reg.acc &= value;
                cpu.pc += 2;
            },
            _ => panic!("Operation not supported!")
        }
        Self::set_flags(cpu);
    }

    fn code(&self) -> Byte {
        match self {
            AND(Addr::Immediate) => 0x29,
            AND(Addr::ZeroPage) => 0x25,
            AND(Addr::ZeroPageX) => 0x35,
            AND(Addr::Absolute) => 0x2D,
            AND(Addr::AbsoluteX) => 0x3D,
            AND(Addr::AbsoluteY) => 0x39,
            AND(Addr::XIndirect) => 0x21,
            AND(Addr::IndirectY) => 0x31,
            _ => panic!("Operation not supported!")
        }
    }
}
