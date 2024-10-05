use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Clear Decimal Mode - Sets the decimal mode flag to zero.
pub struct CLD(pub Addr);

impl CLD {
    fn set_flags(cpu: &mut CPU) {
        cpu.flags.d = false;
    }
}

impl Instruction for CLD {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            CLD(Addr::Implicit) => {
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            CLD(Addr::Implicit) => 0xD8,
            _ => panic!("Operation not supported!")
        }
    }
}
