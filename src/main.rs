extern crate rand;
extern crate sdl2;
use std::env;

mod automata;
pub use crate::automata::*;
mod fps_utils;
pub use crate::fps_utils::fps_utils::*;
mod framebuffer;
pub use crate::framebuffer::*;
mod fast_type;
pub use crate::fast_type::*;
mod ownership;
pub use crate::ownership::*;
mod concurrency;
pub use crate::concurrency::*;

const AUTOMATA: &str = "--automata";
const FAST_TYPE: &str = "--fast_type";
const FRAMEBUFFER: &str = "--framebuffer";
const OWNERSHIP: &str = "--ownership";
const CONCURRENCY: &str = "--concurrency";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "OPTIONS:\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
            AUTOMATA, FAST_TYPE, FRAMEBUFFER, OWNERSHIP, CONCURRENCY
        );
    } else if args.len() > 2 {
        println!("Too many arguments!");
    } else {
        let arg: &str = &args[1][..];
        match arg {
            AUTOMATA => start_automata(),
            FAST_TYPE => start_fast_type(),
            FRAMEBUFFER => start_framebuffer(),
            OWNERSHIP => start_ownership(),
            CONCURRENCY => start_concurrency(),
            _ => println!("Unknown arguement!"),
        }
    }
}
