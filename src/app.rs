use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use piston::*;

use crate::bird::Bird;
use crate::draw::Drawable;
use crate::pipes::Pipes;

use crate::basic::{get_sky_color, get_earth_color};
use crate::basic::Direction;

pub struct App {
    distance: u32,
    score: u16,
    bird: Bird,
    pipes: Pipes,
}

impl App {
    pub fn new() -> Self {
        App {
            distance: 0,
            score: 0,
            bird: Bird::new(),
            pipes: Pipes::new(),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            _ => None,
        };
        self.bird.direction(dir);
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let window_size = (args.window_size[0], args.window_size[1]);
        let (window_width, window_height) = window_size;
        let (earth_width, earth_height) = (window_width, window_height / 8.0);
        let earth_square = [
            0.0,
            (window_height - earth_height),
            window_width,
            earth_height,
        ];

        let bird_square = rectangle::square(20.0, (window_height / 2.0), 30.0);
        let mut bird = Bird::new();

        gl.draw(args.viewport(), |c, g| {
            clear(get_sky_color(), g);
            rectangle(get_earth_color(), earth_square, c.transform, g);
            self.pipes.draw(&c, g, window_size);
            self.bird.draw(&c, g, window_size);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // println!("App Update");
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }
}
