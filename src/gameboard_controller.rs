//! Gameboard controller.

use piston::input::GenericEvent;
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::Event;

use crate::gameboard::{Direction, Gameboard};

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> Self {
        Self { gameboard }
    }

    /// Handles events.
    pub fn event(&mut self, e: &Event) {
        use piston::input::{Button, ButtonState, Input, Key, Loop, UpdateArgs};
        match e {
            Event::Input(Input::Button(button_args), _) => {
                // println!("{:?}", e);
                let pressed = button_args.state == ButtonState::Press;
                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::Left => {
                            let dir = &mut self.gameboard.player.direction;
                            if pressed {
                                *dir = Direction::Left;
                            } else {
                                if *dir == Direction::Left {
                                    *dir = Direction::Idle;
                                }
                            }
                        }
                        Key::Right => {
                            let dir = &mut self.gameboard.player.direction;
                            if pressed {
                                *dir = Direction::Right;
                            } else {
                                if *dir == Direction::Right {
                                    *dir = Direction::Idle;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::Loop(Loop::Update(UpdateArgs { dt })) => {
                // println!("{:?}", e);
                self.gameboard.update(*dt);
            }
            _ => (),
        }
    }
}
