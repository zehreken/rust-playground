extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;
use rand::Rng;
mod grid;
pub use crate::grid::grid_config;

const moore_directions:[Point; 8] = [
	Point{ x:-1, y:-1 },
	Point{ x:-1, y:0 },
	Point{ x:-1, y:1 },
	Point{ x:0, y:1 },
	Point{ x:1, y:1 },
	Point{ x:1, y:0 },
	Point{ x:1, y:-1 },
	Point{ x:0, y:-1 },
];

#[derive(Copy, Clone)]
struct Point
{
	x:i32,
	y:i32,
}

#[derive(Copy, Clone)]
struct Cell
{
	position:Point,
	neighbours:[Point; 8],
	current_state:i32,
	future_state:i32,
}

fn calculate_neighbours(x:i32, y:i32) -> [Point; 8] {
	let neighbours:[Point; 8] = [
		Point{ x: moore_directions[0].x + x, y:moore_directions[0].y + y},
		Point{ x: moore_directions[0].x + x, y:moore_directions[0].y + y},
		Point{ x: moore_directions[0].x + x, y:moore_directions[0].y + y},
		Point{ x: moore_directions[0].x + x, y:moore_directions[0].y + y},
		Point{ x: moore_directions[0].x + x, y:moore_directions[0].y + y},
		Point{ x: moore_directions[0].x + x, y:moore_directions[0].y + y},
		Point{ x: moore_directions[0].x + x, y:moore_directions[0].y + y},
		Point{ x: moore_directions[0].x + x, y:moore_directions[0].y + y},
	];

	return neighbours;
}

fn get_live_neighbour_count() -> i32
{
	return 0;
}

fn cell_tick(mut cell:Cell)
{
	if cell.current_state == 1
	{
		let live_neighbour_count:i32 = get_live_neighbour_count();
		if live_neighbour_count < 2
		{
			cell.future_state = 0;
		}
		else if live_neighbour_count == 2 || live_neighbour_count == 3
		{
			cell.future_state = 1;
		}
		else
		{
			cell.future_state = 0;
		}
	}
	else
	{
		let live_neighbour_count = get_live_neighbour_count();
		if live_neighbour_count == 3
		{
			cell.future_state = 1;
		}
		else
		{
			cell.future_state = 0;
		}
	}
}

fn cell_swap(cell:Cell)
{

}

fn create_cell() -> Cell
{
	let cell = Cell {
		position: Point{ x: 0, y: 0 },
		neighbours: calculate_neighbours(0, 0),
		current_state: 0,
		future_state: 0,
	};

	return cell;
}

fn main()
{
	let mut rng = rand::thread_rng();
	
	let mut grid: [[Cell; grid_config::COLUMN_COUNT as usize]; grid_config::ROW_COUNT as usize] =
		[[create_cell(); grid_config::COLUMN_COUNT as usize]; grid_config::ROW_COUNT as usize];

	for row in 0..grid_config::ROW_COUNT
	{
		for column in 0..grid_config::COLUMN_COUNT
		{
			grid[row as usize][column as usize].position = Point{ x:column, y:row };
			grid[row as usize][column as usize].neighbours = calculate_neighbours(column, row);
			grid[row as usize][column as usize].current_state = if rng.gen_range(0, 4) < 1 { 0 } else { 1 };
		}
	}

	println!("{}, {}", grid[4][5].position.x, grid[4][5].position.y);

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem
		.window("Game", grid_config::SCREEN_WIDTH as u32, grid_config::SCREEN_HEIGHT as u32)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
	let mut i = 0;

	'running: loop
	{
		for event in event_pump.poll_iter()
		{
			match event
			{
				sdl2::event::Event::Quit { .. } => break 'running,
				_ => {}
			}
		}

		// Render here
		i = (i + 1) % 255;
		canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
		canvas.clear();

        for row in 0..grid_config::ROW_COUNT
		{               
			for column in 0..grid_config::COLUMN_COUNT
			{
				//if rng.gen_range(0, 2) == 0
				if grid[row as usize][column as usize].current_state == 1
				{
					canvas.set_draw_color(Color::RGB(0, 0, 0));
					let cell_size:i32 = grid_config::CELL_SIZE;
					canvas.fill_rect(Rect::new(column * cell_size, row * cell_size, cell_size as u32, cell_size as u32));
				}
				cell_tick(grid[row as usize][column as usize]);
			}
		}

		for row in 0..grid_config::ROW_COUNT
		{
			for column in 0..grid_config::COLUMN_COUNT
			{
				cell_swap(grid[row as usize][column as usize]);
			}
		}

		canvas.present();

		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
