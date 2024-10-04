use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::{Byte, Word};

/// Decrement Memory - Decrements the value in the specified byte in memory by one,
/// wrapping around so that the result of decrementing $00 is $FF. The Carry flag is not
/// affected.
pub struct DEC(pub Addr);

impl DEC {
    pub fn set_flags(cpu: &mut CPU, value: Byte) {
        // Set zero flag if the result is 0
        cpu.flags.z = value == 0;
        // Updated negative flag to the value of bit #7 of the result.
        cpu.flags.n = (0b10000000 & value) > 0;
    }
}

impl Instruction for DEC {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 2B, 5C
            DEC(Addr::ZeroPage) => {
                let addr = cpu.read_byte(cpu.pc + 1) as Word;
                let value = cpu.read_byte(addr);
                cpu.write_byte(addr, value.wrapping_sub(1));
                Self::set_flags(cpu, value);
                cpu.pc += 2;
            },
            // 2B, 6C
            DEC(Addr::ZeroPageX) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                let addr = zp_addr.wrapping_add(cpu.reg.x) as Word;
                let value = cpu.read_byte(addr);
                cpu.write_byte(addr, value.wrapping_sub(1));
                Self::set_flags(cpu, value);
                cpu.pc += 2;
            },
            // 3B, 6C
            DEC(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                let value = cpu.read_byte(addr);
                cpu.write_byte(addr, value.wrapping_sub(1));
                Self::set_flags(cpu, value);
                cpu.pc += 3;
            },
            // 3B, 7C
            DEC(Addr::AbsoluteX) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.x as Word;
                let value = cpu.read_byte(addr);
                cpu.write_byte(addr, value.wrapping_sub(1));
                Self::set_flags(cpu, value);
                cpu.pc += 3;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            DEC(Addr::ZeroPage) => 0xC6,
            DEC(Addr::ZeroPageX) => 0xD6,
            DEC(Addr::Absolute) => 0xCE,
            DEC(Addr::AbsoluteX) => 0xDE,
            _ => panic!("Operation not supported!")
        }
    }
}
