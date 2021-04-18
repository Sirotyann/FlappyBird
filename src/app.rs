use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
// use std::ops::Div;

use crate::draw::Drawable;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    distance: u32,
    score: u16,
}

impl App {
    pub fn instance(gl: GlGraphics) -> App {
        App {
            gl: gl,
            distance: 0,
            score: 0,
        }
    }
}

fn rgb_to_color(r: u16, g: u16, b: u16, opacity: f32) -> [f32; 4] {
    [
        (r / 255) as f32,
        (g / 255) as f32,
        (b / 255) as f32,
        opacity,
    ]
}

impl Drawable for App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // let some_color = rgb_to_color(135, 243, 255, 1.0);
       
        // for color in &some_color {
        //     print!("{:.32}", color)
        // }

        let blue = [0.1, 0.5, 1.0, 1.0];
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = 0.0;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(blue, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(red, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }
}
