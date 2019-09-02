extern crate sdl2;

fn main()
{
	let logical: bool = true;
	let a_float: f64 = 1.0;
	let mut an_integer = 5i32; // Suffix annotation

	println!("{}", logical);
	println!("{}", a_float);
	println!("{}", an_integer);

	an_integer = 6;
	println!("{}", an_integer);
	an_integer += 1;
	println!("{}", an_integer);

	let array: [i32; 5] = [1, 2, 3, 4, 5];
	println!("{}", array.len());

	let slice = &array[1 .. 3];
	println!("{}, {}", slice[0], slice.len());
	
	let sdl = sdl2::init().unwrap();
	let video_subsystem = sdl.video().unwrap();
	let window = video_subsystem
		.window("Game", 800, 600)
		.build()
		.unwrap();
	
	let mut event_pump = sdl.event_pump().unwrap();
	'main: loop
	{
		for event in event_pump.poll_iter()
		{
			match event
			{
				sdl2::event::Event::Quit { .. } => break 'main,
				_ => {}
			}
		}

		// Render here
	}
}
