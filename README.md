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
- [x] TAX - Transfer accumulator to X
- [x] TAY - Transfer accumulator to Y
- [x] TXA - Transfer X to accumulator
- [x] TYA - Transfer Y to accumulator

### Stack Operations
- [ ] TSX - Transfer stack pointer to X
- [ ] TXS - Transfer X to stack pointer
- [ ] PHA - Push accumulator on stack
- [ ] PHP - Push processor status on stack
- [ ] PLA - Pull accumulator from stack
- [ ] PLP - Pull processor status from stack

### Logical
- [ ] AND - Logical AND
- [ ] EOR - Exclusive OR
- [ ] ORA - Logical Inclusive OR
- [ ] BIT - Bit Test

### Arithmetic
- [ ] ADC - Add with Carry
- [ ] SBC - Subtract with Carry
- [ ] CMP - Compare accumulator
- [ ] CPX - Compare X register
- [ ] CPY - Compare Y register

### Increments & Decrements
- [ ] INC - Increment a memory location
- [ ] INX - Increment the X register
- [ ] INY - Increment the Y register
- [ ] DEC - Decrement a memory location
- [ ] DEX - Decrement the X register
- [ ] DEY - Decrement the Y register

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
- [ ] CLC - Clear carry flag
- [ ] CLD - Clear decimal mode flag
- [ ] CLI - Clear interrupt disable flag
- [ ] CLV - Clear overflow flag
- [ ] SEC - Set carry flag
- [ ] SED - Set decimal mode flag
- [ ] SEI - Set interrupt disable flag

### System Functions
- [x] [BRK - Force an interrupt](src/ins/sys_funcs/brk.rs)
- [ ] NOP - No Operation
- [ ] RTI - Return from Interrupt

## References
- [The 6502 Microprocessor Resource](http://www.6502.org/users/obelisk/6502/)
- [Programming the NES: The 6502 in detail](https://www.middle-engine.com/blog/posts/2020/06/23/programming-the-nes-the-6502-in-detail)
- [Emulating a CPU in C++ (6502)](https://youtu.be/qJgsuQoy9bc)
