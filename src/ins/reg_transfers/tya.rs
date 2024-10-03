use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Transfer Y to Accumulator - Copies the current contents of the Y register into the
/// accumulator and sets the zero and negative flags as appropriate.
pub struct TYA(pub Addr);

impl TYA {
    fn set_flags(cpu: &mut CPU) {
        // Set zero flag if A = 0
        cpu.flags.z = cpu.reg.acc == 0;
        // Set negative flag if bit 7 of A is set
        cpu.flags.n = (cpu.reg.acc & 0b10000000) > 0;
    }
}

impl Instruction for TYA {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            TYA(Addr::Implicit) => {
                cpu.reg.acc = cpu.reg.y;
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
        Self::set_flags(cpu);
    }

    fn code(&self) -> Byte {
        match self {
            TYA(Addr::Implicit) => 0x98,
            _ => panic!("Operation not supported!")
        }
    }
}
