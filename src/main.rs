use macroquad::{
    color::{BLACK, WHITE},
    shapes::draw_rectangle_lines,
    window::{clear_background, next_frame},
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

#[macroquad::main("Chip-8 Emulator")]
async fn main() {
    let viewport_rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 64.0,
        height: 32.0,
    };

    loop {
        clear_background(WHITE);

        draw_rectangle_lines(
            viewport_rect.x,
            viewport_rect.y,
            viewport_rect.width,
            viewport_rect.height,
            1.0,
            BLACK,
        );

        next_frame().await
    }
}
