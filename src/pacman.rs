mod map;

use self::map::Map;
use self::map::Tile;
use self::map::PU;

pub struct PacManState {
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

pub enum Direction {
    Up, Down, Left, Right
}

struct Ghost {
    x: u32,
    y: u32,
    ttr: f64,
}

impl PacManState {
    pub fn new() -> Self {
        PacManState::default()
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn tick(&mut self) {
        self.move_pacman();
    }

    fn move_pacman(&mut self) {
        let (tx, ty) = match self.direction {
            Direction::Up => (self.x - 1, self.y),
            Direction::Down => (self.x + 1, self.y),
            Direction::Left => (self.x, self.y - 1),
            Direction::Right => (self.x, self.y + 1),
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
}

impl Default for PacManState {
    fn default() -> Self {
        PacManState {
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
