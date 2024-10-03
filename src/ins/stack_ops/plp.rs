use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Pull Processor Status - Pulls an 8 bit value from the stack and into the processor
/// flags. The flags will take on new states as determined by the value pulled.
pub struct PLP(pub Addr);

impl Instruction for PLP {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            PLP(Addr::Implicit) => {
                // Read the value from the top of the stack and transform it into `StatusFlags`
                cpu.flags = cpu.read_byte(CPU::stack_address(cpu.sp + 1)).into();
                // Increment stack pointer (POP)
                cpu.sp += 1;
                // Increment program counter
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            PLP(Addr::Implicit) => 0x28,
            _ => panic!("Operation not supported!")
        }
    }
}
