use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Increment X Register - Increments the value in the X register by one, wrapping around
/// so that the result of incrementing $FF is $00. The Carry flag is not affected.
pub struct INX(pub Addr);

impl INX {
    fn set_flags(cpu: &mut CPU) {
        // Set zero flag is X is 0
        cpu.flags.z = cpu.reg.x == 0;
        // Set negative flag if bit 7 of X is set
        cpu.flags.n = (0b10000000 & cpu.reg.x) > 0;
    }
}

impl Instruction for INX {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            INX(Addr::Implicit) => {
                cpu.reg.x = cpu.reg.x.wrapping_add(1);
                Self::set_flags(cpu);
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            INX(Addr::Implicit) => 0xE8,
            _ => panic!("Operation not supported!")
        }
    }
}
