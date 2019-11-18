extern crate rand;
extern crate sdl2;

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

fn main() {
    // start_automata();
    // start_fast_type();
    // start_framebuffer();
    // start_ownership();
    start_concurrency();
}
