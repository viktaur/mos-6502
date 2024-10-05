use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Set Decimal Flag - Sets the deimal mode flag to one.
pub struct SED(pub Addr);

impl SED {
    fn set_flags(cpu: &mut CPU) {
        cpu.flags.d = true;
    }
}

impl Instruction for SED {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            SED(Addr::Implicit) => {
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            SED(Addr::Implicit) => 0xF8,
            _ => panic!("Operation not supported!")
        }
    }
}
