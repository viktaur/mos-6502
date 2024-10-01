use crate::{cpu::CPU, ins::Instruction, mem::{Addr, Mem}};
use crate::{Byte, Word};

/// Load Accummulator. Loads a byte of memory into the accumulator, setting the zero and
/// negative flags as appropriate.
pub struct LDA(pub Addr);

impl LDA {
    fn set_flags(cpu: &mut CPU) {
        // Set if A = 0
        cpu.flags.z = cpu.reg.acc == 0;
        // Set if bit 7 of A is set
        cpu.flags.n = (cpu.reg.acc & 0b10000000) > 0;
    }
}

impl Instruction for LDA {
    fn execute(&self, cpu: &mut CPU, mem: &mut Mem) {
        match self {
            LDA(Addr::Immediate) => {
                cpu.reg.acc = cpu.read_byte(mem);
            },
            LDA(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(mem);
                cpu.reg.acc = mem.read_byte(zp_addr as Word);
            },
            LDA(Addr::ZeroPageX) => {
                let mut zp_addr = cpu.read_byte(mem);
                zp_addr = zp_addr.wrapping_add(cpu.reg.x);
                cpu.reg.acc = mem.read_byte(zp_addr as Word);
            },
            LDA(Addr::Absolute) => {
                let address = cpu.read_word(mem);
                cpu.reg.acc = mem.read_byte(address);
            },
            LDA(Addr::AbsoluteX) => {
                let mut address = cpu.read_word(mem);
                address += cpu.reg.x as Word;
                cpu.reg.acc = mem.read_byte(address);
            },
            LDA(Addr::AbsoluteY) => {
                let mut address = cpu.read_word(mem);
                address += cpu.reg.y as Word;
                cpu.reg.acc = mem.read_byte(address);
            },
            LDA(Addr::XIndirect) => {
                let mut ptr = cpu.read_byte(mem);
                ptr = ptr.wrapping_add(cpu.reg.x);
                let address = mem.read_word(ptr as Word);
                cpu.reg.acc = mem.read_byte(address);
            },
            LDA(Addr::IndirectY) => {
                let ptr = cpu.read_byte(mem);
                let mut address = mem.read_word(ptr as Word);
                address += cpu.reg.y as Word;
                cpu.reg.acc = mem.read_byte(address);
            },
            _ => panic!("Operation not supported!")
        }
        LDA::set_flags(cpu);
    }

    fn code(&self) -> Byte {
        match self {
            LDA(Addr::Immediate) => 0x49,
            LDA(Addr::ZeroPage) => 0xA5,
            LDA(Addr::ZeroPageX) => 0xB5,
            LDA(Addr::Absolute) => 0xAD,
            LDA(Addr::AbsoluteX) => 0xBD,
            LDA(Addr::AbsoluteY) => 0xB9,
            LDA(Addr::XIndirect) => 0xA1,
            LDA(Addr::IndirectY) => 0xB1,
            _ => panic!("Operation not supported!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CPU;
    use crate::mem::{Mem, Addr};

    #[test]
    fn lda_immediate() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        let cpu_start = cpu.clone();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, LDA(Addr::Immediate).code());
        mem.write_byte(0xFFFD, 0x84);
        cpu.start(&mut mem);

        assert_eq!(cpu.reg.acc, 0x84);

        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, true);
    }

    #[test]
    fn lda_zero_page() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        let cpu_start = cpu.clone();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, LDA(Addr::ZeroPage).code());
        mem.write_byte(0xFFFD, 0x42);
        mem.write_byte(0x0042, 0x84);
        cpu.start(&mut mem);

        assert_eq!(cpu.reg.acc, 0x84);

        // TODO check flags
        assert_eq!(cpu.flags.c, cpu_start.flags.c);
        assert_eq!(cpu.flags.z, false);
        assert_eq!(cpu.flags.i, cpu_start.flags.i);
        assert_eq!(cpu.flags.d, cpu_start.flags.d);
        assert_eq!(cpu.flags.b, cpu_start.flags.b);
        assert_eq!(cpu.flags.v, cpu_start.flags.v);
        assert_eq!(cpu.flags.n, true);
    }

    #[test]
    fn lda_zero_page_x() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        cpu.reg.x = 0x12;
        mem.write_byte(0xFFFC, LDA(Addr::ZeroPageX).code());
        mem.write_byte(0xFFFD, 0x42);
        mem.write_byte(0x0042 + 0x0012, 0x85);
        cpu.start(&mut mem);

        assert_eq!(cpu.reg.acc, 0x85);

        // TODO test flags
    }

    #[test]
    fn lda_absolute() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, LDA(Addr::Absolute).code());
        mem.write_byte(0xFFFD, 0x80);
        mem.write_byte(0xFFFE, 0x44); // 0x4480 (LE)
        mem.write_byte(0x4480, 0x37);
        cpu.start(&mut mem);

        assert_eq!(cpu.reg.acc, 0x37);

        // TODO test flags

    }

    #[test]
    fn lda_absolute_x() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        cpu.reg.x = 0x12;
        mem.write_byte(0xFFFC, LDA(Addr::AbsoluteX).code());
        mem.write_byte(0xFFFD, 0x00);
        mem.write_byte(0xFFFE, 0x44); // 0x4400 (LE)
        mem.write_byte(0x4412, 0x37);
        cpu.start(&mut mem);

        assert_eq!(cpu.reg.acc, 0x37);

        // TODO test flags

    }

    #[test]
    fn lda_absolute_y() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        cpu.reg.y = 0x12;
        mem.write_byte(0xFFFC, LDA(Addr::AbsoluteY).code());
        mem.write_byte(0xFFFD, 0x00);
        mem.write_byte(0xFFFE, 0x44); // 0x4400 (LE)
        mem.write_byte(0x4412, 0x37);
        cpu.start(&mut mem);

        assert_eq!(cpu.reg.acc, 0x37);

        // TODO test flags
    }

    #[test]
    fn lda_x_indirect() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        cpu.reg.x = 0x04;
        mem.write_byte(0xFFFC, LDA(Addr::XIndirect).code());
        mem.write_byte(0xFFFD, 0x02); // 0x02 + 0x04 = 0x06
        mem.write_byte(0x0006, 0x00);
        mem.write_byte(0x0007, 0x80); // 0x8000 (LE)
        mem.write_byte(0x8000, 0x37);
        cpu.start(&mut mem);

        assert_eq!(cpu.reg.acc, 0x37);

        // TODO test flags
    }

    #[test]
    fn lda_indirect_y() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        cpu.reg.y = 0x04;
        mem.write_byte(0xFFFC, LDA(Addr::IndirectY).code());
        mem.write_byte(0xFFFD, 0x02);
        mem.write_byte(0x0002, 0x00);
        mem.write_byte(0x0003, 0x80); // 0x8000 (LE)
        mem.write_byte(0x8004, 0x37); // 0x8000 + 0x0004
        cpu.start(&mut mem);

        assert_eq!(cpu.reg.acc, 0x37);

        // TODO test flags
    }
}
