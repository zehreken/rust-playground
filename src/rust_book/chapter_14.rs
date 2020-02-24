//! Rust Playground
//! 
//! 'rust_playground' is collection of tests and examples
//! on my to learn rust

pub fn run() {}

/// Adds one to the number given.assert_eq!
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = rust_playground::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}