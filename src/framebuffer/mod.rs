use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
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
        .create_texture(None, TextureAccess::Static, WIDTH, HEIGHT)
        .unwrap();
    let ys: [i32; 500] = [0; 500];
    let pixels: [u8; WIDTH as usize * HEIGHT as usize] = [255; WIDTH as usize * HEIGHT as usize];
    framebuffer.update(None, &pixels, 1).unwrap();

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

        canvas
            .copy(&framebuffer, None, Rect::new(0, 0, WIDTH, HEIGHT))
            .unwrap();

        std::thread::sleep(Duration::from_millis(20));
    }
}
