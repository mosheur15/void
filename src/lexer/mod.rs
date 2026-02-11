mod tokens;
use std::fs;

use crate::lexer::tokens::{TokenType, Token};

#[derive(Debug)]
enum Mode {
    Normal,
    String,
    Integer,
    Float,
    Identifier
}

/// A structure representing a file buffer with tracking and tokemization capabilities.
#[derive(Debug)]
pub struct File {
    pub filename: String,
    pub raw: Vec<u8>,
    pub cursor: usize,
    pub line_mapping: Vec<usize>,
    pub tokens: Vec<Token>,
}

impl File {
    /// Creates a new file buffer.
    ///
    /// # Arguments
    /// * `filename` - A `String` containing the name/path of the file.
    pub fn new(filename: String) -> Self {
        let mut file = Self {
            filename: filename,
            raw: vec![],
            cursor: 0,
            line_mapping: vec![],
            tokens: vec![],
        };
        file.read();
        return file;
    }

    /// Reads a file and load the bytes to `File.raw`
    pub fn read(&mut self) {
        let mut empty_err_msg = "".to_string();
        let read_result = fs::read(&self.filename).unwrap();
        self.raw = read_result;
    }

    /// Figures out the line number by a byte position.
    /// 
    /// # Returns
    /// * `.0` - A `usize` as the line number.
    /// * `.1` - A `usize` as the reletive position starting from the line number.
    pub fn get_line_number(&self, byte_position: &usize) -> (usize, usize) {
        return (
            10 as usize,
            20 as usize,
        );
    }


    /// Tokenize the content stored in `File.raw` and save it in `File.tokens`
    pub fn tokenize(&mut self){
        // state machine mode.
        let mut mode = Mode::Normal;

        // holds the starting values for literals.
        let mut str_start = 0;
        let mut int_start = 0;
        let mut flt_start = 0;
        let mut idn_start = 0;

        // state machine.
        while self.cursor < self.raw.len() {
            match mode {
                // Normal mode: handle all the tokens except literals and keywords.
                Mode::Normal => {
                    match self.raw[self.cursor] {
                        // IMPORTANT: do not consume the newline from any other mode. so the line 
                        // number is not broken. also it cab be a nightmare to debug if you do.
                        b'\n' => {
                            self.line_mapping.push(self.cursor);
                            self.cursor += 1;
                        }

                        // single character tokens.
                        b'(' => {
                            self.tokens.push(Token{
                                name: TokenType::LeftParen,
                                position: self.cursor
                            });
                            self.cursor += 1;
                        }

                        // illegal character.
                        _=> {
                            println!("State machine got broken with invalid character");
                            std::process::exit(2);
                        }
                    }
                },
                _ => {
                    println!("State machine got broken with state {:?}", mode);
                    std::process::exit(2);
                }
            }
        }
    }
}

#[cfg(test)]
mod mod_tests; 
