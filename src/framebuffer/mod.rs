use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::*;
use std::time::Duration;

pub fn start_framebuffer() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 400;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("framebuffer", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut framebuffer = texture_creator
        .create_texture(
            PixelFormatEnum::RGB24,
            TextureAccess::Static,
            WIDTH,
            HEIGHT,
        )
        .unwrap();

    const CHANNEL_COUNT: usize = 3;
    let mut pixels: [u8; WIDTH as usize * HEIGHT as usize * CHANNEL_COUNT] =
        [255; WIDTH as usize * HEIGHT as usize * CHANNEL_COUNT];
    let mut offset:usize = 0;
    for i in 0..(WIDTH * HEIGHT) {
        if i < 160000 {
            pixels[offset] = 255;
            pixels[offset + 1] = 0;
            pixels[offset + 2] = 0;
            offset += 3;
        } else {
            // pixels[i as usize] = (i % 200) as u8;
        }
    }
    framebuffer
        .update(None, &pixels, WIDTH as usize * CHANNEL_COUNT)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let state = event_pump.mouse_state();
        // println!("mouse x: {}, y: {}", state.x(), state.y());
        let point:usize = ((state.x() + state.y() * WIDTH as i32) * CHANNEL_COUNT as i32) as usize;
        pixels[point] = 0;
        pixels[point + 1] = 0;
        pixels[point + 2] = 0;
        framebuffer
        .update(None, &pixels, WIDTH as usize * CHANNEL_COUNT)
        .unwrap();

        canvas
            .copy(&framebuffer, None, Rect::new(0, 0, WIDTH, HEIGHT))
            .unwrap();

        canvas.present();
        canvas.clear();

        std::thread::sleep(Duration::from_millis(20));
    }
}
