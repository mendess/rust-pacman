use graphics::types::Color;
use graphics::{Context, Graphics};
use crate::controler::Controler;
use crate::pacman::map::Tile;

pub struct View {
    background_color: Color,
    wall_color: Color,
    blinky_color: Color,
    pinky_color: Color,
    inky_color: Color,
    clyde_color: Color,
    frightened_color: Color,
    tile_size: f64,
}

impl View {
    pub fn new() -> Self {
        View {
            background_color: [0.1294, 0.1294, 0.8706, 1.0],
            wall_color: [0.1294, 0.1294, 0.8706, 1.0],
            blinky_color: [1.0, 0.0, 0.0, 1.0],
            pinky_color: [1.0, 0.7216, 1.0, 1.0],
            inky_color: [0.0, 1.0, 1.0, 1.0],
            clyde_color: [1.0, 0.7216, 0.3176, 1.0],
            frightened_color: [0.0039, 0.0902, 1.0, 1.0],
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
            (x as f64) * self.tile_size,
            (y as f64) * self.tile_size,
            self.tile_size,
            self.tile_size
        ];
        Rectangle::new([1.0, 1.0, 0.0, 1.0])
            .draw(pacman, &c.draw_state, c.transform, g);

        let ghosts = controler.get_ghosts();
        let blinky = [
            ghosts[0].x as f64 * self.tile_size,
            ghosts[0].y as f64 * self.tile_size,
            self.tile_size,
            self.tile_size
        ];
        Rectangle::new(self.blinky_color)
            .draw(blinky, &c.draw_state, c.transform, g);

        let pinky = [
            ghosts[1].x as f64 * self.tile_size,
            ghosts[1].y as f64 * self.tile_size,
            self.tile_size,
            self.tile_size
        ];
        Rectangle::new(self.pinky_color)
            .draw(pinky, &c.draw_state, c.transform, g);

        let inky = [
            ghosts[2].x as f64 * self.tile_size,
            ghosts[2].y as f64 * self.tile_size,
            self.tile_size,
            self.tile_size
        ];
        Rectangle::new(self.inky_color)
            .draw(inky, &c.draw_state, c.transform, g);

        let clyde = [
            ghosts[3].x as f64 * self.tile_size,
            ghosts[3].y as f64 * self.tile_size,
            self.tile_size,
            self.tile_size
        ];
        Rectangle::new(self.clyde_color)
            .draw(clyde, &c.draw_state, c.transform, g);
    }
}
