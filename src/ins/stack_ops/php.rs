use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Push Processor Status - Pushes a copy of the status flags on to the stack.
pub struct PHP(pub Addr);

impl Instruction for PHP {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            PHP(Addr::Implicit) => {
                cpu.write_byte(CPU::stack_address(cpu.sp), cpu.flags.to_owned().into());
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
            PHP(Addr::Implicit) => 0x48,
            _ => panic!("Operation not supported!")
        }
    }
}
