use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Decrement X Register - Decrements the value in the X register by one, wrapping around
/// so that the result of decrementing $00 is $FF. The Carry flag is not affected.
pub struct DEX(pub Addr);

impl DEX {
    fn set_flags(cpu: &mut CPU) {
        // Set zero flag is X is 0
        cpu.flags.z = cpu.reg.x == 0;
        // Set negative flag if bit 7 of X is set
        cpu.flags.n = (0b10000000 & cpu.reg.x) > 0;
    }
}

impl Instruction for DEX {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            DEX(Addr::Implicit) => {
                cpu.reg.x = cpu.reg.x.wrapping_sub(1);
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            DEX(Addr::Implicit) => 0xCA,
            _ => panic!("Operation not supported!")
        }
    }
}
