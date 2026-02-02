
//! # Output Utilities
//! 
//! This module provides tools for managing terminal output, including
//! string colorization, help text formatting, and error messaging.

/// A utility for wrapping strings in ANSI escape codes.
pub struct Colorize;

macro_rules! make_color {
    ($name:ident, $code:expr) => {
        #[doc = concat!("Returns the text wrapped with ANSI color code ", stringify!($code), ".")]
        /// 
        /// # Example
        /// ```
        #[doc = concat!("use crate::out::Colorize;")]
        #[doc = concat!("let text = Colorize::", stringify!($name), "(\"Hello\");")]
        #[doc = concat!("assert_eq!(text, \"\\x1b[", stringify!($code), "mHello\\x1b[0m\");")]
        /// ```
        pub fn $name(text: &str) -> String {
            format!("\x1b[{}m{}\x1b[0m", $code, text)
        }
    };
}

impl Colorize {
    make_color!(black, 30);
    make_color!(red, 31);
    make_color!(green, 32);
    make_color!(yellow, 33);
    make_color!(blue, 34);
    make_color!(magenta, 35);
    make_color!(cyan, 36);
    make_color!(white, 37);
    make_color!(bright_black, 90);
    make_color!(bright_red, 91);
    make_color!(bright_green, 92);
    make_color!(bright_yellow, 93);
    make_color!(bright_blue, 94);
    make_color!(bright_magenta, 95);
    make_color!(bright_cyan, 96);
    make_color!(bright_white, 97);
}

/// Prints a formatted error message to stderr.
pub fn print_error(msg: &str) {
    eprintln!("{} {}", Colorize::red("error:"), msg);
}


// unit tests (the test file is in ./tests.rs)
#[cfg(test)]
mod tests;

