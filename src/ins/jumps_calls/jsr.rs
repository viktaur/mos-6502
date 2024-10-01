use crate::{cpu::CPU, ins::Instruction, mem::{Addr, Mem}};
use crate::{Byte, Word};

pub struct JSR(pub Addr);

impl Instruction for JSR {
    fn execute(&self, cpu: &mut CPU, mem: &mut Mem) {
        match self {
            JSR(Addr::Absolute) => {
                let sub_addr = cpu.read_word(mem);
                mem.write_word(cpu.sp as Word, cpu.pc - 1);
                cpu.pc = sub_addr;
            },
            _ => panic!("Addressing method not supported.")
        }
    }

    fn code(&self) -> Byte {
        match self {
            JSR(Addr::Absolute) => 0x20,
            _ => panic!("Addressing method not supported.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CPU;
    use crate::mem::{Mem, Addr};

    #[test]
    fn jsr_absolute() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        cpu.reset(&mut mem);
        mem.write_byte(0xFFFC, JSR(Addr::Absolute).code());
        mem.write_byte(0xFFFD, 0x42);
        mem.write_byte(0xFFFE, 0x42);
        cpu.start(&mut mem);

        assert_eq!(cpu.pc, 0x4242);

        // TODO test flags
    }
}
