extern crate piston_window;

use piston_window::*;

pub struct App {
    rotation: f64   // Rotation for the square.
}

impl App {
    fn render(&mut self, e: &PistonWindow, args: &RenderArgs) {

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        e.draw_2d(|c, gl| {

            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {

    // Create an Glutin window.
    let window: PistonWindow = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        rotation: 0.0
    };

    for e in window {

        if let Some(r) = e.render_args() {
            app.render(&e,&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}