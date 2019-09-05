extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;
use rand::Rng;
mod grid;
pub use crate::grid::grid_config;

fn main()
{
	let mut rng = rand::thread_rng();

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
				if rng.gen_range(0, 2) == 0
				{
					canvas.set_draw_color(Color::RGB(0, 0, 0));
					let cell_size:i32 = grid_config::CELL_SIZE;
					canvas.fill_rect(Rect::new(column * cell_size, row * cell_size, cell_size as u32, cell_size as u32));
				}
			}
		}

		canvas.present();

		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
