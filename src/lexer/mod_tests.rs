use crate::lexer::tokens::TokenType;
use super::*;

const FILE_PATH: &str = "./src/lexer/DO_NOT_TOUCH.test.txt";

// test if the File::new() throws an error for non existing file path.
#[test]
fn file_new() {
    let result = std::panic::catch_unwind(|| {
        let a = File::new("./there_is_no_file_jdjkwlaozh8291".to_string());
        println!("{:#?}", a);
    });
    assert!(result.is_err(), "Expected panic for non existing file path, got no panic.");
}


// test if the tokenizer gets the right line number and position.
#[test]
fn file_get_line_number() {
    let mut file = File::new(FILE_PATH.to_string());
    file.read();

    // Case 1: Start of file (Index 0)
    // Expected: Line 1, Column 0
    let (line, col) = file.get_line_number(&0);
    assert_eq!(line, 1, "Start of file: expected line 1, got {}", line);
    assert_eq!(col, 0, "Start of file: expected relative position 0, got {}", col);

    // Case 2: End of the first line (just before \n)
    // Expected: Line 1, Column 12
    let (line, col) = file.get_line_number(&12);
    assert_eq!(line, 1, "Before newline (pos 12): expected line 1, got {}", line);
    assert_eq!(col, 12, "Before newline (pos 12): expected relative position 12, got {}", col);

    // Case 3: The newline character itself (\n)
    // Expected: Still technically on Line 1 (standard lexer behavior)
    let (line, col) = file.get_line_number(&13);
    assert_eq!(line, 1, "At newline (pos 13): expected line 1, got {}", line);
    assert_eq!(col, 13, "At newline (pos 13): expected relative position 13, got {}", col);

    // Case 4: First character of the second line
    // Expected: Line 2, Column 0
    let (line, col) = file.get_line_number(&14);
    assert_eq!(line, 2, "After newline (pos 14): expected line 2, got {}", line);
    assert_eq!(col, 0, "After newline (pos 14): expected relative position 0, got {}", col);
}


// test if the tokenizer does its job correctly.
#[test]
fn file_tokenize() {
    // case 1: check tokenization for symbols.
    let mut map = [
        (TokenType::LeftParen, b"("),
        (TokenType::RightParen, b")"),
        (TokenType::LeftBrace, b"{"),
        (TokenType::RightBrace, b"}"),
        (TokenType::LeftBracket, b"["),
        (TokenType::RightBracket, b"]"),
        (TokenType::Comma, b","),
        (TokenType::Dot, b"."),
        (TokenType::Semicolon, b";"),
        (TokenType::Colon, b":"),
    ];

    for (expected, data) in map {
        let mut file = File::new(FILE_PATH.to_string());
        file.raw = data.to_vec();
        file.tokenize();
        assert_eq!(
            vec![expected.clone(), TokenType::EOF],
            file.tokens,
            "Expected `File.tokenize()` to return {:?}. got {:?}",
            vec![expected.clone(), TokenType::EOF],
            file.tokens
        );
    }

    // case 2: check tokenization for Operators.
    let map2 = [
        (TokenType::Plus, b"+" as &[u8]),
        (TokenType::Minus, b"-"),
        (TokenType::Star, b"*"),
        (TokenType::Slash, b"/"),
        (TokenType::Percent, b"%"),
        (TokenType::Assign, b"="),
        (TokenType::Equal, b"=="),
        (TokenType::Not, b"!"),
        (TokenType::NotEqual, b"!="),
        (TokenType::Greater, b">"),
        (TokenType::GreaterEqual, b">="),
        (TokenType::Less, b"<"),
        (TokenType::LessEqual, b"<="),
    ];

    for (expected, data) in map2 {
        let mut file = File::new(FILE_PATH.to_string());
        file.raw = data.to_vec();
        file.tokenize();
        assert_eq!(
            vec![expected.clone(), TokenType::EOF],
            file.tokens,
            "Expected `File.tokenize()` to return {:?}. got {:?}",
            vec![expected.clone(), TokenType::EOF],
            file.tokens
        );
    }


    // case 3: check tokenization for literals with data.
    let map3 = [
        (TokenType::Identifier(b"cat_name".to_vec()), b"cat_name" as &[u8]),
        (TokenType::String(b"\"hello world\"".to_vec()), b"\"hello world\""),
        (TokenType::Integer(b"1234".to_vec()), b"1234"),
        (TokenType::Float(b"12.34".to_vec()), b"12.34"),
    ];

    for (expected, data) in map3 {
        let mut file = File::new(FILE_PATH.to_string());
        file.raw = data.to_vec();
        file.tokenize();
        assert_eq!(
            vec![expected.clone(), TokenType::EOF],
            file.tokens,
            "Expected `File.tokenize()` to return {:?}. got {:?}",
            vec![expected.clone(), TokenType::EOF],
            file.tokens
        );
    }


    // case 4: check tokenization for keywords.
    let map4 = [
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
    ];

    for (expected, data) in map4 {
        let mut file = File::new(FILE_PATH.to_string());
        file.raw = data.to_vec();
        file.tokenize();
        assert_eq!(
            vec![expected.clone(), TokenType::EOF],
            file.tokens,
            "Expected `File.tokenize()` to return {:?}. got {:?}",
            vec![expected.clone(), TokenType::EOF],
            file.tokens
        );
    }
}
