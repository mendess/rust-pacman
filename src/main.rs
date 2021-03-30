mod controler;
mod pacman;
mod view;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, ResizeEvent};
use piston::window::WindowSettings;

use crate::controler::Controler;
use crate::pacman::Pacman;
use crate::view::View;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("pacman-game", [750, 750])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut controler = Controler::new(Pacman::new());
    let mut view = View::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear([0.0; 4], g);
                view.draw(&controler, &c, g);
            })
        } else if let Some(r) = e.resize_args() {
            view.resize(r.window_size[0], r.window_size[1]);
        } else {
            if controler.event(&e) {
                break;
            }
        }
    }
}
