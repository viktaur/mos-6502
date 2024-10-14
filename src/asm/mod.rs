pub mod lexer;
pub mod parser;

#[macro_export]
macro_rules! asm {
    // Operations are divided by new lines
    // Comments start with ';'
    // '$' is used for hex and '%' for binary
    // Instruction mnemonics are not case sensitive ('LDA' and 'lda' are both valid)
    // Different syntax for each addressing method (all w/ operand $04):

        // ADC #$04     ; The opcode is $69.
        // ADC $04, X   ; The opcode is $61.
        // ADC ($04), Y ; The opcode is $71.

    // Labels: associate a name with a particular location in the program code
    // e.g.
    // some_label: LDA #$00

    () => {

    };
}
