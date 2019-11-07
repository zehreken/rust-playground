use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::path::Path;

pub fn start_fast_type() {
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 512;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("fast_type", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("emulogic.ttf");
    let font = ttf_context.load_font(font_path, 8).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let test_text = "There are two motives for reading a book; one, that you enjoy it; the other, that you can boast about it.";
    let mut surface: Surface = font
        .render(test_text)
        .solid(Color::RGB(255, 255, 255))
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let text_query = texture.query();
    let text_rect = Rect::new(0, 0, text_query.width, text_query.height);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.copy(&texture, None, text_rect).unwrap();
        
        canvas.present();

        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        // canvas.clear();
    }
}
