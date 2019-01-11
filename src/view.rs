use graphics::types::Color;
use graphics::{Context, Graphics};
use crate::controler::Controler;
use crate::pacman::map::Tile;

pub struct View {
    pub background_color: Color,
    pub wall_color: Color,
    pub tile_size: f64,
}

impl View {
    pub fn new() -> Self {
        View {
            background_color: [0.1294, 0.1294, 0.8706, 1.0],
            wall_color: [0.1294, 0.1294, 0.8706, 1.0],
            tile_size: 25.0,
        }
    }

    pub fn draw<G: Graphics>(&self, controler: &Controler, c: &Context, g: &mut G) {
        use graphics::{ Rectangle };

        let mut x = 0.0;
        let mut y = 0.0;
        for line in controler.get_map().scan_lines() {
            for tile in line.iter() {
                match tile {
                    Tile::Wall =>
                        Rectangle::new(self.wall_color)
                        .draw(
                            [x, y, self.tile_size, self.tile_size],
                            &c.draw_state,
                            c.transform,
                            g),
                    _ => (),
                }
                x += self.tile_size;
            }
            y += self.tile_size;
            x = 0.0;
        }

        let (x, y, _) = controler.get_player();
        let pacman = [
            0.0 + (x as f64) * self.tile_size,
            0.0 + (y as f64) * self.tile_size,
            self.tile_size,
            self.tile_size
        ];
        println!("pacman: {}, {}", pacman[0], pacman[1]);
        Rectangle::new([1.0, 1.0, 0.0, 1.0])
            .draw(pacman, &c.draw_state, c.transform, g);
    }
}
