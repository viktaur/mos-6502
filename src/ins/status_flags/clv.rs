use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Clear Overflow Flag - Clears the Overflow flag of the Processor Status register.
pub struct CLV(pub Addr);

impl CLV {
    fn set_flags(cpu: &mut CPU) {
        cpu.flags.v = false;
    }
}

impl Instruction for CLV {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            CLV(Addr::Implicit) => {
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            CLV(Addr::Implicit) => 0xB8,
            _ => panic!("Operation not supported!")
        }
    }
}
