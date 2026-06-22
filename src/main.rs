use macroquad::prelude::*;

const TILE_SIZE: f32 = 100.;
#[macroquad::main("OneSweeper")]
async fn main() {
    loop {
        let center_x = screen_width() / 2.;
        let center_y = screen_height() / 2.;
        clear_background(Color::from_hex(0xC0C0C0));

        draw_tile(center_x, center_y);
        handle_input(center_x, center_y);

        next_frame().await
    }
}

fn handle_input(center_x: f32, center_y: f32) {
    // checks if cursor is within x bounds of tile
    if mouse_position().0 >= center_x - (TILE_SIZE / 10.)
        && mouse_position().0 <= center_x + (TILE_SIZE + TILE_SIZE / 10.)
    {
        // checks if cursor is within y bounds of tile
        if mouse_position().1 >= center_y - (TILE_SIZE / 10.)
            && mouse_position().1 <= center_y + (TILE_SIZE + TILE_SIZE / 10.)
        {
            if is_mouse_button_pressed(MouseButton::Left) {
                println!("{:?}", mouse_position());
            }
        }
    }
}

fn draw_tile(center_x: f32, center_y: f32) {
    // top-left border of tile
    draw_triangle(
        vec2(
            center_x + TILE_SIZE + (TILE_SIZE / 10.),
            center_y - (TILE_SIZE / 10.),
        ),
        vec2(center_x - (TILE_SIZE / 10.), center_y - (TILE_SIZE / 10.)),
        vec2(
            center_x - (TILE_SIZE / 10.),
            center_y + TILE_SIZE + (TILE_SIZE / 10.),
        ),
        Color::from_hex(0xFFFFFF),
    );

    // bottom-right border of tile
    draw_triangle(
        vec2(
            center_x - (TILE_SIZE / 10.),
            center_y + TILE_SIZE + (TILE_SIZE / 10.),
        ),
        vec2(
            center_x + TILE_SIZE + (TILE_SIZE / 10.),
            center_y + TILE_SIZE + (TILE_SIZE / 10.),
        ),
        vec2(
            center_x + TILE_SIZE + (TILE_SIZE / 10.),
            center_y - (TILE_SIZE / 10.),
        ),
        Color::from_hex(0x7E7E7E),
    );

    // tile center
    draw_rectangle(
        center_x,
        center_y,
        TILE_SIZE,
        TILE_SIZE,
        Color::from_hex(0xC0C0C0),
    );
}
