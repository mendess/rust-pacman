use crate::pacman::{ Pacman, Direction, ghost::Ghost, map::Map, ghost::GhostMode };
use piston::input::Event;
use piston::input::{ PressEvent, UpdateEvent };
use piston::input::Button;

pub struct Controler {
    game: Pacman,
    delta: f64,
}

impl Controler {
    pub fn new(game :Pacman) -> Self {
        Controler { game, delta: 0.0 }
    }

    pub fn event(&mut self, event :&Event) -> bool {
        if let Some(k) = event.press_args() {
            use piston::input::keyboard::Key;
            match k {
                Button::Keyboard(Key::Up)    => self.game.set_direction_intent(Direction::Up),
                Button::Keyboard(Key::Down)  => self.game.set_direction_intent(Direction::Down),
                Button::Keyboard(Key::Left)  => self.game.set_direction_intent(Direction::Left),
                Button::Keyboard(Key::Right) => self.game.set_direction_intent(Direction::Right),
                Button::Keyboard(Key::Q) => return true,
                Button::Keyboard(Key::K) => self.game.set_direction_intent(Direction::Up),
                Button::Keyboard(Key::J) => self.game.set_direction_intent(Direction::Down),
                Button::Keyboard(Key::H) => self.game.set_direction_intent(Direction::Left),
                Button::Keyboard(Key::L) => self.game.set_direction_intent(Direction::Right),
                _ => (),
            }
        }

        if let Some(u) = event.update_args() {
            self.delta += u.dt;
            if self.delta > 0.25 {
                self.delta -= 0.25;
                self.game.tick();
            }
        }

        false
    }

    pub fn get_player(&self) -> (f64, f64, Direction) {
        self.game.player()
    }

    pub fn get_map(&self) -> &Map {
        self.game.map()
    }

    pub fn get_ghosts(&self) -> &[Ghost] {
        self.game.ghosts()
    }

    pub fn frightened(&self) -> bool {
        self.game.ghost_mode() == GhostMode::Frightened
    }

    // DEBUG VIEWS
    pub fn ghost_targets(&self) -> [(i32, i32); 4] {
        self.game.ghost_targets()
    }
}
