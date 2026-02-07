use std::collections::HashMap;

use super::*;

// test if TokenType::<Type>.extract() returns the correct data.
#[test]
fn tokentype_extract() {
    let data = b"a".to_vec();
    let obj = [
        TokenType::String(data.clone()).extract(),
        TokenType::Integer(data.clone()).extract(),
        TokenType::Float(data.clone()).extract(),
        TokenType::Identifier(data.clone()).extract(),
    ];

    for parsed in obj {
        assert_eq!(
            "a", parsed,
            "expected 'TokenType::<Type>.extract()' to return String 'a'. got {}",
            parsed
        );
    }
}

// test keyword detection by TokenType::keyword_or_identifier()
#[test]
fn tokentype_keyword_or_identifier() {
    let map: HashMap<TokenType, &[u8]> = HashMap::from([
        (TokenType::If, b"if" as &[u8]),
        (TokenType::Else, b"else"),
        (TokenType::For, b"for"),
        (TokenType::While, b"while"),
        (TokenType::Return, b"return"),
        (TokenType::Let, b"let"),
        (TokenType::Const, b"const"),
        (TokenType::Function, b"function"),
        (TokenType::True, b"true"),
        (TokenType::False, b"false"),
        (TokenType::Null, b"null"),
    ]);

    for (key, val) in map {
        let result = TokenType::keyword_or_identifier(val);
        let word = unsafe {std::str::from_utf8_unchecked(val)}.to_string();
        assert_eq!(
            key,
            result,
            "Expected b\"{}\" to be TokenType::{:?}. got TokenType::{:?}",
            word,
            key,
            result
        );
    }
}

