use load_store::{lda::LDA, ldx::LDX, ldy::LDY, sta::STA, stx::STX, sty::STY};
use reg_transfers::{tax::TAX, tay::TAY, txa::TXA, tya::TYA};
use stack_ops::{tsx::TSX, txs::TXS, pha::PHA, php::PHP, pla::PLA, plp::PLP};
use inc_dec::{inc::INC, inx::INX, iny::INY, dec::DEC, dex::DEX, dey::DEY};
use jumps_calls::jsr::JSR;
use sys_funcs::brk::BRK;

use crate::mem::Addr;
use crate::cpu::CPU;
use crate::Byte;

pub mod arithmetic;
pub mod branches;
pub mod inc_dec;
pub mod jumps_calls;
pub mod load_store;
pub mod logical;
pub mod reg_transfers;
pub mod shifts;
pub mod stack_ops;
pub mod status_flags;
pub mod sys_funcs;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU);

    fn code(&self) -> Byte;
}

pub struct InstructionDecoder;

impl InstructionDecoder {
    pub fn from_byte(code: Byte) -> Box<dyn Instruction> {
        match code {
            // Load / Store
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
            0xBE => Box::new(LDX(Addr::AbsoluteY)),

            0xA0 => Box::new(LDY(Addr::Immediate)),
            0xA4 => Box::new(LDY(Addr::ZeroPage)),
            0xB4 => Box::new(LDY(Addr::ZeroPageX)),
            0xAC => Box::new(LDY(Addr::Absolute)),
            0xBC => Box::new(LDY(Addr::AbsoluteX)),

            0x85 => Box::new(STA(Addr::ZeroPage)),
            0x95 => Box::new(STA(Addr::ZeroPageX)),
            0x8D => Box::new(STA(Addr::Absolute)),
            0x9D => Box::new(STA(Addr::AbsoluteX)),
            0x99 => Box::new(STA(Addr::AbsoluteY)),
            0x81 => Box::new(STA(Addr::XIndirect)),
            0x91 => Box::new(STA(Addr::IndirectY)),

            0x86 => Box::new(STX(Addr::ZeroPage)),
            0x96 => Box::new(STX(Addr::ZeroPageY)),
            0x8E => Box::new(STX(Addr::Absolute)),

            0x84 => Box::new(STY(Addr::ZeroPage)),
            0x94 => Box::new(STY(Addr::ZeroPageX)),
            0x8C => Box::new(STY(Addr::Absolute)),


            // Register Transfers
            0xAA => Box::new(TAX(Addr::Implicit)),

            0xA8 => Box::new(TAY(Addr::Implicit)),

            0x8A => Box::new(TXA(Addr::Implicit)),

            0x98 => Box::new(TYA(Addr::Implicit)),

            // Stack operations
            0xBA => Box::new(TSX(Addr::Implicit)),

            0x9A => Box::new(TXS(Addr::Implicit)),

            0x48 => Box::new(PHA(Addr::Implicit)),

            0x08 => Box::new(PHP(Addr::Implicit)),

            0x68 => Box::new(PLA(Addr::Implicit)),

            0x28 => Box::new(PLP(Addr::Implicit)),

            // Increments & Decrements
            0xE6 => Box::new(INC(Addr::ZeroPage)),
            0xF6 => Box::new(INC(Addr::ZeroPageX)),
            0xEE => Box::new(INC(Addr::Absolute)),
            0xFE => Box::new(INC(Addr::AbsoluteX)),

            0xE8 => Box::new(INX(Addr::Implicit)),

            0xC8 => Box::new(INY(Addr::Implicit)),

            0xC6 => Box::new(DEC(Addr::ZeroPage)),
            0xD6 => Box::new(DEC(Addr::ZeroPageX)),
            0xCE => Box::new(DEC(Addr::Absolute)),
            0xDE => Box::new(DEC(Addr::AbsoluteX)),

            0xCA => Box::new(DEX(Addr::Implicit)),

            0x88 => Box::new(DEY(Addr::Implicit)),

            // Jumps & Calls
            0x20 => Box::new(JSR(Addr::Absolute)),

            // System Functions
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
