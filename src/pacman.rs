pub mod ghost;
pub mod map;

use self::map::Map;
use self::map::Tile;
use self::map::PU;

use self::ghost::{Ghost, GhostMode, Ghosts, Interaction};

const START_POS: (i32, i32) = (14, 23);
const SCORE_PELLET: u32 = 10;
const SCORE_PU: u32 = 50;
const SCORE_GHOST: u32 = 200;

pub struct Pacman {
    map: Map,
    lives: u8,
    score: u32,
    level: usize,
    x: i32,
    y: i32,
    direction: Direction,
    direction_intent: Direction,
    ghosts: Ghosts,
    ticks: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vector(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

pub struct Stats {
    pub lives: u8,
    pub score: u32,
    pub level: usize,
}

impl Pacman {
    pub fn new() -> Self {
        Pacman::default()
    }

    pub fn set_direction_intent(&mut self, direction: Direction) {
        if self.lives == 0 {
            return;
        }
        self.direction_intent = direction;
        if self.can_turn() {
            self.direction = self.direction_intent;
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        if self.map.pellets() == 0 {
            self.advance_level();
            return;
        }
        if self.lives == 0 {
            return;
        }
        self.move_pacman();
        self.move_ghosts();
        match self.ghosts.interact_with_player((self.x, self.y)) {
            Some(Interaction::KillPlayer) => {
                self.x = START_POS.0;
                self.y = START_POS.1;
                self.lives -= 1;
            }
            Some(Interaction::KillGhost(n)) => {
                self.score += SCORE_GHOST * n as u32;
            }
            None => (),
        }
    }

    fn move_pacman(&mut self) {
        if self.can_turn() {
            self.direction = self.direction_intent;
        }
        let (x, y) = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        match self.map.get(x, y) {
            None => {
                if x == -1 {
                    self.x = map::MAP_WIDTH as i32 - 1;
                } else if x == map::MAP_WIDTH as i32 {
                    self.x = 0;
                }
            }
            Some(Tile::NotWall(pu)) => {
                self.x = x;
                self.y = y;
                match pu {
                    PU::Empty => (),
                    PU::Dot => {
                        self.map.consume(x, y);
                        self.score += SCORE_PELLET;
                    }
                    PU::PowerUp => {
                        self.map.consume(x, y);
                        self.ghosts.frighten();
                        self.score += SCORE_PU;
                    }
                }
            }
            _ => (),
        }
    }

    fn move_ghosts(&mut self) {
        self.ghosts
            .move_ghosts(&self.map, (self.x, self.y, self.direction));
    }

    fn can_turn(&self) -> bool {
        let (x, y) = match self.direction_intent {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        match self.map.get(x, y) {
            None => false,
            Some(Tile::Wall) => false,
            _ => true,
        }
    }

    fn advance_level(&mut self) {
        self.level += 1;
        self.x = START_POS.0;
        self.y = START_POS.1;
        self.ghosts.reset();
        self.map.reset();
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn player(&self) -> (i32, i32, Direction) {
        (self.x, self.y, self.direction_intent)
    }

    pub fn ghosts(&self) -> &[Ghost] {
        &self.ghosts.get()
    }

    pub fn ghost_mode(&self) -> GhostMode {
        self.ghosts.ghost_mode()
    }

    pub fn stats(&self) -> Stats {
        Stats {
            lives: self.lives,
            score: self.score,
            level: self.level,
        }
    }
}

impl Default for Pacman {
    fn default() -> Self {
        Pacman {
            map: Map::new(),
            lives: 5,
            score: 0,
            level: 1,
            x: START_POS.0,
            y: START_POS.1,
            direction: Direction::Left,
            direction_intent: Direction::Left,
            ghosts: Ghosts::new(),
            ticks: 0,
        }
    }
}

// // DEBUG VIEWS
#[allow(dead_code)]
impl Pacman {
    pub fn ghost_targets(&self) -> [(i32, i32); 4] {
        self.ghosts.targets((self.x, self.y, self.direction))
    }

    pub fn level_up(&mut self) {
        self.map.remove_all_pellets();
    }
}
