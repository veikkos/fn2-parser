use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::fs::{metadata, File};
use std::io::Read;

static INDEX_OFFSET: usize = 0x21;
static SPACE_WIDTH: u8 = 5;

#[derive(Debug)]
pub struct Line {
    pub x: u8,
    pub y: u8,
    pub width: u8,
}

#[derive(Debug)]
pub struct Character {
    pub width: u32,
    pub height: u32,
    pub lines: Vec<Line>,
}

pub type FN2 = Vec<Character>;

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let metadata = metadata(filename).expect("unable to read metadata");
    let size = metadata.len() as usize;
    let mut buffer = vec![0; size];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

pub fn load_font(filename: &str) -> FN2 {
    let data = get_file_as_byte_vec(filename);
    let mut font: FN2 = Vec::new();
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

        font.push(Character {
            width,
            height,
            lines,
        });

        if font.len() == number_of_chars_to_parse {
            break 'parsing;
        }
    }
    font
}

fn get_text_texture_size(font: &FN2, text: &str) -> (u32, u32) {
    let mut width = 0;
    let mut height = 0;

    for c in text.chars() {
        let character_index = char_to_index(c);
        if character_index < INDEX_OFFSET {
            width += SPACE_WIDTH as u32;
        } else {
            let index = (character_index - INDEX_OFFSET) as usize;
            let character = &font[index];
            width += character.width;
            if character.height > height {
                height = character.height;
            }
        }
    }

    (width, height)
}

fn char_to_index(c: char) -> usize {
    (c as u8).into()
}

pub fn create_text_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &FN2,
    text: &str,
) -> Texture<'a> {
    let (width, height) = get_text_texture_size(font, text);

    let mut texture = texture_creator
        .create_texture_target(PixelFormatEnum::RGBA8888, width, height)
        .map_err(|e| e.to_string())
        .unwrap();

    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            let mut offset = 0;

            texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
            texture_canvas.clear();
            texture_canvas.set_draw_color(Color::RGB(255, 0, 0));

            for c in text.chars() {
                let character_index = char_to_index(c);
                if character_index < INDEX_OFFSET {
                    offset += SPACE_WIDTH as u32;
                } else {
                    offset += render_character(
                        texture_canvas,
                        &font,
                        (character_index - INDEX_OFFSET) as usize,
                        offset,
                        0,
                    );
                }
            }
        })
        .map_err(|e| e.to_string())
        .unwrap();

    texture
}

pub fn render_character(
    canvas: &mut Canvas<Window>,
    characters: &FN2,
    index: usize,
    x: u32,
    y: u32,
) -> u32 {
    let character = &characters[index];
    for line in &character.lines {
        canvas
            .draw_line(
                Point::new(line.x as i32 + x as i32, line.y as i32 + y as i32),
                Point::new(
                    line.x as i32 + x as i32 + line.width as i32 - 1,
                    line.y as i32 + y as i32,
                ),
            )
            .unwrap();
    }
    character.width
}

pub fn render_text(canvas: &mut Canvas<Window>, texture: &Texture, x: i32, y: i32) {
    let TextureQuery { width, height, .. } = texture.query();
    let dst = Rect::new(x, y, width, height);
    canvas.copy(&texture, None, dst).unwrap();
}
