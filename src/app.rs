use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use piston::*;
// use std::ops::Div;


use crate::bird::Bird;
use crate::draw::Drawable;
use crate::utils;

pub struct App {
    // pub gl: GlGraphics, // OpenGL drawing backend.
    distance: u32,
    score: u16,
    // bird: Bird,
}

impl App {
    pub fn new() -> Self {
        App {
            // gl: gl,
            distance: 0,
            score: 0,
            // bird: Bird::new(),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some("Up"),
            Key::Down => Some("Down"),
            _ => None,
        };

        println!("{:?} key", dir);
    }
    
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let green = utils::rgb_to_color(15, 125, 37, 1.0);
        let blue = utils::rgb_to_color(135, 243, 255, 1.0);
        let yellow = utils::rgb_to_color(255, 222, 5, 1.0);

        let (window_width, window_height) = (args.window_size[0], args.window_size[1]);
        let (earth_width, earth_height) = (window_width, window_height / 4.0);
        let earth_square = [
            0.0,
            (window_height - earth_height),
            window_width,
            earth_height,
        ];

        let bird_square = rectangle::square(20.0, (window_height / 2.0), 30.0);
        let mut bird = Bird::new();

        gl.draw(args.viewport(), |c, g| {
            // Clear the screen.
            clear(blue, g);

            // let transform = c
            //     .transform
            //     .trans(x, y)
            //     .rot_rad(rotation)
            //     .trans(-25.0, -25.0);

            rectangle(green, earth_square, c.transform, g);

            // Draw a box rotating around the middle of the screen.
            // rectangle(yellow, bird_square, c.transform, g);

            bird.render(window_width, window_height, &c, g);
            
        });

       
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }
}

