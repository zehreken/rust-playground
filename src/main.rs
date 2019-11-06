extern crate rand;
extern crate sdl2;

mod automata;
mod fps_utils;
pub use crate::automata::*;
pub use crate::fps_utils::fps_utils::*;

fn main() {
    automata_start();
}
