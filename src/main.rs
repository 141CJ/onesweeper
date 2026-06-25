use ::rand::{RngExt, rng};
use macroquad::{miniquad::TextureParams, prelude::*};

const TILE_SIZE: f32 = 100.;
const TILE_BORDER_SIZE: f32 = TILE_SIZE / 10.;
const STATS_BAR_SIZE: f32 = 150.;
const WINDOW_BORDER_SIZE: f32 = 20.;
const WINDOW_BORDER_OUTLINE_SIZE: f32 = 10.;

fn window_config() -> Conf {
    Conf {
        window_title: "OneSweeper".to_owned(),
        window_height: 800,
        window_width: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut game = OneSweeper::new().await;
    game.mine_texture.set_filter(FilterMode::Nearest);
    game.win_texture.set_filter(FilterMode::Nearest);
    loop {
        let center_x = screen_width() / 2. - TILE_SIZE / 2.;
        let center_y = screen_height() / 2. - TILE_SIZE / 2.;
        clear_background(Color::from_hex(0xC0C0C0));

        game.draw_window_border();
        game.draw_tile(center_x, center_y);
        game.handle_input(center_x, center_y);
        game.draw_reset_tile(center_x);

        next_frame().await
    }
}

struct OneSweeper {
    tile_clicked: bool,
    mine_placed: bool,
    mine_texture: Texture2D,
    win_texture: Texture2D,
}

impl OneSweeper {
    async fn new() -> Self {
        let mut rng = rng();

        OneSweeper {
            tile_clicked: false,
            mine_placed: rng.random_bool(0.5),
            mine_texture: load_texture("assets/mine.png").await.unwrap(),
            win_texture: load_texture("assets/win.png").await.unwrap(),
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
        if mouse_x >= center_x - TILE_BORDER_SIZE
            && mouse_x <= center_x + (TILE_SIZE + TILE_BORDER_SIZE)
        {
            if mouse_y >= (STATS_BAR_SIZE / 4.) - TILE_BORDER_SIZE
                && mouse_y <= (STATS_BAR_SIZE / 4.) + (TILE_SIZE + TILE_BORDER_SIZE)
            {
                if is_mouse_button_pressed(MouseButton::Left) {}
            }
        }
    }

    fn draw_window_border(&mut self) {
        // top-left border
        draw_triangle(
            vec2(WINDOW_BORDER_SIZE, STATS_BAR_SIZE + WINDOW_BORDER_SIZE), // top left
            vec2(
                screen_width() - WINDOW_BORDER_SIZE,
                WINDOW_BORDER_SIZE + STATS_BAR_SIZE,
            ), // top right
            vec2(WINDOW_BORDER_SIZE, screen_height() - WINDOW_BORDER_SIZE), // bottom left
            Color::from_hex(0x7E7E7E),
        );
        // bottom-right border
        draw_triangle(
            vec2(
                screen_width() - WINDOW_BORDER_SIZE,
                WINDOW_BORDER_SIZE + STATS_BAR_SIZE,
            ), // top right
            vec2(
                screen_width() - WINDOW_BORDER_SIZE,
                screen_height() - WINDOW_BORDER_SIZE,
            ), // bottom right
            vec2(WINDOW_BORDER_SIZE, screen_height() - WINDOW_BORDER_SIZE), // bottom left
            Color::from_hex(0xFFFFFF),
        );

        draw_rectangle(
            WINDOW_BORDER_SIZE + WINDOW_BORDER_OUTLINE_SIZE,
            WINDOW_BORDER_SIZE + WINDOW_BORDER_OUTLINE_SIZE + STATS_BAR_SIZE,
            screen_width() - (WINDOW_BORDER_SIZE * 2.) - (WINDOW_BORDER_OUTLINE_SIZE * 2.),
            screen_height()
                - (WINDOW_BORDER_OUTLINE_SIZE * 2.)
                - (WINDOW_BORDER_SIZE * 2.)
                - STATS_BAR_SIZE,
            Color::from_hex(0xC0C0C0),
        );
    }

    fn draw_tile(&mut self, center_x: f32, center_y: f32) {
        if !self.tile_clicked {
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
        } else if self.tile_clicked {
            if self.mine_placed {
                self.draw_win_loss_tile(center_x, center_y, Color::from_hex(0xFF0000));

                draw_texture_ex(
                    &self.mine_texture,
                    center_x,
                    center_y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                );
            } else {
                self.draw_win_loss_tile(center_x, center_y, Color::from_hex(0xC0C0C0));

                draw_texture_ex(
                    &self.win_texture,
                    center_x + (TILE_BORDER_SIZE / 2.),
                    center_y + (TILE_BORDER_SIZE / 2.),
                    BLUE,
                    DrawTextureParams {
                        dest_size: Some(vec2(
                            TILE_SIZE - TILE_BORDER_SIZE,
                            TILE_SIZE - TILE_BORDER_SIZE,
                        )),
                        ..Default::default()
                    },
                );
            }
        }
    }

    fn draw_win_loss_tile(&mut self, center_x: f32, center_y: f32, color: Color) {
        draw_rectangle(
            center_x - (TILE_BORDER_SIZE / 2.),
            center_y - (TILE_BORDER_SIZE / 2.),
            TILE_SIZE + TILE_BORDER_SIZE,
            TILE_SIZE + TILE_BORDER_SIZE,
            Color::from_hex(0x7E7E7E),
        );
        draw_rectangle(
            center_x + WINDOW_BORDER_OUTLINE_SIZE + WINDOW_BORDER_SIZE,
            center_y + WINDOW_BORDER_OUTLINE_SIZE + WINDOW_BORDER_SIZE + STATS_BAR_SIZE,
            TILE_SIZE,
            TILE_SIZE,
            color,
        );
    }
    fn draw_reset_tile(&mut self, center_x: f32) {
        // top-left border of tile
        draw_triangle(
            vec2(
                center_x + TILE_SIZE + TILE_BORDER_SIZE,
                (STATS_BAR_SIZE / 4.) - TILE_BORDER_SIZE,
            ),
            vec2(
                center_x - TILE_BORDER_SIZE,
                (STATS_BAR_SIZE / 4.) - TILE_BORDER_SIZE,
            ),
            vec2(
                center_x - TILE_BORDER_SIZE,
                (STATS_BAR_SIZE / 4.) + TILE_SIZE + TILE_BORDER_SIZE,
            ),
            Color::from_hex(0xFFFFFF),
        );

        // bottom-right border of tile
        draw_triangle(
            vec2(
                center_x - TILE_BORDER_SIZE,
                (STATS_BAR_SIZE / 4.) + TILE_SIZE + TILE_BORDER_SIZE,
            ),
            vec2(
                center_x + TILE_SIZE + TILE_BORDER_SIZE,
                (STATS_BAR_SIZE / 4.) + TILE_SIZE + TILE_BORDER_SIZE,
            ),
            vec2(
                center_x + TILE_SIZE + TILE_BORDER_SIZE,
                (STATS_BAR_SIZE / 4.) - TILE_BORDER_SIZE,
            ),
            Color::from_hex(0x7E7E7E),
        );

        // tile center
        draw_rectangle(
            center_x,
            STATS_BAR_SIZE / 4.,
            TILE_SIZE,
            TILE_SIZE,
            Color::from_hex(0xC0C0C0),
        );
    }
}
