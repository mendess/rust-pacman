use crate::controler::Controler;
use crate::pacman::map::{Tile, MAP_HEIGHT, MAP_WIDTH, PU};
use crate::pacman::Direction;
use graphics::{circle_arc::CircleArc, image::Image, rectangle::Rectangle, types::Color, Context};
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use std::path::Path;
use texture::TextureSettings;

pub struct View {
    wall_color: Color,
    ghost_textures: [Texture; 4],
    frightened: Texture,
    numbers: Vec<Texture>,
    pacmans: [Texture; 4],
    fruits: [Texture; 20],
    dot_color: Color,
    tile_size: f64,
    x_offset: f64,
    y_offset: f64,
}

fn load_image(name: &str) -> Texture {
    Texture::from_path(
        Path::new(&format!("images/{}.png", name)),
        &TextureSettings::new(),
    )
    .expect(&format!("Failed to load: {}", name))
}

impl View {
    pub fn new() -> Self {
        let ghost_textures = {
            let mut textures = Vec::with_capacity(4);
            for name in ["blinky", "pinky", "inky", "clyde"].iter() {
                textures.push(load_image(name));
            }
            [
                textures.remove(0),
                textures.remove(0),
                textures.remove(0),
                textures.remove(0),
            ]
        };
        let frightened = load_image("frightened");
        let numbers = (0..10)
            .map(|i| load_image(&i.to_string()))
            .collect::<Vec<_>>();
        let pacmans = [
            load_image("pacman_up"),
            load_image("pacman_right"),
            load_image("pacman_down"),
            load_image("pacman_left"),
        ];
        let fruits = [
            load_image("cherry"),
            load_image("strawberry"),
            load_image("orange"),
            load_image("orange"),
            load_image("apple"),
            load_image("apple"),
            load_image("mellon"),
            load_image("mellon"),
            load_image("flower"),
            load_image("flower"),
            load_image("bell"),
            load_image("bell"),
            load_image("key"),
            load_image("key"),
            load_image("key"),
            load_image("key"),
            load_image("key"),
            load_image("key"),
            load_image("key"),
            load_image("key"),
        ];
        View {
            wall_color: [0.1294, 0.1294, 0.8706, 1.0],
            ghost_textures,
            frightened,
            numbers,
            pacmans,
            fruits,
            dot_color: [1.0, 1.0, 1.0, 1.0],
            tile_size: 20.0,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }

    pub fn resize(&mut self, x: f64, y: f64) {
        self.tile_size = y / (MAP_HEIGHT + 6) as f64;
        let blankspace = x - (MAP_WIDTH as f64 * self.tile_size);
        self.x_offset = blankspace / 2.0;
        self.y_offset = self.tile_size * 2.0;
    }

    pub fn draw(&self, controler: &Controler, c: &Context, g: &mut GlGraphics) {
        let offset = |mut a: [f64; 4]| {
            a[0] += self.x_offset;
            a[1] += self.y_offset;
            a
        };
        let mut x = 0.0;
        let mut y = 0.0;
        for line in controler.get_map().scan_lines() {
            for tile in line.iter() {
                match tile {
                    Tile::Wall => {
                        let sqr = offset([
                            x + self.tile_size / 4.0,
                            y + self.tile_size / 4.0,
                            self.tile_size / 2.0,
                            self.tile_size / 2.0,
                        ]);
                        Rectangle::new(self.wall_color).draw(sqr, &c.draw_state, c.transform, g);
                    }
                    Tile::NotWall(PU::Dot) => {
                        let sqr = offset([
                            x + self.tile_size * (5.0 / 12.0),
                            y + self.tile_size * (5.0 / 12.0),
                            self.tile_size / 6.0,
                            self.tile_size / 6.0,
                        ]);
                        Rectangle::new(self.dot_color).draw(sqr, &c.draw_state, c.transform, g);
                    }
                    Tile::NotWall(PU::PowerUp) => {
                        let sqr = offset([
                            x + self.tile_size * (3.0 / 8.0),
                            y + self.tile_size * (3.0 / 8.0),
                            self.tile_size / 4.0,
                            self.tile_size / 4.0,
                        ]);
                        CircleArc::new(self.dot_color, self.tile_size / 4.0, 0.0, 2.0 * 3.14).draw(
                            sqr,
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                    }
                    _ => (),
                }
                x += self.tile_size;
            }
            y += self.tile_size;
            x = 0.0;
        }

        let stats = controler.get_stats();
        {
            // Stats
            let mut sc = stats.score;
            let mut i = -1;
            while sc > 0 {
                sc /= 10;
                i += 1;
            }
            sc = stats.score;
            while sc > 0 {
                let d = sc % 10;
                let sq = offset([
                    (MAP_WIDTH / 2) as f64 * self.tile_size + i as f64 * self.tile_size * 1.702,
                    self.tile_size * -2.0,
                    self.tile_size,
                    self.tile_size * 1.702,
                ]);
                Image::new().rect(sq).draw(
                    &self.numbers[d as usize],
                    &c.draw_state,
                    c.transform,
                    g,
                );
                sc = sc / 10;
                i -= 1;
            }

            for i in 0..stats.lives {
                Image::new()
                    .rect(offset(
                        self.entity_sq(i as i32 * 2, (MAP_HEIGHT + 1) as i32),
                    ))
                    .draw(&self.pacmans[3], &c.draw_state, c.transform, g);
            }

            let cap_at_20 = |i| if i > 20 { 20 } else { i };
            let cap_at_13 = |i| if i > 13 { 13 } else { i };
            self.fruits[cap_at_13(stats.level.saturating_sub(7))..cap_at_20(stats.level)]
                .iter()
                .enumerate()
                .for_each(|(i, t)| {
                    Image::new()
                        .rect(offset(self.entity_sq(
                            (MAP_WIDTH - i - 1) as i32,
                            (MAP_HEIGHT + 1) as i32,
                        )))
                        .draw(t, &c.draw_state, c.transform, g)
                });
        }

        if stats.lives > 0 {
            // pacman
            let (x, y, d) = controler.get_player();
            let pac_texture = match d {
                Direction::Up => &self.pacmans[0],
                Direction::Right => &self.pacmans[1],
                Direction::Down => &self.pacmans[2],
                Direction::Left => &self.pacmans[3],
            };
            Image::new().rect(offset(self.entity_sq(x, y))).draw(
                pac_texture,
                &c.draw_state,
                c.transform,
                g,
            );
        }

        let pick_color = |c| {
            if controler.frightened() {
                &self.frightened
            } else {
                c
            }
        };

        for (i, ghost) in controler.get_ghosts().iter().enumerate() {
            Image::new()
                .rect(offset(self.entity_sq(ghost.x(), ghost.y())))
                .draw(
                    pick_color(&self.ghost_textures[i]),
                    &c.draw_state,
                    c.transform,
                    g,
                );
        }

        // DEBUG
        // for (i, sqr) in controler.ghost_targets().iter().enumerate() {
        //     let ghost_colors = [
        //         [1.0, 0.0, 0.0, 1.0],
        //         [1.0, 0.7216, 1.0, 1.0],
        //         [0.0, 1.0, 1.0, 1.0],
        //         [1.0, 0.7216, 0.3176, 1.0],
        //     ];
        //         Rectangle::new_border(ghost_colors[i], 1.0)
        //             .draw(offset(self.entity_sq(sqr.0, sqr.1)), &c.draw_state, c.transform, g);
        // }
    }

    fn entity_sq(&self, x: i32, y: i32) -> [f64; 4] {
        [
            x as f64 * self.tile_size,
            y as f64 * self.tile_size,
            self.tile_size,
            self.tile_size,
        ]
    }
}
