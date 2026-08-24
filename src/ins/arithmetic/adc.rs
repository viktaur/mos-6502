use crate::ins::Instruction;
use crate::mem::Resolved;
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
        let Resolved { address, page_crossed } = self.0.resolve(cpu);
        let operand = cpu.read_byte(address);

        let carry_in = if cpu.flags.c { 1u16 } else { 0u16 };
        let sum = a as Word + operand as Word + carry_in; // max 0x1FF = 511

        // we write only the 8 low bits. `sum` will still hold the full value.
        cpu.reg.acc = (sum & 0xFF) as Byte;

        let (b, c) = self.bytes_and_cycles(page_crossed);
        cpu.pc += b;
        cpu.cycles += c;

        Self::set_flags(cpu, a, operand, sum);
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

    fn bytes_and_cycles(&self, page_crossed: bool) -> (Word, u32) {
        match self {
            ADC(Addr::Immediate) => (2, 2),
            ADC(Addr::ZeroPage) => (2, 3),
            ADC(Addr::ZeroPageX) => (2, 4),
            ADC(Addr::Absolute) => (3, 4),
            ADC(Addr::AbsoluteX) => (3, 4 + page_crossed as u32),
            ADC(Addr::AbsoluteY) => (3, 4 + page_crossed as u32),
            ADC(Addr::XIndirect) => (2, 6),
            ADC(Addr::IndirectY) => (2, 5 + page_crossed as u32),
            _ => panic!("Operation not supported!")
        }
    }
}
