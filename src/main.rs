use crate::fn2::create_text_texture;
use crate::fn2::load_font;
use crate::fn2::render_character;
use crate::fn2::render_text;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod fn2;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_w = 640;
    let window_h = 480;
    let window = video_subsystem
        .window(".FN2 parser", window_w, window_h)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(5.0, 5.0).unwrap();

    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let font = load_font("./assets/TETRIS.FN2");
    let text_texture =
        create_text_texture(&mut canvas, &texture_creator, &mut &font, "Hello World!");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let mut x_offset = 0;
        let mut y_offset = 0;

        for c in 0..font.len() {
            let character_width = render_character(&mut canvas, &font, c, x_offset, y_offset);
            x_offset += character_width;

            if x_offset > 100 {
                y_offset += 10;
                x_offset = 0;
            }
        }

        render_text(&mut canvas, &text_texture, 0, 70);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
