use std::collections::HashMap;

use super::*;

// test if TokenType::<Type>.extract() returns the correct data.
#[test]
fn tokentype_extract() {
    let data = b"a".to_vec();
    let map: HashMap<&str, String> = HashMap::from([
        ("TokenType::String", TokenType::String(data.clone()).extract()),
        ("TokenType::Integer", TokenType::Integer(data.clone()).extract()),
        ("TokenType::Float", TokenType::Float(data.clone()).extract()),
        ("TokenType::Identifier", TokenType::Identifier(data.clone()).extract()),
    ]);

    for (key, val) in map {
        assert_eq!(
            "a".to_string(), val,
            "Expected {}(b\"a\".to_vec()).extract() to return String \"a\". got \"{}\"",
            key, val
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

    let ids = [b"cat" as &[u8], b"dog", b"sheep"];
    for id in ids {
        let result = TokenType::keyword_or_identifier(id);
        let word = unsafe {std::str::from_utf8_unchecked(id)}.to_string();
        assert_eq!(
            TokenType::Identifier(id.to_vec()),
            result,
            "Expected b\"{}\" to be TokenType::Identifier got TokenType::{}",
            word,
            result
        );
    }
}

