use super::map::{self, Map};
use super::Direction;

const BLINKY_HOME: (i32, i32) = (map::MAP_WIDTH as i32 - 3, -2);
const PINKY_HOME: (i32, i32) = (2, -2);
const INKY_HOME: (i32, i32) = (map::MAP_WIDTH as i32 - 1, map::MAP_HEIGHT as i32);
const CLYDE_HOME: (i32, i32) = (0, map::MAP_HEIGHT as i32);
const FRIGHTNED_TIMER: u16 = 30;
const GHOST_MODE_TIMER: u16 = 7 * 4;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GhostMode {
    Chase,
    Scatter,
    Frightened,
}

#[derive(Debug, Clone, Copy)]
pub enum Name {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

pub enum Interaction {
    KillPlayer,
    KillGhost(u8),
}

pub struct Ghosts {
    ghosts: [Ghost; 4],
    ghost_mode: GhostMode,
    mode_timer: u16,
    frightened_timer: u16,
    num_scatters: u8,
}

impl Ghosts {
    pub fn new() -> Self {
        Ghosts {
            ghosts: [
                Ghost::new(Name::Blinky),
                Ghost::new(Name::Pinky),
                Ghost::new(Name::Inky),
                Ghost::new(Name::Clyde),
            ],
            ghost_mode: GhostMode::Chase,
            mode_timer: 0,
            frightened_timer: 0,
            num_scatters: 2,
        }
    }

    pub fn get(&self) -> &[Ghost] {
        &self.ghosts
    }

    pub fn ghost_mode(&self) -> GhostMode {
        self.ghost_mode
    }

    pub fn frighten(&mut self) {
        self.ghost_mode = GhostMode::Frightened;
        self.frightened_timer = FRIGHTNED_TIMER;
    }

    pub fn move_ghosts(&mut self, map: &Map, player: (i32, i32, Direction)) {
        let blinky = self.ghosts[0].pos;
        for ghst in self.ghosts.iter_mut() {
            if ghst.house_timer != 0 {
                ghst.house_move(map);
                continue;
            }
            let plr = (player.0, player.1);
            match self.ghost_mode {
                GhostMode::Frightened => ghst.flee(map),
                GhostMode::Chase => {
                    let target = match ghst.name {
                        Name::Blinky => plr,
                        Name::Pinky => calc_pinky_target(player),
                        Name::Inky => calc_inky_target(blinky, player),
                        Name::Clyde => calc_clyde_target(ghst.pos, plr),
                    };
                    ghst.move_to(map, target);
                }
                GhostMode::Scatter => {
                    let target = match ghst.name {
                        Name::Blinky => BLINKY_HOME,
                        Name::Pinky => PINKY_HOME,
                        Name::Inky => INKY_HOME,
                        Name::Clyde => CLYDE_HOME,
                    };
                    ghst.move_to(map, target);
                }
            }
        }
        if self.ghost_mode == GhostMode::Frightened {
            self.frightened_timer = self.frightened_timer.saturating_sub(1);
            if self.frightened_timer == 0 {
                self.ghost_mode = GhostMode::Chase;
            }
        } else {
            self.mode_timer = self.mode_timer.saturating_sub(1);
            if self.mode_timer == 0 {
                self.mode_timer = GHOST_MODE_TIMER;
                self.ghost_mode = if self.ghost_mode == GhostMode::Chase && self.num_scatters > 0 {
                    self.num_scatters -= 1;
                    GhostMode::Scatter
                } else {
                    GhostMode::Chase
                }
            }
        }
    }

    pub fn interact_with_player(&mut self, plr: (i32, i32)) -> Option<Interaction> {
        if self.ghost_mode == GhostMode::Frightened {
            let mut killed = 0;
            for g in self.ghosts.iter_mut() {
                if g.pos == plr || g.last_pos == plr {
                    *g = Ghost::new(g.name);
                    killed += 1;
                }
            }
            if killed == 0 {
                None
            } else {
                Some(Interaction::KillGhost(killed))
            }
        } else {
            if self
                .ghosts
                .iter()
                .any(|g| g.pos == plr || g.last_pos == plr)
            {
                Some(Interaction::KillPlayer)
            } else {
                None
            }
        }
    }

    pub fn reset(&mut self) {
        *self = Ghosts::new();
    }
}

#[derive(Debug)]
pub struct Ghost {
    name: Name,
    pos: (i32, i32),
    last_pos: (i32, i32),
    house_timer: u16,
}

impl Ghost {
    fn new(name: Name) -> Self {
        let start_p = match name {
            Name::Blinky => (15, 15),
            Name::Pinky => (15, 14),
            Name::Inky => (14, 15),
            Name::Clyde => (14, 14),
        };
        Ghost {
            pos: start_p,
            last_pos: (i32::min_value(), i32::min_value()),
            house_timer: match name {
                Name::Blinky => 2,
                Name::Pinky => 10,
                Name::Inky => 20,
                Name::Clyde => 30,
            },
            name,
        }
    }

