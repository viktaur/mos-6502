use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Instruction {
    /// Load Accumulator (LDA) Intermediate.
    INS_LDA_IM = 0xA9,
    /// Load Accummulator (LDA) Zero Page.
    INS_LDA_ZP = 0xA5,
    /// Load Accummulator (LDA) Zero Page X.
    INS_LDA_ZPX = 0xB5,
    /// Jump to Subroutine
    INS_JSR = 0x20,
}
