use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::{Byte, Word};

/// Logical EOR - Performs a bitwise EOR operation between the value in the Accumulator
/// and the specified byte, storing the result in the Accumulator.
pub struct EOR(pub Addr);

impl EOR {
    fn set_flags(cpu: &mut CPU) {
        // Set zero flag if acc = 0
        cpu.flags.z = cpu.reg.acc == 0;
        // Set negative flag if bit 7 of acc is set
        cpu.flags.n = (cpu.reg.acc & 0b10000000) > 0;
    }
}

impl Instruction for EOR {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 2B, 2C
            EOR(Addr::Immediate) => {
                let value = cpu.read_byte(cpu.pc + 1);
                cpu.reg.acc ^= value;
                cpu.pc += 2;
            },
            // 2B, 3C
            EOR(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                let value = cpu.read_byte(zp_addr as Word);
                cpu.reg.acc ^= value;
                cpu.pc += 2;
            },
            // 2B, 4C
            EOR(Addr::ZeroPageX) => {
                let mut zp_addr = cpu.read_byte(cpu.pc + 1);
                zp_addr = zp_addr.wrapping_add(cpu.reg.x);
                let value = cpu.read_byte(zp_addr as Word);
                cpu.reg.acc ^= value;
                cpu.pc += 2;
            },
            // 3B, 4C
            EOR(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                let value = cpu.read_byte(addr);
                cpu.reg.acc ^= value;
                cpu.pc += 3;
            },
            // 3B, 4C (+1 if page crossed)
            EOR(Addr::AbsoluteX) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.x as Word;
                let value = cpu.read_byte(addr);
                cpu.reg.acc ^= value;
                cpu.pc += 3;
            },
            // 3B, 4C (+1 if page crossed)
            EOR(Addr::AbsoluteY) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.y as Word;
                let value = cpu.read_byte(addr);
                cpu.reg.acc ^= value;
                cpu.pc += 3;
            },
            // 2B, 6C
            EOR(Addr::XIndirect) => {
                let mut ptr = cpu.read_byte(cpu.pc + 1);
                ptr = ptr.wrapping_add(cpu.reg.x);
                let addr = cpu.read_word(ptr as Word);
                let value = cpu.read_byte(addr);
                cpu.reg.acc ^= value;
                cpu.pc += 2;
            },
            // 2B, 5C (+1 if page crossed)
            EOR(Addr::IndirectY) => {
                let ptr = cpu.read_byte(cpu.pc + 1);
                let mut addr = cpu.read_word(ptr as Word);
                addr += cpu.reg.y as Word;
                let value = cpu.read_byte(addr);
                cpu.reg.acc ^= value;
                cpu.pc += 2;
            },
            _ => panic!("Operation not supported!")
        }
        Self::set_flags(cpu);
    }

    fn code(&self) -> Byte {
        match self {
            EOR(Addr::Immediate) => 0x49,
            EOR(Addr::ZeroPage) => 0x45,
            EOR(Addr::ZeroPageX) => 0x55,
            EOR(Addr::Absolute) => 0x4D,
            EOR(Addr::AbsoluteX) => 0x5D,
            EOR(Addr::AbsoluteY) => 0x59,
            EOR(Addr::XIndirect) => 0x41,
            EOR(Addr::IndirectY) => 0x51,
            _ => panic!("Operation not supported!")
        }
    }
}
