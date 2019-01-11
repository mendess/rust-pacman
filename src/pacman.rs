pub mod map;

use self::map::Map;
use self::map::Tile;
use self::map::PU;

pub struct Pacman {
    map: Map,
    lives: u8,
    score: u32,
    level: u32,
    x: f64,
    y: f64,
    direction: Direction,
    ghosts: [Ghost; 4],
    ghostbusting: bool,
    delta: f64,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up, Down, Left, Right
}

pub struct Ghost {
    x: u32,
    y: u32,
    ttr: f64,
}

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

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn tick(&mut self, dt: f64) {
        self.move_pacman(dt);
    }

    fn move_pacman(&mut self, dt: f64) {
        let (tx, ty) = match self.direction {
            Direction::Up => (self.x, self.y - (dt * 4.0)),
            Direction::Down => (self.x, self.y + (dt * 4.0)),
            Direction::Left => (self.x - (dt * 4.0), self.y),
            Direction::Right => (self.x + (dt * 4.0), self.y),
        };
        let (x, y) = (tx.floor() as u32, ty.floor() as u32);
        match self.map.get(x, y) {
            None => (),
            Some(Tile::Wall) => (),
            Some(Tile::NotWall(pu)) => {
                self.x = tx;
                self.y = ty;
                match pu {
                    PU::Empty => (),
                    PU::Dot => {
                        self.map.consume(x, y);
                        self.score += 10;
                    },
                    PU::PowerUp => {
                        self.map.consume(x, y);
                        self.ghostbusting = true;
                        self.score += 100;
                    },
                }
            },
        }
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn player(&self) -> (f64, f64, Direction) {
        (self.x, self.y, self.direction)
    }

    pub fn ghosts(&self) -> &[Ghost] {
        &self.ghosts
    }

    pub fn stats(&self) -> Stats {
        Stats {
            lives: self.lives,
            score: self.score,
            level: self.level,
            ghostbusting: self.ghostbusting,
        }
    }

    pub const fn map_width() -> u32 {
        self::map::Map::width()
    }

    pub const fn map_height() -> u32 {
        self::map::Map::height()
    }
}

impl Default for Pacman {
    fn default() -> Self {
        Pacman {
            map: Map::default(),
            lives: 5,
            score: 0,
            level: 1,
            x: 1.0,
            y: 1.0,
            direction: Direction::Left,
            ghosts: [
                Ghost{ x: 5, y: 5, ttr: 0.0 },
                Ghost{ x: 5, y: 5, ttr: 0.0 },
                Ghost{ x: 5, y: 5, ttr: 0.0 },
                Ghost{ x: 5, y: 5, ttr: 0.0 }
            ],
            ghostbusting: false,
            delta: 0.0,
        }
    }
}
