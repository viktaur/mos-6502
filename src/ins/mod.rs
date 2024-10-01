use load_store::{lda::LDA, ldx::LDX, ldy::LDY};
use jumps_calls::jsr::JSR;
use sys_funcs::brk::BRK;

use crate::mem::{Memory, Addr};
use crate::cpu::CPU;
use crate::{Byte, Word};

pub mod arithmetic;
pub mod branches;
pub mod inc_dec;
pub mod jumps_calls;
pub mod load_store;
pub mod logical;
pub mod shifts;
pub mod stack_ops;
pub mod status_flags;
pub mod sys_funcs;
pub mod transfers;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU);

    fn code(&self) -> Byte;
}

pub struct InstructionDecoder;

impl InstructionDecoder {
    pub fn from_byte(code: Byte) -> Box<dyn Instruction> {
        match code {
            0x49 => Box::new(LDA(Addr::Immediate)),
            0xA5 => Box::new(LDA(Addr::ZeroPage)),
            0xB5 => Box::new(LDA(Addr::ZeroPageX)),
            0xAD => Box::new(LDA(Addr::Absolute)),
            0xBD => Box::new(LDA(Addr::AbsoluteX)),
            0xB9 => Box::new(LDA(Addr::AbsoluteY)),
            0xA1 => Box::new(LDA(Addr::XIndirect)),
            0xB1 => Box::new(LDA(Addr::IndirectY)),
            0xA2 => Box::new(LDX(Addr::Immediate)),
            0xA6 => Box::new(LDX(Addr::ZeroPage)),
            0xB6 => Box::new(LDX(Addr::ZeroPageY)),
            0xAE => Box::new(LDX(Addr::Absolute)),
            0xA0 => Box::new(LDY(Addr::Immediate)),
            0xBE => Box::new(LDX(Addr::AbsoluteY)),
            0xA4 => Box::new(LDY(Addr::ZeroPage)),
            0xB4 => Box::new(LDY(Addr::ZeroPageX)),
            0xAC => Box::new(LDY(Addr::Absolute)),
            0xBC => Box::new(LDY(Addr::AbsoluteX)),
            0x20 => Box::new(JSR(Addr::Absolute)),
            0x00 => Box::new(BRK(Addr::Implicit)),
            _ => panic!()
        }
    }
}

pub trait DecodeIns {
    /// Decode instruction.
    fn decode(self) -> Box<dyn Instruction>;
}

impl DecodeIns for Byte {
    fn decode(self) -> Box<dyn Instruction> {
        InstructionDecoder::from_byte(self)
    }
}
