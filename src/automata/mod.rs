mod cell;
mod grid;

use crate::automata::cell::*;
use crate::automata::grid::*;
use crate::fps_utils::*;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::path::Path;
use std::time::{Duration, Instant};

fn get_live_neighbour_count(cell: Cell, grid: &Vec<Vec<Cell>>) -> i32 {
    let mut neighbour_count: i32 = 0;
    for i in 0..8 {
        if cell.neighbours[i].x >= 0
            && cell.neighbours[i].x < COLUMN_COUNT
            && cell.neighbours[i].y >= 0
            && cell.neighbours[i].y < ROW_COUNT
        {
            let current_state =
                grid[cell.neighbours[i].y as usize][cell.neighbours[i].x as usize].current_state;
            neighbour_count += current_state;
        }
    }

    neighbour_count
}

fn cell_tick(cell: &mut Cell, live_neighbour_count: i32) {
    if cell.current_state == 1 {
        if live_neighbour_count < 2 {
            cell.future_state = 0;
            cell.on_count = 0;
        } else if live_neighbour_count == 2 || live_neighbour_count == 3 {
            cell.future_state = 1;
            cell.on_count += 1;
        } else {
            cell.future_state = 0;
            cell.on_count = 0;
        }
    } else {
        if live_neighbour_count == 3 {
            cell.future_state = 1;
        } else {
            cell.future_state = 0;
        }
    }
}

fn cell_swap(cell: &mut Cell) {
    cell.current_state = cell.future_state;
}

pub fn run() {
    let mut rng = rand::thread_rng();

    let mut grid: Vec<Vec<Cell>> = Vec::new();

    for row in 0..ROW_COUNT {
        grid.push(Vec::new());
        for column in 0..COLUMN_COUNT {
            grid[row as usize].push(create_cell());
            grid[row as usize][column as usize].position = Point { x: column, y: row };
            grid[row as usize][column as usize].neighbours = calculate_neighbours(column, row);
            grid[row as usize][column as usize].current_state =
                if rng.gen_range(0..2) < 1 { 0 } else { 1 };
        }
    }

    println!("{}", grid[0][0]);

    let mut fps_counter = FpsCounter::new();

    /*
    grid[0][0].current_state = 1;
    grid[1][1].current_state = 1;
    grid[2][2].current_state = 1;
    grid[3][3].current_state = 1;
    grid[4][4].current_state = 1;
    grid[1][3].current_state = 1;
    println!(
        "{}, {}, current_state: {}",
        grid[0][0].position.x, grid[0][0].position.y, grid[0][0].current_state
    );
    grid[0][0] = cell_tick(grid[0][0], grid);
    println!("current_state: {}", grid[0][0].current_state);
    */

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("automata", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("fonts/emulogic.ttf");
    let font = ttf_context.load_font(font_path, 8).unwrap();
    //font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut surface: Surface = font
        .render("FPS: 000")
        .solid(Color::RGBA(255, 0, 255, 255))
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let text_query = texture.query();
    let text_rect = Rect::new(0, 0, text_query.width, text_query.height);
    //    println!("{}, {}", text_query.width, text_query.height);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut now = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        let duration: Duration = Instant::now() - now;
        let fps_count = fps_counter.tick(duration.as_millis());
        if fps_count > 0 {
            surface = font
                .render(&format!("FPS: {}", fps_count))
                .solid(Color::RGBA(255, 0, 255, 255))
                .unwrap();
            texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
        }
        now = Instant::now();

        // Render here
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for row in 0..ROW_COUNT {
            for column in 0..COLUMN_COUNT {
                let cell: Cell = grid[row as usize][column as usize];
                if cell.current_state == 1 {
                    let red: u8 = if 20 * cell.on_count > 255 {
                        255
                    } else {
                        20 * cell.on_count as u8
                    };
                    canvas.set_draw_color(Color::RGB(red, 0, 0));
                    canvas
                        .fill_rect(Rect::new(
                            column * CELL_SIZE,
                            row * CELL_SIZE,
                            CELL_SIZE as u32,
                            CELL_SIZE as u32,
                        ))
                        .unwrap();
                }

                let live_neighbour_count: i32 =
                    get_live_neighbour_count(grid[row as usize][column as usize], &grid);
                cell_tick(
                    &mut grid[row as usize][column as usize],
                    live_neighbour_count,
                );
            }
        }

        for row in 0..ROW_COUNT {
            for column in 0..COLUMN_COUNT {
                cell_swap(&mut grid[row as usize][column as usize]);
            }
        }

        canvas.copy(&texture, None, text_rect).unwrap();

        canvas.present();

        std::thread::sleep(Duration::from_millis(20));
    }
}
