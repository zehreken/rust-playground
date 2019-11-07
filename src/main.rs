extern crate rand;
extern crate sdl2;

mod automata;
mod fps_utils;
pub use crate::automata::*;
pub use crate::fps_utils::fps_utils::*;

mod fast_type;
pub use crate::fast_type::*;

fn main() {
    // start_automata();
    start_fast_type();
}
