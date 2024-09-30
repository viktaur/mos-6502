pub mod cpu;
pub mod mem;
pub mod ins;

// These represent the types of the emulated 6502 CPU.
type Byte = u8;
type Word = u16;

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;
    use crate::ins::Ins;
    use crate::mem::{Mem, Addr};

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
