mod cpu;
mod mem;

// These represent the types of the emulated 6502 CPU.
type Byte = u8;
type Word = u16;

#[cfg(test)]
mod tests {
    use crate::cpu::{CPU, Ins};
    use crate::mem::{Mem, Addressing};

    #[test]
    fn ins_lda_im() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, Ins::LDA(Addressing::Immediate).code());
        mem.write_byte(0xFFFD, 0x84);
        cpu.execute(&mut mem);

        assert_eq!(cpu.registers.a, 0x84)
    }

    #[test]
    fn ins_lda_zp() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, Ins::LDA(Addressing::ZeroPage).code());
        mem.write_byte(0xFFFD, 0x42);
        mem.write_byte(0x0042, 0x84);
        cpu.execute(&mut mem);

        assert_eq!(cpu.registers.a, 0x84);
    }

    #[test]
    fn ins_lda_zpx() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, Ins::LDA(Addressing::ZeroPageX).code());
        mem.write_byte(0xFFFD, 0x42);
        mem.write_byte(0xFFFE, 0x42);
        mem.write_byte(0x4242, 0x84);
        cpu.execute(&mut mem);

        assert_eq!(cpu.registers.a, 0x84);
    }

    #[test]
    fn ins_jsr() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, Ins::JSR(Addressing::Absolute).code());
        mem.write_byte(0xFFFD, 0x42);
        mem.write_byte(0xFFFE, 0x42);
        cpu.execute(&mut mem);

        assert_eq!(cpu.pc, 0x4242);
    }
}
