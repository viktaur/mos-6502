use crate::{ins::Instruction, mem::Addr};
use crate::cpu::CPU;

/// The BRK instruction forces the generation of an interrupt request. The program counter
/// and processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F
/// is loaded into the PC and the break flag in the status set to one.
pub struct BRK(pub Addr);

impl Instruction for BRK {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            BRK(Addr::Implicit) => {
                cpu.flags.b = true;

                // TODO push the pc and processor status onto the stack. Check if the
                // following is correct.
                cpu.pc = 0xFFFE;
                // Terminate the execution.
                std::process::exit(0);
            },
            _ => panic!("Addressing method not supported")
        }
    }

    fn code(&self) -> crate::Byte {
        match self {
            BRK(Addr::Implicit) => 0x00,
            _ => panic!("Operation not supported")
        }
    }
}
