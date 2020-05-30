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
use automata::*;
mod fps_utils;
mod framebuffer;
use framebuffer::*;
mod concurrency;
use concurrency::*;
mod opengl_test;
use opengl_test::*;
mod memory;
use memory::*;
mod cpal_test;
use cpal_test::*;
mod rust_book;
use rust_book::*;

const AUTOMATA: &str = "automata";
const FRAMEBUFFER: &str = "framebuffer";
const CONCURRENCY: &str = "concurrency";
const OPENGL_TEST: &str = "opengltest";
const MEMORY: &str = "memory";
const CPAL_TEST: &str = "cpaltest";
const RUST_BOOK: &str = "rustbook";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        Ordering::Less => {
            println!(
                "OPTIONS:\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
                AUTOMATA, FRAMEBUFFER, CONCURRENCY, OPENGL_TEST, MEMORY, CPAL_TEST, RUST_BOOK,
            );
        }
        _ => {
            let arg: &str = &args[1][..];
            match arg {
                AUTOMATA => start_automata(),
                FRAMEBUFFER => start_framebuffer(),
                CONCURRENCY => start_concurrency(),
                OPENGL_TEST => start_opengl_test(),
                MEMORY => start_memory(),
                CPAL_TEST => start_cpal(),
                RUST_BOOK => start_rust_book(),
                _ => println!("Unknown argument!"),
            }
        }
    }
}
