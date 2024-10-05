use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Clear Interrupt Disable - Clears the interrupt disable flag allowing normal interrupt
/// requests to be serviced.
pub struct CLI(pub Addr);

impl CLI {
    fn set_flags(cpu: &mut CPU) {
        cpu.flags.i = false;
    }
}

impl Instruction for CLI {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            CLI(Addr::Implicit) => {
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            CLI(Addr::Implicit) => 0x58,
            _ => panic!("Operation not supported!")
        }
    }
}
