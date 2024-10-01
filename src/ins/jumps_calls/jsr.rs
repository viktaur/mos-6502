use crate::{cpu::CPU, ins::Instruction, mem::{Addr, Memory}};
use crate::{Byte, Word};

pub struct JSR(pub Addr);

impl Instruction for JSR {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            JSR(Addr::Absolute) => {
                let sub_addr = cpu.read_word();

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
    use crate::mem::Addr;

    #[test]
    fn jsr_absolute() {
        let mut cpu = CPU::new();

        cpu.reset();
        cpu.mem.write_byte(0xFFFC, JSR(Addr::Absolute).code());
        cpu.mem.write_byte(0xFFFD, 0x42);
        cpu.mem.write_byte(0xFFFE, 0x42);
        cpu.start();

        assert_eq!(cpu.pc, 0x4242);

        // TODO test flags
    }
}
