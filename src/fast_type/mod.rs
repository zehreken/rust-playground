use console::Key;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::path::Path;
use std::time::Duration;

const CELL_WIDTH: i32 = 10;
const CELL_HEIGHT: i32 = 19;

pub fn start_fast_type() {
    let sample_text = "Talent is cheaper than table salt. What separates the talented individual from the successful one is a lot of hard work.";
    let term = console::Term::stdout();
    let mut input = String::from("_");
    term.write_line(sample_text);
    // let res = term.read_line_initial_text("");
    term.hide_cursor();
    let mut res;
    'running: loop {
        res = term.read_key();
        match res.unwrap() {
            Key::Char(c) => {
                input.pop();
                input.push(c);
                input.push_str("_");
            }
            Key::Escape => {
                break 'running
            }
            Key::Backspace => {
                input.pop();
                input.pop();
                input.push_str("_");
            }
            _ => {}
        }
        term.write_line(&input[..]);
        term.move_cursor_up(1);
    }
    println!("exit");
    return;
    const WIDTH: u32 = 500;
    const HEIGHT: u32 = 200;
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
    let font = ttf_context.load_font(font_path, 16).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let sample_text = "Talent is cheaper than table salt. What separates the talented individual from the successful one is a lot of hard work.";
    let mut chars: Vec<char> = Vec::new();
    let mut input_chars: Vec<char> = Vec::new();
    let mut match_chars: Vec<bool> = Vec::new();
    for b in sample_text.chars() {
        chars.push(b);
    }
    println!("chars: {}", chars.len());
    let words: Vec<&str> = sample_text.split(' ').collect();
    let word_count = words.len();
    let mut char_index: i32 = -1;
    let mut current_word_index = 0;
    let mut current_word = "".to_string();

    // for word in &words {
    //     println!("{}", word);
    // }
    let mut surface: Surface = font
        .render(sample_text)
        .blended_wrapped(Color::RGB(150, 150, 150), WIDTH)
        // .solid(Color::RGB(255, 255, 255))
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let mut text_query = texture.query();
    let text_rect = Rect::new(0, 0, text_query.width, text_query.height);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let cursor = "_"; // String literal is slice
    let mut input = cursor.to_string();
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
                        input.pop();
                        input.push_str(&cursor);
                        current_word.pop();
                        input_chars.pop();
                        match_chars.pop();
                        if char_index >= 0 {
                            char_index -= 1;
                        }
                    }
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    if current_word.len() > 0 {
                        println!("{} -> Next word!", current_word);
                        current_word = "".to_string();
                        current_word_index += 1;
                        if current_word_index == word_count {
                            println!("{}", "Sentence complete");
                        }
                    }
                }
                sdl2::event::Event::TextInput {
                    timestamp: _,
                    window_id: _,
                    text: text,
                } => {
                    if text != " " {
                        current_word.push_str(&text);
                    }
                    input.pop();
                    input.push_str(&text);
                    input.push_str(&cursor);
                    char_index += 1;
                    input_chars.push(if let Some(c) = text.chars().next() {
                        c
                    } else {
                        ' '
                    });
                    if chars[char_index as usize] == input_chars[char_index as usize] {
                        match_chars.push(true);
                    } else {
                        match_chars.push(false);
                    }
                    // println!(
                    // "check: {:?}, {:?}, {:?}",
                    // words[current_word_index] == current_word,
                    // words[current_word_index],
                    // current_word
                    // );
                }
                _ => {}
            }
        }

        if input.len() > 0 {
            for i in 0..match_chars.len() {
                if match_chars[i] {
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                }
                canvas
                    .fill_rect(Rect::new(
                        (i % 50) as i32 * CELL_WIDTH,
                        (i / 50) as i32 * CELL_HEIGHT,
                        CELL_WIDTH as u32,
                        CELL_HEIGHT as u32,
                    ))
                    .unwrap();
            }
        }
        canvas.copy(&texture, None, text_rect).unwrap();

        // canvas.set_draw_color(Color::RGB(255, 255, 255));

        // if char_index >= 0 {
        //     println!("{}, {}, {:?}", chars.len(), input_chars.len(), match_chars);
        // }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        if input.len() > 0 {
            surface = font
                .render(&input)
                .blended_wrapped(Color::RGB(0, 0, 0), WIDTH)
                .unwrap();
            input_texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            text_query = input_texture.query();
            let input_rect = Rect::new(0, 0, text_query.width, text_query.height);
            canvas.copy(&input_texture, None, input_rect).unwrap();
        }

        canvas.present();

        canvas.clear();

        ::std::thread::sleep(Duration::from_millis(20));
    }

    video_subsystem.text_input().stop();
}
