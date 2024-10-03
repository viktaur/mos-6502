use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Transfer X to Stack Pointer - Copies the current contents of the X register into the
/// stack pointer.
pub struct TXS(pub Addr);

impl Instruction for TXS {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            TXS(Addr::Implicit) => {
                cpu.write_byte(CPU::stack_address(cpu.sp), cpu.reg.x);
                // Increment program counter
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            TXS(Addr::Implicit) => 0x9A,
            _ => panic!("Operation not supported!")
        }
    }
}
