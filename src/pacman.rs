mod map;

use self::map::Map;
use self::map::Tile;
use self::map::PU;

pub struct Pacman {
    map: Map,
    lives: u8,
    score: u32,
    level: u32,
    x: u32,
    y: u32,
    direction: Direction,
    ghosts: [Ghost; 4],
    ghostbusting: bool,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up, Down, Left, Right
}

struct Ghost {
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

    pub fn tick(&mut self) {
        self.move_pacman();
    }

    fn move_pacman(&mut self) {
        let (tx, ty) = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        let tile = match self.map.get(tx, ty) {
            None => (),
            Some(Tile::Wall) => (),
            Some(Tile::NotWall(pu)) => {
                self.x = tx;
                self.y = ty;
                match pu {
                    PU::Empty => (),
                    PU::Dot => {
                        self.map.consume(tx, ty);
                        self.score += 10;
                    },
                    PU::PowerUp => {
                        self.map.consume(tx, ty);
                        self.ghostbusting = true;
                        self.score += 100;
                    },
                }
            },
        };
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn player(&self) -> (u32, u32, Direction) {
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
            x: 1,
            y: 1,
            direction: Direction::Left,
            ghosts: [
                Ghost{ x: 5, y: 5, ttr: 0.0 },
                Ghost{ x: 5, y: 5, ttr: 0.0 },
                Ghost{ x: 5, y: 5, ttr: 0.0 },
                Ghost{ x: 5, y: 5, ttr: 0.0 }
            ],
            ghostbusting: false,
        }
    }
}
