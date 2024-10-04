use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Increment Y Register - Increments the value in the X register by one, wrapping around
/// so that the result of incrementing $FF is $00. The Carry flag is not affected.
pub struct INY(pub Addr);

impl INY {
    fn set_flags(cpu: &mut CPU) {
        // Set zero flag is Y is 0
        cpu.flags.z = cpu.reg.y == 0;
        // Set negative flag if bit 7 of Y is set
        cpu.flags.n = (0b10000000 & cpu.reg.y) > 0;
    }
}

impl Instruction for INY {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            INY(Addr::Implicit) => {
                cpu.reg.y = cpu.reg.y.wrapping_add(1);
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            INY(Addr::Implicit) => 0xC8,
            _ => panic!("Operation not supported!")
        }
    }
}
