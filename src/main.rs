extern crate rand;
extern crate sdl2;

mod automata;
mod cell;
mod fps_utils;
mod grid;
pub use crate::cell::cell::*;
pub use crate::fps_utils::fps_utils::*;
pub use crate::grid::grid_config::*;
pub use crate::automata::automata::*;

fn main() {
    automata_start();
}
