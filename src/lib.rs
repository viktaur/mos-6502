mod cpu;
mod ins;
mod mem;

// These represent the types of the emulated 6502 CPU.
type Byte = u8;
type Word = u16;

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;
    use crate::mem::Mem;
    use crate::ins::Instruction;
    use num::ToPrimitive;

    #[test]
    fn test_name() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, Instruction::INS_LDA_IM.to_u8().unwrap());
        mem.write_byte(0xFFFD, 0x42);
        mem.write_byte(0xFFFE, 0x84);
        cpu.execute(&mut mem);
        println!("Successfully executed")
    }
}
