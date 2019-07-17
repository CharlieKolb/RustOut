extern crate glutin_window;
extern crate piston;

use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

use opengl_graphics::{GlGraphics, OpenGL};

use glutin_window::GlutinWindow;

pub use crate::gameboard::Gameboard;
pub use crate::gameboard_controller::GameboardController;
pub use crate::gameboard_view::{GameboardView, GameboardViewSettings};

mod gameboard;
mod gameboard_controller;
mod gameboard_view;
mod vec2;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    let mut event_settings = EventSettings::new();
    event_settings.set_max_fps(144);
    event_settings.set_ups(288);

    let mut events = Events::new(event_settings);
    let mut gl = GlGraphics::new(opengl);

    let gameboard_view_settings = GameboardViewSettings::new();
    let mut gameboard_controller =
        GameboardController::new(Gameboard::new(gameboard_view_settings.size));
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, &c, g);
            });
        }
    }
}
