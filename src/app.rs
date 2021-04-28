use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use piston::*;
use std::process;

use crate::bird::Bird;
use crate::draw::Drawable;
use crate::pipes::Pipes;

use crate::basic::Direction;
use crate::basic::{get_earth_color, get_error_color, get_sky_color};

pub struct App {
    bird: Bird,
    pipes: Pipes,
    game_over: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            bird: Bird::new(),
            pipes: Pipes::new(),
            game_over: false,
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

        gl.draw(args.viewport(), |c, g| {
            clear(get_sky_color(), g);

            //辅助线
            // rectangle(get_earth_color(), [0.0, 195.0, window_width, 1.0], c.transform, g);
            // rectangle(get_error_color(), [0.0, 211.0, window_width, 1.0], c.transform, g);

            rectangle(get_earth_color(), earth_square, c.transform, g);
            self.pipes.draw(&c, g, window_size);
            self.bird.draw(&c, g, window_size);

            let bird_square = self.bird.get_square(window_size);
            if self.pipes.is_hit(bird_square) {
                println!("HIT!!");
                self.game_over = true;
                // process::exit(1);
            }

            if self.game_over {
                rectangle(
                    get_error_color(),
                    [0.0, 0.0, window_width, window_height],
                    c.transform,
                    g,
                );
            }
        });       
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.game_over == false {
            self.pipes.move_forward();
            self.bird.g_move();
        }
    }
}
