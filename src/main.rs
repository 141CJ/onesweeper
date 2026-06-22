use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(Color::from_hex(0xC0C0C0));

        // bottom-right border of tile
        draw_triangle(
            vec2((screen_width() / 2.) - 10., (screen_height() / 2.) + 110.),
            vec2((screen_width() / 2.) + 110., (screen_height() / 2.) + 110.),
            vec2((screen_width() / 2.) + 110., (screen_height() / 2.) - 10.),
            BLUE,
        );

        // top-left border of tile
        draw_triangle(
            vec2((screen_width() / 2.) + 110., (screen_height() / 2.) - 10.),
            vec2((screen_width() / 2.) - 10., (screen_height() / 2.) - 10.),
            vec2((screen_width() / 2.) - 10., (screen_height() / 2.) + 110.),
            GREEN,
        );

        // tile center
        draw_rectangle(screen_width() / 2., screen_height() / 2., 100., 100., RED);

        next_frame().await
    }
}
