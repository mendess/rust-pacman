
const MAP_WIDTH :u32 = 28;
const MAP_HEIGHT :u32 = 29;

pub struct Map {
    tiles: [Tile; (MAP_WIDTH*MAP_HEIGHT) as usize],
}

#[derive(Clone, Copy)]
pub enum Tile {
    Wall,
    NotWall(PU),
}

#[derive(Clone, Copy)]
pub enum PU {
    Dot,
    PowerUp,
    Empty,
}

impl Map {
    pub fn new() -> Self {
        Map::default()
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Tile> {
        if x * y < MAP_WIDTH * MAP_HEIGHT {
            Some(self.tiles[(MAP_WIDTH * x + y) as usize])
        } else {
            None
        }
    }

    fn set(&mut self, x: u32, y: u32, tile: Tile) {
        self.tiles[(MAP_WIDTH * x + y) as usize] = tile;
    }

    pub fn consume(&mut self, x: u32, y: u32) {
        self.set(x, y, Tile::NotWall(PU::Empty))
    }

    pub const fn width() -> u32 {
        MAP_WIDTH
    }

    pub const fn height() -> u32 {
        MAP_HEIGHT
    }
}

impl Default for Map {
    fn default() -> Self {
        let mut m = Map {
            tiles: [Tile::NotWall(PU::Empty); (MAP_WIDTH * MAP_HEIGHT) as usize],
        };
        for i in 0..28 {
            m.set(0, i, Tile::Wall);
            m.set(24, i, Tile::Wall);
        }
        for i in 1..24 {
            m.set(i, 0, Tile::Wall);
            m.set(i, 27, Tile::Wall);
        }
        m
    }
}
