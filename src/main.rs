use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

#[derive(Debug)]
struct Line {
    x: u8,
    y: u8,
    width: u8,
}

#[derive(Debug)]
struct Character {
    width: u32,
    height: u32,
    lines: Vec<Line>,
}

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

    let mut event_pump = sdl_context.event_pump().unwrap();

    let data = get_file_as_byte_vec("./assets/TETRIS.FN2");
    let size = data.len();
    assert_eq!(size % 8, 0);

    println!("File size: {} bytes", size);
    let mut characters: Vec<Character> = Vec::new();
    let mut offset: usize = 0x027D;
    let number_of_chars_to_parse = 92;
    'parsing: loop {
        let width = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let height = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let color_bytes = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let line_bytes = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        offset += color_bytes as usize;

        let mut lines: Vec<Line> = Vec::new();
        for _line in 0..(line_bytes / 3) {
            let line = Line {
                x: data[offset],
                y: data[offset + 1],
                width: data[offset + 2],
            };
            if line.width > 0 {
                lines.push(line);
            }
            offset += 3;
        }

        characters.push(Character {
            width,
            height,
            lines,
        });

        if characters.len() == number_of_chars_to_parse {
            break 'parsing;
        }
    }

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

        for c in 0..characters.len() {
            let character = &characters[c];
            for line in &character.lines {
                canvas
                    .draw_line(
                        Point::new(line.x as i32 + x_offset, line.y as i32 + y_offset),
                        Point::new(
                            line.x as i32 + x_offset + line.width as i32 - 1,
                            line.y as i32 + y_offset,
                        ),
                    )
                    .unwrap();
            }
            x_offset += character.width as i32;

            if x_offset > 100 {
                y_offset += 10;
                x_offset = 0;
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn get_color(data: u8) -> (u8, u8, u8) {
    let r = (((data >> 5) as u32) * 255 / 7) as u8;
    let g = ((((data & 0x1C) >> 2) as u32) * 255 / 7) as u8;
    let b = (((data & 0x03) as u32) * 255 / 3) as u8;
    (r, g, b)
}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let metadata = fs::metadata(filename).expect("unable to read metadata");
    let size = metadata.len() as usize;
    let mut buffer = vec![0; size];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

#[test]
fn color_tests() {
    let (r, g, b) = get_color(0xFF);
    assert_eq!(r, 255);
    assert_eq!(g, 255);
    assert_eq!(b, 255);

    let (r, g, b) = get_color(0xE0);
    assert_eq!(r, 255);
    assert_eq!(g, 0);
    assert_eq!(b, 0);

    let (r, g, b) = get_color(0x1C);
    assert_eq!(r, 0);
    assert_eq!(g, 255);
    assert_eq!(b, 0);

    let (r, g, b) = get_color(0x03);
    assert_eq!(r, 0);
    assert_eq!(g, 0);
    assert_eq!(b, 255);
}
