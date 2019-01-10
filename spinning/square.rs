use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    going_back: bool,
    position: (f64, f64),
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x,y) = (args.width / 2.0 + self.position.0, args.height / 2.0 + self.position.1);

        self.gl.draw(args.viewport(), |c , gl| {
            clear(GREEN, gl);
            let transform = c.transform.trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if !self.going_back {
            self.rotation += 2.0 * args.dt;
        } else {
            self.rotation -= 2.0 * args.dt;
        }
    }

    fn go_back(&mut self) {
        self.going_back = true;
    }

    fn stop_going_back(&mut self) {
        self.going_back = false;
    }

    fn go_up(&mut self) {
        self.position.1 -= 2.0;
    }

    fn go_right(&mut self) {
        self.position.0 += 2.0;
    }

    fn go_left(&mut self) {
        self.position.0 -= 2.0;
    }

    fn go_down(&mut self) {
        self.position.1 += 2.0;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        going_back: false,
        position: (0.0, 0.0),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(p) = e.press_args() {
            use piston::input::keyboard::Key;
            match p {
                Button::Keyboard(Key::Left) => app.go_back(),
                Button::Keyboard(Key::H) => app.go_left(),
                Button::Keyboard(Key::J) => app.go_down(),
                Button::Keyboard(Key::K) => app.go_up(),
                Button::Keyboard(Key::L) => app.go_right(),
                _ => (),
            }
        }

        if let Some(r) = e.release_args() {
            match r {
                Button::Keyboard(keyboard::Key::Left) => app.stop_going_back(),
                _ => (),
            }
        }
    }
}
