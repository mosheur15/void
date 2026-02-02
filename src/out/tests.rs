use super::*;

#[test]
fn test_colorize_functions() {
    let colors = [
        // Standard Colors (30-37)
        (Colorize::black("text"), 30),
        (Colorize::red("text"), 31),
        (Colorize::green("text"), 32),
        (Colorize::yellow("text"), 33),
        (Colorize::blue("text"), 34),
        (Colorize::magenta("text"), 35),
        (Colorize::cyan("text"), 36),
        (Colorize::white("text"), 37),
        // Bright Colors (90-97)
        (Colorize::black("text"), 30),
        (Colorize::red("text"), 31),
        (Colorize::green("text"), 32),
        (Colorize::yellow("text"), 33),
        (Colorize::blue("text"), 34),
        (Colorize::magenta("text"), 35),
        (Colorize::cyan("text"), 36),
        (Colorize::white("text"), 37),
    ];

    for (result, code) in colors {
        let expected = format!("\x1b[{}mtext\x1b[0m", code);
        assert_eq!(result, expected);
    }
}
