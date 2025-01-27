use std::{thread, time::Duration};

use context::Context;
use macroquad::{
    color::{GRAY, WHITE},
    math::vec2,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
    time::{self, get_frame_time},
    window::{clear_background, next_frame, screen_height, screen_width},
};

mod context;
mod instructions;
mod parser;
mod test_data;

struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

const WIDTH: u16 = 64;
const HEIGHT: u16 = 32;

const VIEWPORT_WIDTH: f32 = 64.0 * 10.0;
const VIEWPORT_HEIGHT: f32 = 32.0 * 10.0;

#[macroquad::main("Chip-8 Emulator")]
async fn main() {
    let data = test_data::DATA;
    let mut context: Context = Context::new(&data, time::get_time() as u64);

    let graphics_buffer = convert_graphics_buffer(&context.get_flat_graphics_buffer());
    let texture = Texture2D::from_rgba8(WIDTH, HEIGHT, &graphics_buffer);
    texture.set_filter(macroquad::texture::FilterMode::Nearest);

    loop {
        clear_background(GRAY);
        context.tick();

        let graphics_buffer = convert_graphics_buffer(&context.get_flat_graphics_buffer());

        texture.update_from_bytes(
            texture.width() as u32,
            texture.height() as u32,
            &graphics_buffer,
        );

        draw_texture_ex(
            &texture,
            (screen_width() / 2.0) - (VIEWPORT_WIDTH / 2.0),
            (screen_height() / 2.0) - (VIEWPORT_HEIGHT / 2.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(VIEWPORT_WIDTH, VIEWPORT_HEIGHT)),
                ..Default::default()
            },
        );

        thread::sleep(Duration::from_millis(60));
        next_frame().await;
    }
}

fn convert_graphics_buffer(buffer: &[u8]) -> Vec<u8> {
    let result = buffer
        .iter()
        .flat_map(|byte| {
            let mut colors: Vec<u8> = vec![];
            let mut i = 8;
            while i > 0 {
                let bit = (byte & (1 << (i - 1))) >> (i - 1);
                for _ in 0..3 {
                    colors.push(!(0xFF >> (8 * bit)) as u8);
                }
                colors.push(0xFF);
                i -= 1
            }
            colors
        })
        .collect::<Vec<u8>>();
    result
}
