pub mod cpu;
pub mod mem;
pub mod ins;
pub mod asm;

// These represent the types of the emulated 6502 CPU.
type Byte = u8;
type Word = u16;
