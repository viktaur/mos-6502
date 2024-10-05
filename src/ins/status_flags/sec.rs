use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Set Carry Flag - Sets the carry flag to one.
pub struct SEC(pub Addr);

impl SEC {
    fn set_flags(cpu: &mut CPU) {
        cpu.flags.c = true;
    }
}

impl Instruction for SEC {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            SEC(Addr::Implicit) => {
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            SEC(Addr::Implicit) => 0x38,
            _ => panic!("Operation not supported!")
        }
    }
}
