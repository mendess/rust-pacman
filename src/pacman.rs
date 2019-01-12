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
    x: f64,
    y: f64,
    direction: Direction,
    direction_intent: Direction,
    ghosts: [Ghost; 4],
    ghost_mode: GhostMode,
    ticks: u32,
    delta: f64,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up, Down, Left, Right
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
        self.ticks += 1; // TODO: make this usefull
        self.move_pacman(dt);
    }

    fn move_pacman(&mut self, dt: f64) {
        // if self.can_turn() {
        //     self.direction = self.direction_intent;
        // }
        let (x, y) = match self.direction {
            Direction::Up => (self.x, self.y - (dt * 4.0)),
            Direction::Down => (self.x, self.y + (dt * 4.0)),
            Direction::Left => (self.x - (dt * 4.0), self.y),
            Direction::Right => (self.x + (dt * 4.0), self.y),
        };
        let past_center = match self.direction {
            Direction::Up    => y < y.round(),
            Direction::Down  => y > y.round(),
            Direction::Left  => x < x.round(),
            Direction::Right => x > x.round(),
        };
        if past_center {
            let (ix, iy) = match self.direction {
                Direction::Up    => (x as u32          , y.floor() as u32),
                Direction::Down  => (x as u32          , y.ceil() as u32),
                Direction::Left  => (x.floor() as u32  , y as u32),
                Direction::Right => (x.ceil() as u32, y as u32),
            };
            println!("{:?}, {:?}", (x,y), (ix, iy));
            match self.map.get(ix, iy) {
                None | Some(Tile::Wall) => { self.x = self.x.round(); self.y = self.x.round(); }
                Some(Tile::NotWall(pu)) => {
                    self.x = x;
                    self.y = y;
                    match pu {
                        PU::Empty => (),
                        PU::Dot => {
                            self.map.consume(ix, iy);
                            self.score += 10;
                        },
                        PU::PowerUp => {
                            self.map.consume(ix, iy);
                            self.ghost_mode = GhostMode::Frightened;
                            self.score += 100;
                        },
                    }
                },
            }
        } else {
            self.x = x;
            self.y = y;
        }
        println!("{:?}", (self.x, self.y));
    }

    fn can_turn(&self) -> bool {
        let (x, y) = match self.direction_intent {
            Direction::Up => (self.x, self.y - 1.0),
            Direction::Down => (self.x, self.y + 1.0),
            Direction::Left => (self.x - 1.0, self.y),
            Direction::Right => (self.x + 1.0, self.y),
        };
        match self.map.get(x.floor() as u32, y.floor() as u32) {
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
            x: 1.0,
            y: 1.0,
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
