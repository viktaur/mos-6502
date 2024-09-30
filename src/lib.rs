pub mod cpu;
pub mod mem;

// These represent the types of the emulated 6502 CPU.
type Byte = u8;
type Word = u16;

#[cfg(test)]
mod tests {
    use crate::cpu::{CPU, Ins};
    use crate::mem::{Mem, Addr};

    #[test]
    fn ins_lda_im() {
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
    fn ins_lda_zp() {
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
    fn ins_lda_zpx() {
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
    fn ins_lda_abs() {
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
    fn ins_lda_absx() {
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
    fn ins_lda_absy() {
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
    fn ins_lda_indx() {
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
    fn ins_lda_indy() {
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

    #[test]
    fn ins_jsr() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, Ins::JSR(Addr::Absolute).code());
        mem.write_byte(0xFFFD, 0x42);
        mem.write_byte(0xFFFE, 0x42);
        cpu.execute(&mut mem);

        assert_eq!(cpu.pc, 0x4242);

        // TODO test flags
    }
}
