//! # Void (.vd)
//!
//! **Void** is a systems programming language designed to be as simple as a
//! high-level scripting language while retaining the power of manual memory management.
//!
//! ## The Void Philosophy
//! - **Familiar Syntax:** No "weird" symbols. If you know C, Python, or JS, you know Void.
//! - **Batteries Included:** High-level built-in methods (strings, arrays, maps) come standard.
//! - **Direct Control:** Manual memory management when you need it, without the boilerplate.
//! - **Smart Defaults:** Type inference is the standard; explicit typing is the exception.
//!
//! ## Language Goals
//! Void aims to eliminate the "complexity tax" of systems programming. It provides the
//! performance of low-level languages with the ergonomics of high-level ones.
//!
//! ## Compilation Pipeline
//! 1. **Lexer** (Current): Raw source (.vd) → Token Stream.
//! 2. **Parser**: Token Stream → Abstract Syntax Tree (AST).
//! 3. **Type Checker**: Validates logic and infers missing types.
//! 4. **Backend**: Generates executable machine code.
//!
//! ## Quick Start
//! ```bash
//! void run main.vd
//! ```

#![allow(unused)]
pub mod out;
pub mod lexer;

use out::Colorize;

fn main() {

    let mut file = lexer::File::new("./test.vd".to_string());
    file.read();
    println!("{:#?}", file.raw);
    //
    // let colors: Vec<(&str, fn(&str) -> String)> = vec![
    //     ("Black", Colorize::black),
    //     ("Red", Colorize::red),
    //     ("Green", Colorize::green),
    //     ("Yellow", Colorize::yellow),
    //     ("Blue", Colorize::blue),
    //     ("Magenta", Colorize::magenta),
    //     ("Cyan", Colorize::cyan),
    //     ("White", Colorize::white),
    //     ("Bright Black", Colorize::bright_black),
    //     ("Bright Red", Colorize::bright_red),
    //     ("Bright Green", Colorize::bright_green),
    //     ("Bright Yellow", Colorize::bright_yellow),
    //     ("Bright Blue", Colorize::bright_blue),
    //     ("Bright Magenta", Colorize::bright_magenta),
    //     ("Bright Cyan", Colorize::bright_cyan),
    //     ("Bright White", Colorize::bright_white),
    // ];
    //
    // for (name, func) in colors {
    //     println!("{}: {}", name, func("Sample Text"));
    // }
    //
}
