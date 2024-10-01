use crate::{cpu::CPU, ins::Instruction, mem::{Addr, Memory}};
use crate::{Byte, Word};

/// Load Accummulator. Loads a byte of memory into the accumulator, setting the zero and
/// negative flags as appropriate.
pub struct LDX(pub Addr);

impl LDX {
    fn set_flags(&self, cpu: &mut CPU) {
        todo!()
    }
}

impl Instruction for LDX {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            // 2 bytes
            LDX(Addr::Immediate) => {
                cpu.reg.x = cpu.read_byte(cpu.pc + 1);
                cpu.pc += 2;
            },
            // 2 bytes
            LDX(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                cpu.reg.x = cpu.read_byte(zp_addr as Word);
            },
            // 2 bytes
            LDX(Addr::ZeroPageY) => {
                let mut zp_addr = cpu.read_byte(cpu.pc + 1);
                zp_addr += cpu.reg.y;
                cpu.reg.y = mem.read_byte(zp_addr as Word);
            },
            LDX(Addr::Absolute) => {
                let addr = cpu.read_word(mem);
                cpu.reg.x = mem.read_byte(addr);
            },
            LDX(Addr::AbsoluteY) => {
                let mut addr = cpu.read_word(mem);
                addr += cpu.reg.y as Word;
                cpu.reg.x = mem.read_byte(addr);
            }
            _ => panic!("Operation not supported!")
        }
        self.set_flags(cpu);
    }

    fn code(&self) -> Byte {
        match self {
            LDX(Addr::Immediate) => 0xA2,
            LDX(Addr::ZeroPage) => 0xA6,
            LDX(Addr::ZeroPageY) => 0xB6,
            LDX(Addr::Absolute) => 0xAE,
            LDX(Addr::AbsoluteY) => 0xBE,
            _ => panic!("Operation not supported!")
        }
    }

    // fn from_byte(code: Byte) -> Self {
    //     match code {
    //         0xA2 => LDX(Addr::Immediate),
    //         0xA6 => LDX(Addr::ZeroPage),
    //         0xB6 => LDX(Addr::ZeroPageY),
    //         0xAE => LDX(Addr::Absolute),
    //         0xBE => LDX(Addr::AbsoluteY),
    //         _ => panic!("Unable to identify instruction.")
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {

    }
}
