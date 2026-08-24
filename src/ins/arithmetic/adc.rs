use crate::ins::Instruction;
use crate::{cpu::CPU, mem::Addr};
use crate::{Byte, Word};

/// This instruction adds the contents of a memory location to the accumulator together
/// with the carry bit. If overflow occurs the carry bit is set, this enables multiple
/// byte addition to be performed.
pub struct ADC(pub Addr);

impl ADC {
    fn set_flags(cpu: &mut CPU, a: Byte, operand: Byte, sum: Word) {
        let sign_bits_the_same = (a ^ operand) & 0x80 == 0;
        let result_sign_differs = (a as Word ^ sum) & 0x80 != 0;

        // Set if overflow in bit 7
        cpu.flags.c = sum > 0xFF;
        // Set if A = 0
        cpu.flags.z = cpu.reg.acc == 0;
        // Set if sign bit is incorrect
        cpu.flags.v = sign_bits_the_same && result_sign_differs;
        // Set if bit 7 set
        cpu.flags.n = cpu.reg.acc & 0x80 != 0;
    }
}

impl Instruction for ADC {
    fn execute(&self, cpu: &mut CPU) {
        let a = cpu.reg.acc;
        // no side effects
        let add_with_carry = |operand: Byte| -> Word {
            let carry_in = if cpu.flags.c { 1u16 } else { 0u16 };
            let sum = a as Word + operand as Word + carry_in; // max 0x1FF = 511
            sum
        };

        match self {
            // 2B, 2C
            ADC(Addr::Immediate) => {
                let operand = cpu.read_byte(cpu.pc + 1);
                let sum = add_with_carry(operand);
                // we write only the 8 low bits. `sum` will still hold the full value.
                cpu.reg.acc = (sum & 0xFF) as Byte;
                cpu.pc += 2;
                cpu.cycles += 2;
                Self::set_flags(cpu, a, operand, sum);
            },
            // 2B, 3C
            ADC(Addr::ZeroPage) => {
                let zp_addr = cpu.read_byte(cpu.pc + 1);
                let operand = cpu.read_byte(zp_addr as Word);
                let sum = add_with_carry(operand);
                cpu.reg.acc = (sum & 0xFF) as Byte;
                cpu.pc += 2;
                cpu.cycles += 3;
                Self::set_flags(cpu, a, operand, sum);
            },
            // 2B, 4C
            ADC(Addr::ZeroPageX) => {
                let mut zp_addr = cpu.read_byte(cpu.pc + 1);
                zp_addr = zp_addr.wrapping_add(cpu.reg.x);
                let operand = cpu.read_byte(zp_addr as Word);
                let sum = add_with_carry(operand);
                cpu.reg.acc = (sum & 0xFF) as Byte;
                cpu.pc += 2;
                cpu.cycles += 4;
                Self::set_flags(cpu, a, operand, sum);
            },
            // 3B, 4C
            ADC(Addr::Absolute) => {
                let address = cpu.read_word(cpu.pc + 1);
                let operand = cpu.read_byte(address);
                let sum = add_with_carry(operand);
                cpu.reg.acc = (sum & 0xFF) as Byte;
                cpu.pc += 3;
                cpu.cycles += 4;
                Self::set_flags(cpu, a, operand, sum);
            },
            // 3B, 4C (+1 if page crossed)
            ADC(Addr::AbsoluteX) => {
                let mut address = cpu.read_word(cpu.pc + 1);
                address += cpu.reg.x as Word;
                let operand = cpu.read_byte(address);
                let sum = add_with_carry(operand);
                cpu.reg.acc = (sum & 0xFF) as Byte;
                cpu.pc += 3;
                cpu.cycles += 4; // TODO account for page crossing
                Self::set_flags(cpu, a, operand, sum);
            },

            // 3B, 4C (+1 if page crossed)
            ADC(Addr::AbsoluteY) => {
                let mut address = cpu.read_word(cpu.pc + 1);
                address += cpu.reg.y as Word;
                let operand = cpu.read_byte(address);
                let sum = add_with_carry(operand);
                cpu.reg.acc = (sum & 0xFF) as Byte;
                cpu.pc += 3;
                cpu.cycles += 4; // TODO account for page crossing
                Self::set_flags(cpu, a, operand, sum);
            },

            // 2B, 6C
            ADC(Addr::XIndirect) => {
                let mut ptr = cpu.read_byte(cpu.pc + 1);
                ptr = ptr.wrapping_add(cpu.reg.x);
                let address = cpu.read_word(ptr as Word);

            },

            // 2B, 5C (+1 if page crossed)
            ADC(Addr::IndirectY) => {
                todo!()
            },

            _ => panic!("Operation not supported!")
        }
    }

    fn code(&self) -> Byte {
        match self {
            ADC(Addr::Immediate) => 0x69,
            ADC(Addr::ZeroPage) => 0x65,
            ADC(Addr::ZeroPageX) => 0x75,
            ADC(Addr::Absolute) => 0x6D,
            ADC(Addr::AbsoluteX) => 0x7D,
            ADC(Addr::AbsoluteY) => 0x79,
            ADC(Addr::XIndirect) => 0x61,
            ADC(Addr::IndirectY) => 0x71,
            _ => panic!("Operation not supported!")
        }
    }
}
