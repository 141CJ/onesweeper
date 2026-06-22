use macroquad::prelude::*;

const TILE_SIZE: f32 = 100.;
#[macroquad::main("OneSweeper")]
async fn main() {
    loop {
        clear_background(Color::from_hex(0xC0C0C0));

        draw_tile();

        next_frame().await
    }
}

fn draw_tile() {
    // top-left border of tile
    draw_triangle(
        vec2(
            (screen_width() / 2.) + TILE_SIZE + (TILE_SIZE / 10.),
            (screen_height() / 2.) - (TILE_SIZE / 10.),
        ),
        vec2(
            (screen_width() / 2.) - (TILE_SIZE / 10.),
            (screen_height() / 2.) - (TILE_SIZE / 10.),
        ),
        vec2(
            (screen_width() / 2.) - (TILE_SIZE / 10.),
            (screen_height() / 2.) + TILE_SIZE + (TILE_SIZE / 10.),
        ),
        Color::from_hex(0xFFFFFF),
    );

    // bottom-right border of tile
    draw_triangle(
        vec2(
            (screen_width() / 2.) - (TILE_SIZE / 10.),
            (screen_height() / 2.) + TILE_SIZE + (TILE_SIZE / 10.),
        ),
        vec2(
            (screen_width() / 2.) + TILE_SIZE + (TILE_SIZE / 10.),
            (screen_height() / 2.) + TILE_SIZE + (TILE_SIZE / 10.),
        ),
        vec2(
            (screen_width() / 2.) + TILE_SIZE + (TILE_SIZE / 10.),
            (screen_height() / 2.) - (TILE_SIZE / 10.),
        ),
        Color::from_hex(0x7E7E7E),
    );

    // tile center
    draw_rectangle(
        screen_width() / 2.,
        screen_height() / 2.,
        TILE_SIZE,
        TILE_SIZE,
        Color::from_hex(0xC0C0C0),
    );
}
