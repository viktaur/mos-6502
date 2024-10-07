# MOS 6502 emulator
The [MOS Technology 6502](https://en.wikipedia.org/wiki/MOS_Technology_6502) is an early 8-bit microprocessor introduced in 1975. It consists of only a
few internal registers, 56 instructions with multiple addressing modes, and 64 KB of
memory space (using 16-bit address words).

Given its relatively simple design, the 6502 is a fantastic architecture to learn how CPUs
actually work and how they can be emulated.

<img src="https://github.com/user-attachments/assets/d5dd902d-df3b-4e54-8b87-991bec17b23f" alt="6502 processor in a DIP-40 plastic package" width="400"/>
<br></br>
<img src="https://github.com/user-attachments/assets/c494bfde-e476-4073-b2b9-84872a9f4da9" alt="6502 processor die with drawn-in NMOS transistors and labels hinting at the functionality of the 6502's components" width="600"/>

## Instruction set
The following is the complete set of instructions supported by the 6502 and whether they
have been implemented in this project yet.

### Load/Store operations
- [x] [LDA - Load Accumulator](src/ins/load_store/lda.rs)
- [x] [LDX - Load X Register](src/ins/load_store/ldx.rs)
- [x] [LDY - Load Y Register](src/ins/load_store/ldy.rs)
- [x] [STA - Store Accumulator](src/ins/load_store/sta.rs)
- [x] [STX - Store X Register](src/ins/load_store/stx.rs)
- [x] [STY - Store Y Register](src/ins/load_store/sty.rs)

### Register Transfers
- [x] [TAX - Transfer accumulator to X](src/ins/reg_transfers/tax.rs)
- [x] [TAY - Transfer accumulator to Y](src/ins/reg_transfers/tay.rs)
- [x] [TXA - Transfer X to accumulator](src/ins/reg_transfers/txa.rs)
- [x] [TYA - Transfer Y to accumulator](src/ins/reg_transfers/tya.rs)

### Stack Operations
- [x] [TSX - Transfer stack pointer to X](src/ins/stack_ops/tsx.rs)
- [x] [TXS - Transfer X to stack pointer](src/ins/stack_ops/txs.rs)
- [x] [PHA - Push accumulator on stack](src/ins/stack_ops/pha.rs)
- [x] [PHP - Push processor status on stack](src/ins/stack_ops/php.rs)
- [x] [PLA - Pull accumulator from stack](src/ins/stack_ops/pla.rs)
- [x] [PLP - Pull processor status from stack](src/ins/stack_ops/plp.rs)

### Logical
- [x] [AND - Logical AND](src/ins/logical/and.rs)
- [x] [EOR - Exclusive OR](src/ins/logical/eor.rs)
- [x] [ORA - Logical Inclusive OR](src/ins/logical/ora.rs)
- [x] [BIT - Bit Test](src/ins/logical/bit.rs)

### Arithmetic
- [ ] ADC - Add with Carry
- [ ] SBC - Subtract with Carry
- [ ] CMP - Compare accumulator
- [ ] CPX - Compare X register
- [ ] CPY - Compare Y register

### Increments & Decrements
- [x] [INC - Increment a memory location](src/ins/inc_dec/inc.rs)
- [x] [INX - Increment the X register](src/ins/inc_dec/inx.rs)
- [x] [INY - Increment the Y register](src/ins/inc_dec/iny.rs)
- [x] [DEC - Decrement a memory location](src/ins/inc_dec/dec.rs)
- [x] [DEX - Decrement the X register](src/ins/inc_dec/dex.rs)
- [x] [DEY - Decrement the Y register](src/ins/inc_dec/dey.rs)

### Shifts
- [ ] ASL - Arithmetic Shift Left
- [ ] LSR - Logical Shift Right
- [ ] ROL - Rotate Left
- [ ] ROR - Rotate Right

### Jumps & Calls
- [ ] JMP - Jump to another location
- [x] [JSR - Jump to a subroutine](src/ins/jumps_calls/jsr.rs)
- [ ] RTS - Return from subroutine

### Branches
- [ ] BCC - Branch if carry flag clear
- [ ] BCS - Branch if carry flag set
- [ ] BEQ - Branch if zero flag set
- [ ] BMI - Branch if negative flag set
- [ ] BNE - Branch if zero flag clear
- [ ] BPL - Branch if negative flag clear
- [ ] BVC - Branch if overflow flag clear
- [ ] BVS - Branch if overflow flag set

### Status Flag Changes
- [x] [CLC - Clear carry flag](src/ins/status_flags/clc.rs)
- [x] [CLD - Clear decimal mode flag](src/ins/status_flags/cld.rs)
- [x] [CLI - Clear interrupt disable flag](src/ins/status_flags/cli.rs)
- [x] [CLV - Clear overflow flag](src/ins/status_flags/clv.rs)
- [x] [SEC - Set carry flag](src/ins/status_flags/sec.rs)
- [x] [SED - Set decimal mode flag](src/ins/status_flags/sed.rs)
- [x] [SEI - Set interrupt disable flag](src/ins/status_flags/sei.rs)

### System Functions
- [x] [BRK - Force an interrupt](src/ins/sys_funcs/brk.rs)
- [ ] NOP - No Operation
- [ ] RTI - Return from Interrupt

## Contributing
This project is a great opportunity for intermediate to advanced Rust developers, as well as more experienced developers coming from a C/C++ background who are interested in learning Rust. It is not only a fun challenge, but will also help you understand the low-level logic that drives everyday devices at a foundational level.

To get started, it would be helpful to compare the existing implementations in this codebase with the instruction set reference. Understanding these implementation patterns will make it easier to learn and write the remaining parts of the instruction set. Another way of helping is by writing more unit tests, with the aim of having at least one test per opcode.

If you're interested in making a contribution, please **feel free to open a PR :)** !

## References
- [The 6502 Microprocessor Resource](http://www.6502.org/users/obelisk/6502/)
- [Programming the NES: The 6502 in detail](https://www.middle-engine.com/blog/posts/2020/06/23/programming-the-nes-the-6502-in-detail)
- [Emulating a CPU in C++ (6502)](https://youtu.be/qJgsuQoy9bc)
