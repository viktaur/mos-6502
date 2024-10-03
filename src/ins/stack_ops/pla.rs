use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Pull Accumulator - Pops the topmost byte from the stack and stores it in the
/// accumulator.
pub struct PLA(pub Addr);

impl Instruction for PLA {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 4C
            PLA(Addr::Implicit) => {
                // TODO handle overflow case (when stack is empty)
                // Read the value at the top of the stack
                cpu.reg.acc = cpu.read_byte(CPU::stack_address(cpu.sp + 1));
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
            PLA(Addr::Implicit) => 0x68,
            _ => panic!("Operation not supported!")
        }
    }
}
