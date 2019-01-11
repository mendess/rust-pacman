use crate::pacman::{ Pacman, Direction, ghost::Ghost, map::Map };
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
                Button::Keyboard(Key::Up) => self.game.set_direction_intent(Direction::Up),
                Button::Keyboard(Key::Down) => self.game.set_direction_intent(Direction::Down),
                Button::Keyboard(Key::Left) => self.game.set_direction_intent(Direction::Left),
                Button::Keyboard(Key::Right) => self.game.set_direction_intent(Direction::Right),
                Button::Keyboard(Key::Q) => return true,
                _ => (),
            }
        }

        if let Some(u) = event.update_args() {
            self.delta += u.dt;
            self.game.update_float_coords(u.dt);
            if self.delta > 0.1 {
                self.delta -= 0.1;
                self.game.tick(u.dt);
            }
        }

        false
    }

    // pub fn get_player(&self) -> (f64, f64, Direction) {
    //     let (x, y, d) = self.game.player();
    //     let delta = self.delta * 10.0;
    //     match d {
    //         Direction::Up =>    (x.into()        , y as f64 - delta, self.game.direction_intent()),
    //         Direction::Down =>  (x.into()        , y as f64 + delta, self.game.direction_intent()),
    //         Direction::Left =>  (x as f64 - delta, y.into()        , self.game.direction_intent()),
    //         Direction::Right => (x as f64 + delta, y.into()        , self.game.direction_intent()),
    //         Direction::Still => (x.into(), y.into(), self.game.direction_intent()),
    //     }
    // }

    pub fn get_player(&self) -> (f64, f64, Direction) {
        self.game.player()
    }

    pub fn get_map(&self) -> &Map {
        self.game.map()
    }

    pub fn get_ghosts(&self) -> &[Ghost] {
        self.game.ghosts()
    }
}
