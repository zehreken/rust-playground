extern crate rand;
extern crate sdl2;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
mod grid;
pub use crate::grid::grid_config;

const moore_directions: [Point; 8] = [
    Point { x: -1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: -1 },
    Point { x: 0, y: -1 },
];

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    position: Point,
    neighbours: [Point; 8],
    current_state: i32,
    future_state: i32,
}

fn calculate_neighbours(x: i32, y: i32) -> [Point; 8] {
    let neighbours: [Point; 8] = [
        Point {
            x: moore_directions[0].x + x,
            y: moore_directions[0].y + y,
        },
        Point {
            x: moore_directions[0].x + x,
            y: moore_directions[0].y + y,
        },
        Point {
            x: moore_directions[0].x + x,
            y: moore_directions[0].y + y,
        },
        Point {
            x: moore_directions[0].x + x,
            y: moore_directions[0].y + y,
        },
        Point {
            x: moore_directions[0].x + x,
            y: moore_directions[0].y + y,
        },
        Point {
            x: moore_directions[0].x + x,
            y: moore_directions[0].y + y,
        },
        Point {
            x: moore_directions[0].x + x,
            y: moore_directions[0].y + y,
        },
        Point {
            x: moore_directions[0].x + x,
            y: moore_directions[0].y + y,
        },
    ];

    return neighbours;
}

fn get_live_neighbour_count(
    cell: Cell,
    grid: [[Cell; grid_config::COLUMN_COUNT as usize]; grid_config::ROW_COUNT as usize],
) -> i32 {
    let mut neighbour_count: i32 = 0;
    for i in 0..8 {
        println!("cell: {}", cell.position);
        if cell.neighbours[i].x >= 0
            && cell.neighbours[i].x < grid_config::ROW_COUNT
            && cell.neighbours[i].y >= 0
            && cell.neighbours[i].y < grid_config::COLUMN_COUNT
        {
            let current_state =
                grid[cell.neighbours[i].x as usize][cell.neighbours[i].y as usize].current_state;
            neighbour_count +=
                grid[cell.neighbours[i].x as usize][cell.neighbours[i].y as usize].current_state;
        }
    }

    println!("neighbour count: {}", neighbour_count);
    return neighbour_count;
}

fn cell_tick(
    mut cell: Cell,
    grid: [[Cell; grid_config::COLUMN_COUNT as usize]; grid_config::ROW_COUNT as usize],
) -> Cell {
    println!("before: {}", cell.current_state);
    if cell.current_state == 1 {
        let live_neighbour_count: i32 = get_live_neighbour_count(cell, grid);
        if live_neighbour_count < 2 {
            cell.future_state = 0;
        } else if live_neighbour_count == 2 || live_neighbour_count == 3 {
            cell.future_state = 1;
        } else {
            cell.future_state = 0;
        }
        cell.current_state = 0;
    } else {
        let live_neighbour_count: i32 = get_live_neighbour_count(cell, grid);
        if live_neighbour_count == 3 {
            cell.future_state = 1;
        } else {
            cell.future_state = 0;
        }
        cell.current_state = 1;
    }
    println!("after: {}", cell.current_state);

    return cell;
}

fn cell_swap(mut cell: Cell) {
    cell.current_state = cell.future_state;
}

fn create_cell() -> Cell {
    let cell = Cell {
        position: Point { x: 0, y: 0 },
        neighbours: calculate_neighbours(0, 0),
        current_state: 0,
        future_state: 0,
    };

    return cell;
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut grid: [[Cell; grid_config::COLUMN_COUNT as usize]; grid_config::ROW_COUNT as usize] =
        [[create_cell(); grid_config::COLUMN_COUNT as usize]; grid_config::ROW_COUNT as usize];

    for row in 0..grid_config::ROW_COUNT {
        for column in 0..grid_config::COLUMN_COUNT {
            grid[row as usize][column as usize].position = Point { x: column, y: row };
            grid[row as usize][column as usize].neighbours = calculate_neighbours(column, row);
            grid[row as usize][column as usize].current_state =
                if rng.gen_range(0, 4) < 1 { 0 } else { 1 };
        }
    }

    println!(
        "{}, {}, current_state: {}",
        grid[0][0].position.x, grid[0][0].position.y, grid[0][0].current_state
    );
    grid[0][0] = cell_tick(grid[0][0], grid);
    println!("current_state: {}", grid[0][0].current_state);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Game",
            grid_config::SCREEN_WIDTH as u32,
            grid_config::SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // Render here
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for row in 0..grid_config::ROW_COUNT {
            for column in 0..grid_config::COLUMN_COUNT {
                //if rng.gen_range(0, 2) == 0
                if grid[row as usize][column as usize].current_state == 1 {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    let cell_size: i32 = grid_config::CELL_SIZE;
                    canvas.fill_rect(Rect::new(
                        column * cell_size,
                        row * cell_size,
                        cell_size as u32,
                        cell_size as u32,
                    ));
                }
                //cell_tick(grid[row as usize][column as usize]);
            }
        }

        for row in 0..grid_config::ROW_COUNT {
            for column in 0..grid_config::COLUMN_COUNT {
                //cell_swap(grid[row as usize][column as usize]);
            }
        }

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
