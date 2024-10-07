use thiserror::Error;
use miette::{Diagnostic, SourceSpan, NamedSource};

use crate::{Byte, Word};

/// A lexer for tokenising raw assembly code. Our lexer should be able to: maintain a
/// reference to the input source, keep track of the progress, look ahead `n` places when
/// necessary, and raise errors when and where appropriate.
pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn tokenise(&mut self) -> Result<Vec<Token>, LexingError> {
        let mut tokens = vec![];

        loop {
            match self.next() {
                Some(t) => {
                    tokens.push(t);

                    if let Token::EOF = t {
                        break;
                    }
                },
                None => { return Err(LexingError {
                    src: NamedSource::new("bad_file.rs", self.src.to_owned()),
                    bad_bit: self.get_line_col().into(),
                });}
            }
        }

        Ok(tokens)
    }

    fn get_line_col(&self) -> (usize, usize) {
        let cur_str = &self.src[..self.pos];
        let lines: Vec<&str> = cur_str.split('\n').collect();
        if lines.len() == 0 {
            return (0, 0)
        }
        let col = lines.last().unwrap().len();
        (lines.len(), col)
    }

    fn cur_char(&self) -> Option<char> {
        self.src.chars().nth(self.pos)
    }

}

/// We want to implement our lexer as a stream of tokens, so we implement the iterator
/// trait for our lexer.
impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur_char() {
            // If we find a whitespace, skip
            Some(' ') => {
                self.pos += 1;
                self.next()
            },
            Some('x' | 'X') => {
                self.pos += 1;
                Some(Token::XReg)
            },
            Some('y' | 'Y') => {
                self.pos += 1;
                Some(Token::YReg)
            },
            // An alphabetic character (other than X and Y) can only be an instruction identifier
            Some('a'..='z' | 'A'..='Z') => {
                let t = Token::Instruction(&self.src[self.pos..self.pos + 3]);
                self.pos += 3;
                Some(t)
            },
            Some('#') => {
                self.pos += 1;
                Some(Token::ImmediateSpecifier)
            },
            // Tokenise hex values
            Some('$') => {
                self.pos += 1; // Skip the '$'
                let start = self.pos;
                while let Some(c) = self.cur_char() {
                    if c.is_digit(16) {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                let hex_str = &self.src[start..self.pos];

                if hex_str.len() == 2 {
                    let val = u8::from_str_radix(hex_str, 16).ok()?;
                    Some(Token::ByteValue(val))
                } else if hex_str.len() == 4 {
                    let val = u16::from_str_radix(hex_str, 16).ok()?;
                    Some(Token::WordValue(val))
                } else {
                    None
                }
            },
            Some('%') => {
                self.pos += 1;
                let start = self.pos;
                while let Some(c) = self.cur_char() {
                    if c.is_digit(2) {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                let bin_str = &self.src[start..self.pos];

                if bin_str.len() == 8 {
                    let val = u8::from_str_radix(bin_str, 2).ok()?;
                    Some(Token::ByteValue(val))
                } else if bin_str.len() == 16 {
                    let val = u16::from_str_radix(bin_str, 2).ok()?;
                    Some(Token::WordValue(val))
                } else {
                    None
                }
            },
            Some('(') => {
                self.pos += 1;
                Some(Token::LeftBracket)
            },
            Some(')') => {
                self.pos += 1;
                Some(Token::RightBracket)
            },
            Some(',') => {
                self.pos += 1;
                Some(Token::Comma)
            },
            Some(';') => {
                self.pos += 1;
                Some(Token::CommentDelimiter)
            },
            Some('\n') => {
                self.pos += 1;
                Some(Token::NewLine)
            },
            _ => None
        }
    }
}

#[derive(Clone, Copy)]
pub enum Token<'a> {
    Instruction(&'a str),
    Comma,
    XReg,
    YReg,
    LeftBracket,
    RightBracket,
    ImmediateSpecifier,
    ByteValue(Byte),
    WordValue(Word),
    CommentDelimiter,
    Value(&'a str),
    NewLine,
    EOF
}

#[derive(Error, Debug, Diagnostic)]
#[error("Error while attempting to tokenise the provided source.")]
pub struct LexingError {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource<String>,
    // Snippets and highlights can be included in the diagnostic!
    #[label("This bit here")]
    bad_bit: SourceSpan,
}
