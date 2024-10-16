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
                let result = value.wrapping_sub(1);
                cpu.write_byte(addr, result);
                Self::set_flags(cpu, result);
                cpu.pc += 2;
            },
            // 2B, 6C
            DEC(Addr::ZeroPageX) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                let addr = zp_addr.wrapping_add(cpu.reg.x) as Word;
                let value = cpu.read_byte(addr);
                let result = value.wrapping_sub(1);
                cpu.write_byte(addr, result);
                Self::set_flags(cpu, result);
                cpu.pc += 2;
            },
            // 3B, 6C
            DEC(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                let value = cpu.read_byte(addr);
                let result = value.wrapping_sub(1);
                cpu.write_byte(addr, result);
                Self::set_flags(cpu, result);
                cpu.pc += 3;
            },
            // 3B, 7C
            DEC(Addr::AbsoluteX) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.x as Word;
                let value = cpu.read_byte(addr);
                let result = value.wrapping_sub(1);
                cpu.write_byte(addr, result);
                Self::set_flags(cpu, result);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dec_zero_page() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.mem.write_byte(0xFFFC, DEC(Addr::ZeroPage).code());
        cpu.mem.write_byte(0xFFFD, 0x48);
        cpu.mem.write_byte(0x0048, 0x00);

        let cpu_start = cpu.clone();
        cpu.start();

        assert_eq!(cpu.mem.read_byte(0x0048), 0xFF);

        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, true);

        assert_eq!(cpu_start.pc, 0xFFFC);
        assert_eq!(cpu.pc, 0xFFFE);
    }

    #[test]
    fn dec_zero_page_x() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.x = 3;
        cpu.mem.write_byte(0xFFFC, DEC(Addr::ZeroPageX).code());
        cpu.mem.write_byte(0xFFFD, 0xFF);
        cpu.mem.write_byte(0x0002, 0x00); // 0xFF + 0x03 % 0xFF

        let cpu_start = cpu.clone();
        cpu.start();

        assert_eq!(cpu.mem.read_byte(0x0002), 0xFF);

        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, true);

        assert_eq!(cpu_start.pc, 0xFFFC);
        assert_eq!(cpu.pc, 0xFFFE);
    }

    #[test]
    fn dec_absolute() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.mem.write_byte(0xFFFC, DEC(Addr::Absolute).code());
        cpu.mem.write_byte(0xFFFD, 0x34);
        cpu.mem.write_byte(0xFFFE, 0x12); // 0x1234 (LE)
        cpu.mem.write_byte(0x1234, 0x05);

        let cpu_start = cpu.clone();
        cpu.start();

        assert_eq!(cpu.mem.read_byte(0x1234), 0x04);

        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, false);

        assert_eq!(cpu_start.pc, 0xFFFC);
        assert_eq!(cpu.pc, 0xFFFF);
    }

    #[test]
    fn dec_absolute_x() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.x = 0x12;
        cpu.mem.write_byte(0xFFFC, DEC(Addr::AbsoluteX).code());
        cpu.mem.write_byte(0xFFFD, 0x00);
        cpu.mem.write_byte(0xFFFE, 0x44); // 0x4400 (LE)
        cpu.mem.write_byte(0x4412, 0x05);

        let cpu_start = cpu.clone();
        cpu.start();

        assert_eq!(cpu.mem.read_byte(0x4412), 0x04);

        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, false);

        assert_eq!(cpu_start.pc, 0xFFFC);
        assert_eq!(cpu.pc, 0xFFFF);
    }
}
