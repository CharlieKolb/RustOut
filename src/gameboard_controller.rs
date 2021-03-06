//! Gameboard controller.
use piston::Event;
use piston::input::GenericEvent;

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
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, ButtonState, Key, UpdateArgs};

        if let Some(button_args) = e.button_args() {
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
        
        if let Some(UpdateArgs { dt }) = e.update_args() {
            // println!("{}", dt);
            self.gameboard.update(dt);
        }
    }
}
