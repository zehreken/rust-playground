use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::keyboard::TextInputUtil;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;

pub fn start_fast_type() {
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 512;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.text_input().start();

    let window = video_subsystem
        .window("fast_type", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("fonts/VeraMono.ttf");
    let font = ttf_context.load_font(font_path, 18).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let test_text = "There are two motives for reading a book; one, that you enjoy it; the other, that you can boast about it.";
    let mut surface: Surface = font
        .render(test_text)
        .blended_wrapped(Color::RGB(255, 255, 255), WIDTH)
        // .solid(Color::RGB(255, 255, 255))
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let mut text_query = texture.query();
    let text_rect = Rect::new(0, 0, text_query.width, text_query.height);

    let shift_text = "SHIFT";
    surface = font
        .render(shift_text)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let mut shift_texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    text_query = shift_texture.query();
    let shift_rect = Rect::new(0, 200, text_query.width, text_query.height);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_shift_pressed: bool = false;

    let mut input = "".to_string();
    let mut input_texture;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    if input.len() > 0 {
                        input.pop();
                    }
                }
                sdl2::event::Event::TextInput {
                    timestamp: _,
                    window_id: _,
                    text: text,
                } => {
                    input.push_str(&text);
                }
                _ => {}
            }
        }

        canvas.copy(&texture, None, text_rect).unwrap();

        if is_shift_pressed {
            canvas.copy(&shift_texture, None, shift_rect).unwrap();
        }

        if input.len() > 0 {
            surface = font
                .render(&input)
                .blended_wrapped(Color::RGB(255, 255, 255), WIDTH)
                .unwrap();
            input_texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            text_query = input_texture.query();
            let input_rect = Rect::new(0, 250, text_query.width, text_query.height);
            canvas.copy(&input_texture, None, input_rect).unwrap();
        }

        canvas.present();

        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        ::std::thread::sleep(Duration::from_millis(20));
    }

    video_subsystem.text_input().stop();
}
