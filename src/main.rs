#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_must_use)]
extern crate rand;
extern crate sdl2;
use std::cmp::Ordering;
use std::env;

mod automata;
mod concurrency;
mod fps_utils;
mod framebuffer;
mod memory;
mod opengl_test;
mod rust_book;
mod todo_list;
mod traits;

const AUTOMATA: &str = "automata";
const FRAMEBUFFER: &str = "framebuffer";
const CONCURRENCY: &str = "concurrency";
const OPENGL_TEST: &str = "opengltest";
const MEMORY: &str = "memory";
const RUST_BOOK: &str = "rustbook";
const TODO: &str = "todo";
const TRAITS: &str = "traits";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        Ordering::Less => {
            println!(
                "OPTIONS:\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
                AUTOMATA, FRAMEBUFFER, CONCURRENCY, OPENGL_TEST, MEMORY, RUST_BOOK, TODO, TRAITS
            );
        }
        _ => {
            let arg: &str = &args[1][..];
            match arg {
                AUTOMATA => automata::run(),
                FRAMEBUFFER => framebuffer::run(),
                CONCURRENCY => concurrency::run(),
                OPENGL_TEST => opengl_test::run(),
                MEMORY => memory::run(),
                RUST_BOOK => rust_book::run(),
                TODO => todo_list::run(),
                TRAITS => traits::run(),
                _ => println!("Unknown argument!"),
            }
        }
    }
}
