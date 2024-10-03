use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Push Accumulator - Pushes a copy of the accumulator on to the stack.
pub struct PHA(pub Addr);

impl Instruction for PHA {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            PHA(Addr::Implicit) => {
                cpu.write_byte(CPU::stack_address(cpu.sp), cpu.reg.acc);
                // Decrease stack pointer (PUSH)
                cpu.sp -= 1;
                // Increase program counter
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            PHA(Addr::Implicit) => 0x48,
            _ => panic!("Operation not supported!")
        }
    }
}
