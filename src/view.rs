use graphics::types::Color;
use graphics::{Context, Graphics};
use crate::controler::Controler;

pub struct View {
    pub background_color: Color,
    pub tile_size: f64,
}

impl View {
    pub fn new() -> Self {
        View {
            background_color: [0.1294, 0.1294, 0.8706, 1.0],
            tile_size: 32.0,
        }
    }

    pub fn draw<G: Graphics>(&self, controler: &Controler, c: &Context, g: &mut G) {
        use graphics::{ Rectangle };
        let board = [
            5.0,
            5.0,
            Controler::map_width()  as f64 * self.tile_size,
            Controler::map_height() as f64 * self.tile_size
        ];
        Rectangle::new_border(self.background_color, 10.0)
            .draw(board, &c.draw_state, c.transform, g);

        let (x, y, _) = controler.get_player();
        let pacman = [
            5.0 + (x as f64) * self.tile_size,
            5.0 + (y as f64) * self.tile_size,
            self.tile_size,
            self.tile_size
        ];

        Rectangle::new([1.0, 1.0, 0.0, 1.0])
            .draw(pacman, &c.draw_state, c.transform, g);
    }
}
