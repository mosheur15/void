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


fn main() {
    // Entry point for the Void compiler
    println!("Void: Systems power with scripting simplicity.");
}
