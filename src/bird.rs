use crate::draw::Drawable;
use crate::utils::rgb_to_color;
use opengl_graphics::GlGraphics;
use graphics::rectangle;
use piston::input::{RenderArgs, UpdateArgs};

use crate::utils;

const BIRD_SIZE: f64 = 30.0;

pub struct Bird {
    color: [f32; 4],
    offset_y: f32,
}

impl Bird {
    pub fn new() -> Self {
        Bird {
            color: rgb_to_color(255, 135, 31, 1.0),
            offset_y: 0.0,
        }
    }

    pub fn render(&mut self, window_width: f64, window_height: f64, con: &graphics::Context, g: &mut opengl_graphics::GlGraphics) {
        let y = (window_height - BIRD_SIZE) / 2.0 + (self.offset_y as f64);
        let x = (window_width - BIRD_SIZE) / 2.0;
        let square = [x, y, BIRD_SIZE, BIRD_SIZE];
        rectangle(self.color, square, con.transform, g)
    }
}

impl Drawable for Bird {
    fn draw(&mut self, con: &graphics::Context, g: &mut opengl_graphics::GlGraphics) {
        
        // let (window_width, window_height) = (args.window_size[0], args.window_size[1]);
        // let y = (window_height - BIRD_SIZE) / 2.0 + (self.offset_y as f64);
        // let x = (window_width - BIRD_SIZE) / 2.0;

        // let square = [x, y, BIRD_SIZE, BIRD_SIZE];
        // // rectangle(self.color, square, c.transform, gl);
        // self.gl.draw(args.viewport(), |c, gl| {
        //     // rectangle(self.color, square, c.transform, gl);
        // });
    }
}
