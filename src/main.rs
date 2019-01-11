mod pacman;
mod view;
mod controler;

use piston::window::WindowSettings;
use piston::event_loop::{ EventSettings, Events };
use piston::event_loop::EventLoop;
use piston::input::RenderEvent;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use crate::pacman::Pacman;
use crate::view::View;
use crate::controler::Controler;

// TODO: Control the ghosts for Milady

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("pacman-game", [1000, 1000])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut controler = Controler::new(Pacman::default());
    let view = View::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear([0.0; 4], g);
                view.draw(&controler, &c, g);
            })
        } else {
            if controler.event(&e) { break }
        }
    }
}
