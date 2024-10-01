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
            // 2B, 2C
            LDX(Addr::Immediate) => {
                cpu.reg.x = cpu.read_byte(cpu.pc + 1);
                cpu.pc += 2;
            },
            // 2B, 3C
            LDX(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                cpu.reg.x = cpu.read_byte(zp_addr as Word);
                cpu.pc += 2;
            },
            // 2B, 4C
            LDX(Addr::ZeroPageY) => {
                let mut zp_addr = cpu.read_byte(cpu.pc + 1);
                zp_addr += cpu.reg.y;
                cpu.reg.y = cpu.read_byte(zp_addr as Word);
                cpu.pc += 2;
            },
            // 3B, 4C
            LDX(Addr::Absolute) => {
                let addr = cpu.read_word(cpu.pc + 1);
                cpu.reg.x = cpu.read_byte(addr);
                cpu.pc += 3;
            },
            // 3B, 4C (+1 if page crossed)
            LDX(Addr::AbsoluteY) => {
                let mut addr = cpu.read_word(cpu.pc + 1);
                addr += cpu.reg.y as Word;
                cpu.reg.x = cpu.read_byte(addr);
                cpu.pc += 3;
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
