extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;
use rand::Rng;

fn main()
{
	let mut rng = rand::thread_rng();

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem
		.window("Game", 800, 600)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();

	let row_count = 60;
	let column_count = 80;

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

        for row in 0..row_count
		{               
			for column in 0..column_count
			{
				if rng.gen_range(0, 2) == 0
				{
					canvas.set_draw_color(Color::RGB(0, 0, 0));
					canvas.fill_rect(Rect::new(column * 10, row * 10, 10, 10));
				}
			}
		}

		canvas.present();

		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
