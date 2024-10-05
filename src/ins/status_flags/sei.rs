use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Set Interrupt Disable - Set the interrupt disable flag to one.
pub struct SEI(pub Addr);

impl SEI {
    fn set_flags(cpu: &mut CPU) {
        cpu.flags.c = true;
    }
}

impl Instruction for SEI {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            SEI(Addr::Implicit) => {
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            SEI(Addr::Implicit) => 0x78,
            _ => panic!("Operation not supported!")
        }
    }
}
