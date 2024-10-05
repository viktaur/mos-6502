use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Clear Carry Flag - Sets the carry flag to zero.
pub struct CLC(pub Addr);

impl CLC {
    fn set_flags(cpu: &mut CPU) {
        cpu.flags.c = false;
    }
}

impl Instruction for CLC {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            CLC(Addr::Implicit) => {
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            CLC(Addr::Implicit) => 0x18,
            _ => panic!("Operation not supported!")
        }
    }
}
