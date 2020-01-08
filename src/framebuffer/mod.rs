use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::*;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 400;
const CHANNEL_COUNT: u32 = 3;

pub fn start_framebuffer() {
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
        .create_texture(PixelFormatEnum::RGB24, TextureAccess::Static, WIDTH, HEIGHT)
        .unwrap();

    let mut pixels: [u8; WIDTH as usize * HEIGHT as usize * CHANNEL_COUNT as usize] =
        [255; WIDTH as usize * HEIGHT as usize * CHANNEL_COUNT as usize];

    for x in 0..100 {
        for y in 0..100 {
            let x_: i32 = x - 50;
            let y_: i32 = y - 50;
            if x_ * x_ + y_ * y_ <= 2500 {
                color_pixel(&mut pixels, x as usize, y as usize, Color::RGB(0, 0, 0));
            }
        }
    }

    framebuffer
        .update(None, &pixels, WIDTH as usize * CHANNEL_COUNT as usize)
        .unwrap();

    let mut center: (i32, i32) = (0, 0);
    let radius: i32 = 50;

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

        // let state = event_pump.mouse_state();
        // color_pixel(
        //     &mut pixels,
        //     state.x() as usize,
        //     state.y() as usize,
        //     Color::RGB(0, 0, 0),
        // );

        center.0 += 1;
        center.1 = 200;
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let x_: i32 = center.0 - x as i32;
                let y_: i32 = center.1 - y as i32;
                let a = x_ * x_ + y_ * y_;
                if a <= radius * radius && a > 1000 {
                    color_pixel(&mut pixels, x as usize, y as usize, Color::RGB(0, 0, 0));
                } else {
                    color_pixel(
                        &mut pixels,
                        x as usize,
                        y as usize,
                        Color::RGB(255, 255, 255),
                    );
                }
            }
        }

        framebuffer
            .update(None, &pixels, WIDTH as usize * CHANNEL_COUNT as usize)
            .unwrap();

        canvas
            .copy(&framebuffer, None, Rect::new(0, 0, WIDTH, HEIGHT))
            .unwrap();

        canvas.present();
        canvas.clear();

        std::thread::sleep(Duration::from_millis(20));
    }
}

fn color_pixel(
    pixels: &mut [u8; WIDTH as usize * HEIGHT as usize * CHANNEL_COUNT as usize],
    x: usize,
    y: usize,
    color: Color,
) {
    let index: usize = (x + y * WIDTH as usize) * CHANNEL_COUNT as usize;
    pixels[index] = color.r;
    pixels[index + 1] = color.g;
    pixels[index + 2] = color.b;
}
