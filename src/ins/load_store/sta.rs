use crate::{ins::Instruction, mem::Addr, cpu::CPU};
use crate::{Byte, Word};

/// Store Accumulator - Store the contents of the accumulator register into memory.
pub struct STA(pub Addr);

impl Instruction for STA {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 2B, 3C
            STA(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                cpu.write_byte(zp_addr as Word, cpu.reg.acc);
                cpu.pc += 2
            },
            // 2B, 4C
            STA(Addr::ZeroPageX) => {
                let mut zp_addr = cpu.read_byte(cpu.pc + 1);
                zp_addr = zp_addr.wrapping_add(cpu.reg.x);
                cpu.write_byte(zp_addr as Word, cpu.reg.acc);
                cpu.pc += 2;
            },
            // 3B, 4C
            STA(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                cpu.write_byte(addr, cpu.reg.acc);
                cpu.pc += 3;
            },
            // 3B, 5C
            STA(Addr::AbsoluteX) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.x as Word;
                cpu.write_byte(addr, cpu.reg.acc);
                cpu.pc += 3;
            },
            // 3B, 5C
            STA(Addr::AbsoluteY) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.y as Word;
                cpu.write_byte(addr, cpu.reg.acc);
                cpu.pc += 3;
            },
            // 2B, 6C
            STA(Addr::XIndirect) => {
                let mut ptr = cpu.read_byte(cpu.pc + 1);
                ptr = ptr.wrapping_add(cpu.reg.x);
                let address = cpu.read_word(ptr as Word);
                cpu.write_byte(address, cpu.reg.acc);
                cpu.pc += 2;
            },
            // 2B, 6C
            STA(Addr::IndirectY) => {
                let ptr = cpu.read_byte(cpu.pc + 1);
                let mut address = cpu.read_word(ptr as Word);
                address += cpu.reg.y as Word;
                cpu.write_byte(address, cpu.reg.acc);
                cpu.pc += 2;
            }
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            STA(Addr::ZeroPage) => 0x85,
            STA(Addr::ZeroPageX) => 0x95,
            STA(Addr::Absolute) => 0x8D,
            STA(Addr::AbsoluteX) => 0x9D,
            STA(Addr::AbsoluteY) => 0x99,
            STA(Addr::XIndirect) => 0x81,
            STA(Addr::IndirectY) => 0x91,
            _ => panic!("Operation not supported!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sta_zero_page() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.acc = 0x42;
        cpu.mem.write_byte(0xFFFC, STA(Addr::ZeroPage).code());
        cpu.mem.write_byte(0xFFFD, 0x22);
        cpu.start();

        let value = cpu.mem.read_byte(0x0022);
        assert_eq!(value, cpu.reg.acc);
    }

    #[test]
    fn sta_zero_page_x() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.acc = 0x42;
        cpu.reg.x = 0x03;
        cpu.mem.write_byte(0xFFFC, STA(Addr::ZeroPageX).code());
        cpu.mem.write_byte(0xFFFD, 0xFE);
        cpu.start();

        let value = cpu.mem.read_byte(0x0001);
        assert_eq!(value, cpu.reg.acc);
    }

    #[test]
    fn sta_absolute() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.acc = 0x42;
        cpu.mem.write_byte(0xFFFC, STA(Addr::Absolute).code());
        cpu.mem.write_byte(0xFFFD, 0x22);
        cpu.mem.write_byte(0xFFFE, 0x44); // 0x4422 (LE)
        cpu.start();

        let value = cpu.mem.read_byte(0x4422);
        assert_eq!(value, cpu.reg.acc);
    }

    #[test]
    fn sta_absolute_x() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.acc = 0x42;
        cpu.reg.x = 0x05;
        cpu.mem.write_byte(0xFFFC, STA(Addr::AbsoluteX).code());
        cpu.mem.write_byte(0xFFFD, 0x22);
        cpu.mem.write_byte(0xFFFE, 0x44); // 0x4422 (LE)
        cpu.start();

        let value = cpu.mem.read_byte(0x4427);
        assert_eq!(value, cpu.reg.acc);
    }

    #[test]
    fn sta_absolute_y() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.acc = 0x42;
        cpu.reg.y = 0x05;
        cpu.mem.write_byte(0xFFFC, STA(Addr::AbsoluteY).code());
        cpu.mem.write_byte(0xFFFD, 0x22);
        cpu.mem.write_byte(0xFFFE, 0x44); // 0x4422 (LE)
        cpu.start();

        let value = cpu.mem.read_byte(0x4427);
        assert_eq!(value, cpu.reg.acc);
    }

    #[test]
    fn sta_absolute_x_indirect() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.acc = 0x42;
        cpu.reg.x = 0x05;
        cpu.mem.write_byte(0xFFFC, STA(Addr::XIndirect).code());
        cpu.mem.write_byte(0xFFFD, 0x60);
        cpu.mem.write_byte(0x0065, 0x22);
        cpu.mem.write_byte(0x0066, 0x44); // 0x4422 (LE)
        cpu.start();

        let value = cpu.mem.read_byte(0x4422);
        assert_eq!(value, cpu.reg.acc);
    }

    #[test]
    fn sta_absolute_indirect_y() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.reg.acc = 0x42;
        cpu.reg.y = 0x05;
        cpu.mem.write_byte(0xFFFC, STA(Addr::IndirectY).code());
        cpu.mem.write_byte(0xFFFD, 0x60);
        cpu.mem.write_byte(0x0060, 0x22);
        cpu.mem.write_byte(0x0061, 0x44); // 0x4422 (LE)
        cpu.start();

        let value = cpu.mem.read_byte(0x4427);
        assert_eq!(value, cpu.reg.acc);
    }
}
