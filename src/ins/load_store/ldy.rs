use crate::{cpu::CPU, ins::Instruction, mem::{Addr, Memory}};
use crate::{Byte, Word};

/// Load Y Register. Loads a byte of memory into the Y register setting the zero and
/// negative flags as appropriate.
pub struct LDY(pub Addr);

impl LDY {
    fn set_flags(&self, cpu: &mut CPU) {
        // Set zero flag if register Y is 0
        cpu.flags.z = cpu.reg.y == 0;
        // Set negative flag is bit 7 of Y is set
        cpu.flags.n = (cpu.reg.y & 0b10000000) > 0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldy_immediate() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.mem.write_byte(0xFFFC, LDY(Addr::Immediate).code());
        cpu.mem.write_byte(0xFFFD, 0x84);

        let cpu_start = cpu.clone();
        cpu.start();

        assert_eq!(cpu.reg.y, 0x84);

        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, true);
    }

    #[test]
    fn ldy_zero_page() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.mem.write_byte(0xFFFC, LDY(Addr::ZeroPage).code());
        cpu.mem.write_byte(0xFFFD, 0x42);
        cpu.mem.write_byte(0x0042, 0x84);

        let cpu_start = cpu.clone();
        cpu.start();

        assert_eq!(cpu.reg.y, 0x84);

        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, true);
    }

    #[test]
    fn ldy_zero_page_y() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.x = 0x02;
        cpu.mem.write_byte(0xFFFC, LDY(Addr::ZeroPageX).code());
        cpu.mem.write_byte(0xFFFD, 0xFF);
        cpu.mem.write_byte(0x0001, 0x00); // 0xFF + 0x02 % 0xFF = 0x01

        let cpu_start = cpu.clone();
        cpu.start();

        assert_eq!(cpu.reg.y, 0x00);

        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, true);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, false);
    }

    #[test]
    fn ldy_absolute() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.mem.write_byte(0xFFFC, LDY(Addr::Absolute).code());
        cpu.mem.write_byte(0xFFFD, 0x33);
        cpu.mem.write_byte(0xFFFE, 0x44); // 0x4433 (LE)
        cpu.mem.write_byte(0x4433, 0x84);

        let cpu_start = cpu.clone();
        cpu.start();

        assert_eq!(cpu.reg.y, 0x84);

        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, true);
    }

    #[test]
    fn ldy_absolute_x() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.x = 0x02;
        cpu.mem.write_byte(0xFFFC, LDY(Addr::AbsoluteX).code());
        cpu.mem.write_byte(0xFFFD, 0x33);
        cpu.mem.write_byte(0xFFFE, 0x44); // 0x4433 (LE)
        cpu.mem.write_byte(0x4435, 0x84); // 0x4433 + 0x02

        let cpu_start = cpu.clone();
        cpu.start();

        assert_eq!(cpu.reg.y, 0x84);

        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, true);
    }
}
