use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Decrement Y Register - Decrements the value in the Y register by one, wrapping around
/// so that the result of decrementing $00 is $FF. The Carry flag is not affected.
pub struct DEY(pub Addr);

impl DEY {
    fn set_flags(cpu: &mut CPU) {
        // Set zero flag is Y is 0
        cpu.flags.z = cpu.reg.y == 0;
        // Set negative flag if bit 7 of Y is set
        cpu.flags.n = (0b10000000 & cpu.reg.y) > 0;
    }
}

impl Instruction for DEY {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            DEY(Addr::Implicit) => {
                cpu.reg.y = cpu.reg.y.wrapping_sub(1);
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            DEY(Addr::Implicit) => 0x88,
            _ => panic!("Operation not supported!")
        }
    }
}
