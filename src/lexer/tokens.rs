#![allow(unused)]

use std::fmt::Display;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TokenType {
    // Structural / Single Characters
    //
    // Paren   ->   ()
    // Brace   ->   {}
    // Bracket ->   []
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    Equal,
    Not,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals (The ones that carry data)
    Identifier(Vec<u8>),
    String(Vec<u8>),
    Integer(Vec<u8>),
    Float(Vec<u8>),

    // Keywords
    If,
    Else,
    For,
    While,
    Return,
    Let,
    Const,
    Function,
    True,
    False,
    Null,

    // Special
    EOF,
}

// idk what it does, but it lets you call .to_string() to get the enum name as a string.
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl TokenType {
    /// Extract the data inside enums. returns emty string if theres no data.
    pub fn extract(&self) -> String {
        match self {
            TokenType::Identifier(data)
            | TokenType::String(data)
            | TokenType::Integer(data)
            | TokenType::Float(data) => {
                let parsed = unsafe { std::str::from_utf8_unchecked(data) }.to_string();
                return parsed;
            }
            _ => "".to_string(),
        }
    }

    /// returns keyword type if a u8 slice is a knwon keyword otherwise returns identifier.
    pub fn keyword_or_identifier(slice: &[u8]) -> TokenType {
        match slice {
            b"if" => TokenType::If,
            b"else" => TokenType::Else,
            b"for" => TokenType::For,
            b"while" => TokenType::While,
            b"return" => TokenType::Return,
            b"let" => TokenType::Let,
            b"const" => TokenType::Const,
            b"function" => TokenType::Function,
            b"true" => TokenType::True,
            b"false" => TokenType::False,
            b"null" => TokenType::Null,
            _ => TokenType::Identifier(slice.to_vec())
        }
    }
}


// unit test.
#[cfg(test)]
#[path = "tokens_tests.rs"]
mod tokens_tests;
