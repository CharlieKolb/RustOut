//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::gameboard;
use crate::gameboard_controller::GameboardController;

/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 0.5],
            border_color: [0.0, 0.0, 0.2, 0.5],
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

fn u8_color_to_f32_color (color: [u8; 4]) -> [f32; 4] {
    [
        color[0] as f32 / 255.,
        color[1] as f32 / 255.,
        color[2] as f32 / 255.,
        color[3] as f32 / 255.,

    ]
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView { settings }
    }

    fn draw_hitbox<G: Graphics>(&self, color: Color, rect: [f64; 4], c: &Context, g: &mut G) {
        use graphics::Rectangle;

        Rectangle::new(color).draw(rect, &c.draw_state, c.transform, g);
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G) {
        use graphics::Rectangle;
        let ref board = controller.gameboard;

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.size,
            settings.size,
        ];

        // Draw board background.
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        let rect_of_hitbox = |hitbox: &gameboard::Rectangle| {
            [
                self.settings.position[0] + hitbox.position.x,
                self.settings.position[1] + hitbox.position.y,
                hitbox.dimension.x,
                hitbox.dimension.y,
            ]
        };
        self.draw_hitbox(
            [1.0, 0.0, 0.0, 1.0],
            rect_of_hitbox(&board.player.body.hitbox),
            &c,
            g,
        );
        self.draw_hitbox(
            [1.0, 0.0, 0.0, 1.0],
            rect_of_hitbox(&board.ball.body.hitbox),
            &c,
            g,
        );

        for block in &board.blocks {
            self.draw_hitbox(
                u8_color_to_f32_color(block.color),
                rect_of_hitbox(&block.body.hitbox),
                &c,
                g,
            );
        }
    }
}
