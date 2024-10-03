use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Transfer Accumulator to Y - Copies the current contents of the accumulator into the Y
/// register and sets the zero and negative flags as appropriate.
pub struct TAY(pub Addr);

impl TAY {
    fn set_flags(cpu: &mut CPU) {
        // Set zero flag if Y = 0
        cpu.flags.z = cpu.reg.y == 0;
        // Set negative flag if bit 7 of Y is set
        cpu.flags.n = (cpu.reg.y & 0b10000000) > 0;
    }
}

impl Instruction for TAY {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            TAY(Addr::Implicit) => {
                cpu.reg.y = cpu.reg.acc;
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
        Self::set_flags(cpu);
    }

    fn code(&self) -> Byte {
        match self {
            TAY(Addr::Implicit) => 0xA8,
            _ => panic!("Operation not supported!")
        }
    }
}
