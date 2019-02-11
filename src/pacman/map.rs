const MAP_WIDTH :usize = 28;
const MAP_HEIGHT :usize = 31;

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

    pub fn get(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || x >= MAP_WIDTH as i32 {
            None
        } else if y < 0 || y >= MAP_HEIGHT as i32 {
            None
        } else {
            Some(self.tiles[MAP_WIDTH * y as usize + x as usize])
        }
    }

    fn set(&mut self, x: u32, y: u32, tile: Tile) {
        let (x, y) = (x as usize, y as usize);
        self.tiles[MAP_WIDTH * y + x] = tile;
    }

    pub fn consume(&mut self, x: i32, y: i32) -> i32 {
        let score = match self.get(x, y) {
            Some(Tile::NotWall(PU::Dot)) => 10,
            Some(Tile::NotWall(PU::PowerUp)) => 100,
            _ => 0,
        };
        self.set(x as u32, y as u32, Tile::NotWall(PU::Empty));
        score
    }

    pub fn scan_lines(&self) -> ScanLine {
        ScanLine {
            map: &self,
            line: 0,
        }
    }

    pub const fn map_width() -> usize {
        MAP_WIDTH
    }
}

impl Default for Map {
    fn default() -> Self {
        let map_str = [
            "############################",
            "#............##............#",
            "#.####.#####.##.#####.####.#",
            "#X####.#####.##.#####.####X#",
            "#.####.#####.##.#####.####.#",
            "#..........................#",
            "#.####.##.########.##.####.#",
            "#.####.##.########.##.####.#",
            "#......##....##....##......#",
            "######.##### ## #####.######",
            "######.##### ## #####.######",
            "######.##          ##.######",
            "######.## ###  ### ##.######",
            "######.## #      # ##.######",
            "      .   #      #   .      ",
            "######.## #      # ##.######",
            "######.## ######## ##.######",
            "######.##          ##.######",
            "######.## ######## ##.######",
            "######.## ######## ##.######",
            "#............##............#",
            "#.####.#####.##.#####.####.#",
            "#.####.#####.##.#####.####.#",
            "#X..##................##..X#",
            "###.##.##.########.##.##.###",
            "###.##.##.########.##.##.###",
            "#......##....##....##......#",
            "#.##########.##.##########.#",
            "#.##########.##.##########.#",
            "#..........................#",
            "############################"];

        let map :Vec<Tile> = map_str.iter().flat_map(|x| x.chars())
            .filter_map(|c| {
                match c {
                    '#' => Some(Tile::Wall),
                    '.' => Some(Tile::NotWall(PU::Dot)),
                    ' ' => Some(Tile::NotWall(PU::Empty)),
                    'X' => Some(Tile::NotWall(PU::PowerUp)),
                    _ => None,
                }
            }).collect();
        let mut m = [Tile::NotWall(PU::Empty); MAP_WIDTH * MAP_HEIGHT];
        for i in 0..map.len() {
            m[i] = map[i];
        }
        Map {
            tiles: m
        }
    }
}

pub struct ScanLine<'a> {
    map: &'a Map,
    line: usize,
}


impl<'a> Iterator for ScanLine<'a> {
    type Item = &'a [Tile];

    fn next(&mut self) -> Option<&'a [Tile]> {
        let line_start = self.line * MAP_WIDTH;
        self.line += 1;
        if line_start >= self.map.tiles.len() {
            None
        } else {
            Some(&self.map.tiles[line_start..(line_start + MAP_WIDTH)])
        }
    }
}
