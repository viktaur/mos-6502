use crate::{cpu::CPU, ins::Instruction, mem::{Addr, Mem}};
use crate::{Byte, Word};

/// Load Accummulator. Loads a byte of memory into the accumulator, setting the zero and
/// negative flags as appropriate.
pub struct LDA(Addr);

impl LDA {
    fn set_flags(&self, cpu: &mut CPU) {
        todo!()
    }
}

impl Instruction for LDA {
    fn execute(&self, cpu: &mut CPU, mem: &mut Mem) {
        match self {
            LDA(Addr::Immediate) => {
                cpu.reg.acc = cpu.fetch_byte(mem);
                self.set_flags(cpu);
            },
            LDA(Addr::ZeroPage) => {
                let zero_page_addr = cpu.fetch_byte(mem);
                cpu.reg.acc = mem.read_byte(zero_page_addr as Word);
                self.set_flags(cpu);
            },
            LDA(Addr::ZeroPageX) => {
                let mut zero_page_addr = cpu.fetch_byte(mem);
                zero_page_addr += cpu.reg.x;
                cpu.reg.acc = mem.read_byte(zero_page_addr as Word);
                self.set_flags(cpu);
            },
            LDA(Addr::Absolute) => {
                let address = cpu.fetch_word(mem);
                cpu.reg.acc = mem.read_byte(address);
                self.set_flags(cpu);
            },
            LDA(Addr::AbsoluteX) => {
                let mut address = cpu.fetch_word(mem);
                address += cpu.reg.x as Word;
                cpu.reg.acc = mem.read_byte(address);
                self.set_flags(cpu);
            },
            LDA(Addr::AbsoluteY) => {
                let mut address = cpu.fetch_word(mem);
                address += cpu.reg.y as Word;
                cpu.reg.acc = mem.read_byte(address);
                self.set_flags(cpu);
            },
            LDA(Addr::XIndirect) => {
                let mut ptr = cpu.fetch_byte(mem);
                ptr += cpu.reg.x;
                let address = mem.read_word(ptr as Word);
                cpu.reg.acc = mem.read_byte(address);
                self.set_flags(cpu);
            },
            LDA(Addr::IndirectY) => {
                let ptr = cpu.fetch_byte(mem);
                let mut address = mem.read_word(ptr as Word);
                address += cpu.reg.y as Word;
                cpu.reg.acc = mem.read_byte(address);
                self.set_flags(cpu);
            },
            _ => panic!("Operation not supported!")
        }
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

    fn from_byte(code: Byte) -> Self {
        match code {
            0x49 => LDA(Addr::Immediate),
            0xA5 => LDA(Addr::ZeroPage),
            0xB5 => LDA(Addr::ZeroPageX),
            0xAD => LDA(Addr::Absolute),
            0xBD => LDA(Addr::AbsoluteX),
            0xB9 => LDA(Addr::AbsoluteY),
            0xA1 => LDA(Addr::XIndirect),
            0xB1 => LDA(Addr::IndirectY),
            _ => panic!("Unable to identify instruction.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;
    use crate::ins::Ins;
    use crate::mem::{Mem, Addr};

    #[test]
    fn lda_immediate() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        let cpu_start = cpu.clone();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, Ins::LDA(Addr::Immediate).code());
        mem.write_byte(0xFFFD, 0x84);
        cpu.execute(&mut mem);

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
        mem.write_byte(0xFFFC, Ins::LDA(Addr::ZeroPage).code());
        mem.write_byte(0xFFFD, 0x42);
        mem.write_byte(0x0042, 0x84);
        cpu.execute(&mut mem);

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
        mem.write_byte(0xFFFC, Ins::LDA(Addr::ZeroPageX).code());
        mem.write_byte(0xFFFD, 0x42);
        mem.write_byte(0x0042 + 0x0012, 0x85);
        cpu.execute(&mut mem);

        assert_eq!(cpu.reg.acc, 0x85);

        // TODO test flags
    }

    #[test]
    fn lda_absolute() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, Ins::LDA(Addr::Absolute).code());
        mem.write_byte(0xFFFD, 0x80);
        mem.write_byte(0xFFFE, 0x44); // 0x4480 (LE)
        mem.write_byte(0x4480, 0x37);
        cpu.execute(&mut mem);

        assert_eq!(cpu.reg.acc, 0x37);

        // TODO test flags

    }

    #[test]
    fn lda_absolute_x() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        cpu.reg.x = 0x12;
        mem.write_byte(0xFFFC, Ins::LDA(Addr::AbsoluteX).code());
        mem.write_byte(0xFFFD, 0x00);
        mem.write_byte(0xFFFE, 0x44); // 0x4400 (LE)
        mem.write_byte(0x4412, 0x37);
        cpu.execute(&mut mem);

        assert_eq!(cpu.reg.acc, 0x37);

        // TODO test flags

    }

    #[test]
    fn lda_absolute_y() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        cpu.reg.y = 0x12;
        mem.write_byte(0xFFFC, Ins::LDA(Addr::AbsoluteY).code());
        mem.write_byte(0xFFFD, 0x00);
        mem.write_byte(0xFFFE, 0x44); // 0x4400 (LE)
        mem.write_byte(0x4412, 0x37);
        cpu.execute(&mut mem);

        assert_eq!(cpu.reg.acc, 0x37);

        // TODO test flags
    }

    #[test]
    fn lda_x_indirect() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        cpu.reg.x = 0x04;
        mem.write_byte(0xFFFC, Ins::LDA(Addr::XIndirect).code());
        mem.write_byte(0xFFFD, 0x02); // 0x02 + 0x04 = 0x06
        mem.write_byte(0x0006, 0x00);
        mem.write_byte(0x0007, 0x80); // 0x8000 (LE)
        mem.write_byte(0x8000, 0x37);
        cpu.execute(&mut mem);

        assert_eq!(cpu.reg.acc, 0x37);

        // TODO test flags
    }

    #[test]
    fn lda_indirect_y() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        cpu.reg.y = 0x04;
        mem.write_byte(0xFFFC, Ins::LDA(Addr::IndirectY).code());
        mem.write_byte(0xFFFD, 0x02);
        mem.write_byte(0x0002, 0x00);
        mem.write_byte(0x0003, 0x80); // 0x8000 (LE)
        mem.write_byte(0x8004, 0x37); // 0x8000 + 0x0004
        cpu.execute(&mut mem);

        assert_eq!(cpu.reg.acc, 0x37);

        // TODO test flags
    }
}
