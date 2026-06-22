use macroquad::prelude::*;

#[macroquad::main("OneSweeper")]
async fn main() {
    loop {
        clear_background(Color::from_hex(0xC0C0C0));

        // top-left border of tile
        draw_triangle(
            vec2((screen_width() / 2.) + 110., (screen_height() / 2.) - 10.),
            vec2((screen_width() / 2.) - 10., (screen_height() / 2.) - 10.),
            vec2((screen_width() / 2.) - 10., (screen_height() / 2.) + 110.),
            Color::from_hex(0xFFFFFF),
        );

        // bottom-right border of tile
        draw_triangle(
            vec2((screen_width() / 2.) - 10., (screen_height() / 2.) + 110.),
            vec2((screen_width() / 2.) + 110., (screen_height() / 2.) + 110.),
            vec2((screen_width() / 2.) + 110., (screen_height() / 2.) - 10.),
            Color::from_hex(0x7E7E7E),
        );

        // tile center
        draw_rectangle(
            screen_width() / 2.,
            screen_height() / 2.,
            100.,
            100.,
            Color::from_hex(0xC0C0C0),
        );

        next_frame().await
    }
}
