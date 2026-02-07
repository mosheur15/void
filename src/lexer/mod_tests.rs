use super::*;

const FILE_PATH: &str = "./src/lexer/DO_NOT_TOUCH.test.txt";

#[test]
fn file_new() {
    let result = std::panic::catch_unwind(|| {
        let a = File::new("./there_is_no_file_jdjkwlaozh8291".to_string());
        println!("{:#?}", a);
    });
    assert!(result.is_err(), "Expected panic for non existing file path, got no panic.");
}



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



