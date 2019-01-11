use crate::pacman::{ Pacman, Direction };
use piston::input::Event;
use piston::input::{ PressEvent, UpdateEvent };
use piston::input::Button;

pub struct Controler {
    game: Pacman,
}

impl Controler {
    pub fn new(game :Pacman) -> Self {
        Controler { game }
    }

    pub fn event(&mut self, event :&Event) -> bool {
        if let Some(k) = event.press_args() {
            use piston::input::keyboard::Key;
            match k {
                Button::Keyboard(Key::Up) => self.game.set_direction(Direction::Up),
                Button::Keyboard(Key::Down) => self.game.set_direction(Direction::Down),
                Button::Keyboard(Key::Left) => self.game.set_direction(Direction::Left),
                Button::Keyboard(Key::Right) => self.game.set_direction(Direction::Right),
                Button::Keyboard(Key::Q) => return true,
                _ => (),
            }
        }

        if let Some(u) = event.update_args() {
            self.game.tick();
        }

        false
    }

    pub const fn map_width() -> u32 {
        super::pacman::Pacman::map_width()
    }

    pub const fn map_height() -> u32 {
        super::pacman::Pacman::map_height()
    }

    pub fn get_player(&self) -> (u32, u32, Direction) {
        self.game.player()
    }
}
