use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::{Byte, Word};

/// Bit Test - Performs a bitwise AND operation between the value in the Accumulator and
/// the specified byte in the CPU's address space. The value in the Accumulator is not
/// updated.
pub struct BIT(pub Addr);

impl BIT {
    fn set_flags(cpu: &mut CPU, mem_value: Byte, result: Byte) {
        // Zero flag: Set if the result of the AND operation is zero (none of the bits
        // tested were set in both bytes), otherwise cleared.
        cpu.flags.z = result == 0;
        // Overflow flag: Update to bit 6 of the specified memory byte.
        cpu.flags.v = (mem_value & 0b01000000) > 0;
        // Negative flag: Update to bit 7 of the specified memory byte.
        cpu.flags.n = (mem_value & 0b10000000) > 0;
    }
}

impl Instruction for BIT {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 2B, 3C
            BIT(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                let mem_value = cpu.read_byte(zp_addr as Word);
                let result = cpu.reg.acc & mem_value;
                Self::set_flags(cpu, mem_value, result);
                cpu.pc += 2;
            },
            // 3B, 4C
            BIT(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                let mem_value = cpu.read_byte(addr);
                let result = cpu.reg.acc & mem_value;
                Self::set_flags(cpu, mem_value, result);
                cpu.pc += 3;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            BIT(Addr::ZeroPage) => 0x24,
            BIT(Addr::Absolute) => 0x2C,
            _ => panic!("Operation not supported!")
        }
    }
}
