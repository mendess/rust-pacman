pub mod map;
pub mod ghost;

use self::map::Map;
use self::map::Tile;
use self::map::PU;

use self::ghost::{Ghost, GhostMode};

const TICK_RATE :u32 = 5;

#[allow(dead_code)]
pub struct Pacman {
    map: Map,
    lives: u8,
    score: u32,
    level: u32,
    x: u32,
    y: u32,
    xf: f64,
    yf: f64,
    direction: Direction,
    direction_intent: Direction,
    ghosts: [Ghost; 4],
    ghost_mode: GhostMode,
    ticks: u32,
    delta: f64,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up, Down, Left, Right, Still
}

#[allow(dead_code)]
pub struct Stats {
    pub lives: u8,
    pub score: u32,
    pub level: u32,
    pub ghostbusting: bool,
}

impl Pacman {
    pub fn new() -> Self {
        Pacman::default()
    }

    pub fn set_direction_intent(&mut self, direction: Direction) {
        self.direction_intent = direction;
        if self.can_turn() {
            self.direction = self.direction_intent;
        }
    }

    pub fn direction_intent(&self) -> Direction {
        self.direction_intent
    }

    pub fn tick(&mut self, dt: f64) {
        self.ticks += 1;
        self.move_pacman(dt);
    }

    pub fn update_float_coords(&mut self, dt: f64) {
        let delta = dt * 10.0;
        match self.direction {
            Direction::Up =>    self.yf -= delta,
            Direction::Down =>  self.yf += delta,
            Direction::Left =>  self.xf -= delta,
            Direction::Right => self.xf += delta,
            Direction::Still => (),
        };
    }

    fn move_pacman(&mut self, _dt: f64) {
        if self.can_turn() {
            self.direction = self.direction_intent;
        }
        let (x, y) = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
            Direction::Still => return,
        };
        match self.map.get(x, y) {
            None => (),
            Some(Tile::Wall) => self.direction = Direction::Still,
            Some(Tile::NotWall(pu)) => {
                self.x = x;
                self.y = y;
                match pu {
                    PU::Empty => (),
                    PU::Dot => {
                        self.map.consume(x, y);
                        self.score += 10;
                    },
                    PU::PowerUp => {
                        self.map.consume(x, y);
                        self.ghost_mode = GhostMode::Frightened;
                        self.score += 100;
                    },
                }
            },
        }
    }

    fn can_turn(&self) -> bool {
        let (x, y) = match self.direction_intent {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
            Direction::Still => unreachable!(),
        };
        match self.map.get(x, y) {
            None => false,
            Some(Tile::Wall) => false,
            _ => true
        }
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn player(&self) -> (f64, f64, Direction) {
        (self.x.into(), self.y.into(), self.direction_intent)
    }

    pub fn ghosts(&self) -> &[Ghost] {
        &self.ghosts
    }

    // pub fn stats(&self) -> Stats {
    //     Stats {
    //         lives: self.lives,
    //         score: self.score,
    //         level: self.level,
    //     }
    // }
}

impl Default for Pacman {
    fn default() -> Self {
        Pacman {
            map: Map::new(),
            lives: 5,
            score: 0,
            level: 1,
            x: 1,
            y: 1,
            xf: 1.0,
            yf: 1.0,
            direction: Direction::Left,
            direction_intent: Direction::Left,
            ghosts: [
                Ghost{ x: 15, y: 15, ttr: 0.0 }, // blinky
                Ghost{ x: 5, y: 5, ttr: 0.0 }, // pinky
                Ghost{ x: 5, y: 5, ttr: 0.0 }, // inky
                Ghost{ x: 5, y: 5, ttr: 0.0 }  // clyde
            ],
            ghost_mode: GhostMode::Scatter,
            ticks: 0,
            delta: 0.0,
        }
    }
}
