pub mod map;
pub mod ghost;

use self::map::Map;
use self::map::Tile;
use self::map::PU;

use self::ghost::{Ghost, GhostMode};

const START_POS :(i32, i32) = (14, 23);
const FRIGHTNED_TIMER :u16 = 30;

pub struct Pacman {
    map: Map,
    // lives: u8,
    score: u32,
    // level: u32,
    x: i32,
    y: i32,
    direction: Direction,
    direction_intent: Direction,
    ghosts: [Ghost; 4],
    ghost_mode: GhostMode,
    ghost_mode_timer: u16,
    ticks: u32,
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

    pub fn tick(&mut self) {
        self.ticks += 1;
        self.move_pacman();
        self.move_timers();
        if self.player_ghost_overlap() {
            self.x = START_POS.0;
            self.y = START_POS.1;
        }
    }

    fn move_pacman(&mut self) {
        if self.can_turn() {
            self.direction = self.direction_intent;
        }
        let (x, y) = match self.direction {
            Direction::Up    => (self.x, self.y - 1),
            Direction::Down  => (self.x, self.y + 1),
            Direction::Left  => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        match self.map.get(x, y) {
            None => if x == -1 {
                self.x = Map::map_width() as i32 - 1;
            } else if x == Map::map_width() as i32 {
                self.x = 0;
            },
            Some(Tile::Wall) => (),
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
                        self.ghost_mode_timer = FRIGHTNED_TIMER;
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

    pub fn ghost_mode(&self) -> GhostMode {
        self.ghost_mode
    }

    pub fn move_timers(&mut self) {
        self.ghost_mode_timer = self.ghost_mode_timer.saturating_sub(1);
        if self.ghost_mode_timer == 0 {
            self.ghost_mode = GhostMode::Chase;
        }
    }

    pub fn player_ghost_overlap(&self) -> bool {
        self.ghost_mode != GhostMode::Frightened
            && self.ghosts.iter()
            .any(|g| g.x as i32 == self.x && g.y as i32 == self.y)
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
            // lives: 5,
            score: 0,
            // level: 1,
            x: START_POS.0,
            y: START_POS.1,
            direction: Direction::Left,
            direction_intent: Direction::Left,
            ghosts: [
                Ghost{ x: 15, y: 15, ttr: 0.0 }, // blinky
                Ghost{ x: 5, y: 5, ttr: 0.0 }, // pinky
                Ghost{ x: 5, y: 5, ttr: 0.0 }, // inky
                Ghost{ x: 5, y: 5, ttr: 0.0 }  // clyde
            ],
            ghost_mode: GhostMode::Scatter,
            ghost_mode_timer: 0,
            ticks: 0,
        }
    }
}
