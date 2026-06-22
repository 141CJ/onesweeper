use macroquad::prelude::*;

const TILE_SIZE: f32 = 100.;
const TILE_BORDER_SIZE: f32 = TILE_SIZE / 10.;
#[macroquad::main("OneSweeper")]
async fn main() {
    let mut game = OneSweeper::new();
    loop {
        let center_x = screen_width() / 2.;
        let center_y = screen_height() / 2.;
        clear_background(Color::from_hex(0xC0C0C0));

        game.draw_tile(center_x, center_y);
        game.handle_input(center_x, center_y);

        next_frame().await
    }
}

struct OneSweeper {
    tile_clicked: bool,
}

impl OneSweeper {
    fn new() -> Self {
        OneSweeper {
            tile_clicked: false,
        }
    }

    fn handle_input(&mut self, center_x: f32, center_y: f32) {
        let (mouse_x, mouse_y) = mouse_position();
        // checks if cursor is within x bounds of tile
        if mouse_x >= center_x - TILE_BORDER_SIZE
            && mouse_x <= center_x + (TILE_SIZE + TILE_BORDER_SIZE)
        {
            // checks if cursor is within y bounds of tile
            if mouse_y >= center_y - TILE_BORDER_SIZE
                && mouse_y <= center_y + (TILE_SIZE + TILE_BORDER_SIZE)
            {
                if is_mouse_button_pressed(MouseButton::Left) {
                    self.tile_clicked = true;
                }
            }
        }
    }

    fn draw_tile(&mut self, center_x: f32, center_y: f32) {
        // top-left border of tile
        draw_triangle(
            vec2(
                center_x + TILE_SIZE + TILE_BORDER_SIZE,
                center_y - TILE_BORDER_SIZE,
            ),
            vec2(center_x - TILE_BORDER_SIZE, center_y - TILE_BORDER_SIZE),
            vec2(
                center_x - TILE_BORDER_SIZE,
                center_y + TILE_SIZE + TILE_BORDER_SIZE,
            ),
            Color::from_hex(0xFFFFFF),
        );

        // bottom-right border of tile
        draw_triangle(
            vec2(
                center_x - TILE_BORDER_SIZE,
                center_y + TILE_SIZE + TILE_BORDER_SIZE,
            ),
            vec2(
                center_x + TILE_SIZE + TILE_BORDER_SIZE,
                center_y + TILE_SIZE + TILE_BORDER_SIZE,
            ),
            vec2(
                center_x + TILE_SIZE + TILE_BORDER_SIZE,
                center_y - TILE_BORDER_SIZE,
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
}
