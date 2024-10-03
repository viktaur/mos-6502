use crate::{cpu::CPU, ins::Instruction, mem::Addr};
use crate::Byte;

/// Transfer Stack Pointer to X - Copies the current contents of the stack pointer into
/// the X register and sets the zero and negative flags as appropriate.
pub struct TSX(pub Addr);

impl TSX {
    fn set_flags(cpu: &mut CPU) {
        // Set if X = 0
        cpu.flags.z = cpu.reg.x == 0;
        // Set if bit 7 of X is set
        cpu.flags.n = (cpu.reg.x & 0b10000000) > 0;
    }
}

impl Instruction for TSX {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 1B, 2C
            TSX(Addr::Implicit) => {
                // We assume it's actually the stack pointer value and not necessarily the
                // top of the stack.
                cpu.reg.x = cpu.read_byte(CPU::stack_address(cpu.sp));
                // Increment program counter
                cpu.pc += 1;
            },
            _ => panic!("Operation not supported!")
        }
        Self::set_flags(cpu);
    }

    fn code(&self) -> Byte {
        match self {
            TSX(Addr::Implicit) => 0xBA,
            _ => panic!("Operation not supported!")
        }
    }
}
