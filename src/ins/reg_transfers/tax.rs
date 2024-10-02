use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Transfer Accumulator to X - Copies the current contents of the accumulator into the X
/// register and sets the zero and negative flags as appropriate.
pub struct TAX(pub Addr);

impl TAX {
    fn set_flags(cpu: &mut CPU) {
        // Set zero flag if X = 0
        cpu.flags.z = cpu.reg.x == 0;
        // Set negative flag if bit 7 of X is set
        cpu.flags.n = (cpu.reg.x & 0b10000000) > 0;
    }
}

impl Instruction for TAX {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            TAX(Addr::Implicit) => {
                cpu.reg.x = cpu.reg.acc;
            },
            _ => panic!("Operation not supported!")
        }
        Self::set_flags(cpu);
    }

    fn code(&self) -> Byte {
        match self {
            TAX(Addr::Implicit) => 0xAA,
            _ => panic!("Operation not supported!")
        }
    }
}