    pub fn x(&self) -> i32 {
        self.pos.0
    }

    pub fn y(&self) -> i32 {
        self.pos.1
    }

    fn move_to(&mut self, map: &Map, mut target: (i32, i32)) {
        if map.is_house(self.pos.0, self.pos.1) {
            target = (13, 11); // (14, 11)
        }
        let options = self.get_options();
        let decision = options
            .iter()
            .filter(|opt| **opt != self.last_pos)
            .filter(|(x, y)| map.is_house(*x, *y) || !map.is_wall(*x, *y))
            .min_by_key(|(x, y)| (*x - target.0).pow(2) + (*y - target.1).pow(2));
        if let Some(d) = decision {
            self.change_pos(*d);
        }
    }

    fn flee(&mut self, map: &Map) {
        let mut options = self.get_options();
        options.retain(|opt| *opt != self.last_pos);
        use rand::Rng;
        let mut rng = rand::thread_rng();
        while !options.is_empty() {
            let i = rng.gen::<usize>() % options.len();
            let opt = options.swap_remove(i);
            if !map.is_wall(opt.0, opt.1) {
                self.change_pos(opt);
                break;
            }
        }
    }

    fn house_move(&mut self, map: &Map) {
        let mut options = self.get_options();
        options.retain(|opt| *opt != self.last_pos);
        use rand::Rng;
        let mut rng = rand::thread_rng();
        while !options.is_empty() {
            let i = rng.gen::<usize>() % options.len();
            let opt = options.swap_remove(i);
            if map.is_house(opt.0, opt.1) {
                self.change_pos(opt);
                break;
            }
        }
        self.house_timer = self.house_timer.saturating_sub(1);
    }

    fn change_pos(&mut self, to: (i32, i32)) {
        self.last_pos = self.pos;
        self.pos = to;
    }

    fn get_options(&self) -> Vec<(i32, i32)> {
        let wrap = |x| {
            if x < 0 {
                map::MAP_WIDTH as i32 - 1
            } else if x == map::MAP_WIDTH as i32 {
                0
            } else {
                x
            }
        };
        vec![
            (self.pos.0 + 1, self.pos.1),
            (self.pos.0 - 1, self.pos.1),
            (self.pos.0, self.pos.1 + 1),
            (self.pos.0, self.pos.1 - 1),
        ]
        .iter()
        .cloned()
        .map(|(x, y)| (wrap(x), y))
        .collect()
    }
}

fn calc_pinky_target(player: (i32, i32, Direction)) -> (i32, i32) {
    let v = player.2.to_vector();
    let plr = (player.0, player.1);
    (plr.0 + v.0 * 4, plr.1 + v.1 * 4)
}

fn calc_inky_target(blinky: (i32, i32), player: (i32, i32, Direction)) -> (i32, i32) {
    let plr = (player.0, player.1);
    let mid_tgt = {
        let v = player.2.to_vector();
        (plr.0 + v.0 * 2, plr.1 + v.1 * 2)
    };
    let tgt_vec = ((mid_tgt.0 - blinky.0) * 2, (mid_tgt.1 - blinky.1) * 2);
    (blinky.0 + tgt_vec.0, blinky.1 + tgt_vec.1)
}

fn calc_clyde_target(clyde: (i32, i32), plr: (i32, i32)) -> (i32, i32) {
    if (((clyde.0 - plr.0).pow(2) + (clyde.1 - plr.1).pow(2)) as f64).sqrt() < 8.0 {
        CLYDE_HOME
    } else {
        plr
    }
}

// DEBUG VIEWS
#[allow(dead_code)]
impl Ghosts {
    pub fn targets(&self, plr: (i32, i32, Direction)) -> [(i32, i32); 4] {
        match self.ghost_mode {
            GhostMode::Chase => [
                (plr.0, plr.1),
                calc_pinky_target(plr),
                calc_inky_target(self.ghosts[0].pos, plr),
                calc_clyde_target(self.ghosts[3].pos, (plr.0, plr.1)),
            ],
            GhostMode::Scatter => [BLINKY_HOME, PINKY_HOME, INKY_HOME, CLYDE_HOME],
            GhostMode::Frightened => [(300, 300), (300, 300), (300, 300), (300, 300)],
        }
    }
}
