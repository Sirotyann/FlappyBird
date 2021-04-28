use graphics::rectangle;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use crate::basic::rgb_to_color;
use crate::basic::Direction;
use crate::basic::BIRD_SIZE;
use crate::draw::Drawable;

pub struct Bird {
    color: [f32; 4],
    offset_y: f64,
    g_speed: f64,
    up_speed: f64,
}

const GA: f64 = 8.0;

impl Bird {
    pub fn new() -> Self {
        Bird {
            color: rgb_to_color(255, 135, 31, 1.0),
            offset_y: 0.0,
            g_speed: 0.0,
            up_speed: 0.0,
        }
    }

    pub fn direction(&mut self, direction: Option<Direction>) {
        match direction {
            Some(Direction::Up) => {
                // self.up_speed = 2.5 * GA;
                self.offset_y -= 50.0;
                self.g_speed = 0.0;
                // println!("Move up  {}", self.g_speed);
            }
            Some(Direction::Down) => {
                // println!("Move down");
                // self.offset_y = 50.0;
                self.g_speed += (GA / 4.0);
            }
            _ => {}
        }
    }

    pub fn get_square(&self, window_size: (f64, f64)) -> [f64; 4] {
        let (window_width, window_height) = window_size;
        let y = (window_height - BIRD_SIZE) / 2.0 + self.offset_y as f64;
        let x = (window_width - BIRD_SIZE) / 2.0;
        [x, y, BIRD_SIZE, BIRD_SIZE]
    }

    pub fn g_move(&mut self) {
        self.offset_y += self.g_speed;
        self.g_speed = self.g_speed + GA - self.up_speed;

        self.up_speed = if self.up_speed < 1.0 {
            0.0
        } else {
            self.up_speed / 2.0
        };
        // if self.offset_y != 0.0 {
        //     self.offset_y = if self.offset_y.abs() < 1.0 {
        //         0.0
        //     } else {
        //         self.offset_y / 2.0
        //     };
        // }
    }
}

impl Drawable for Bird {
    fn draw(
        &mut self,
        con: &graphics::Context,
        g: &mut opengl_graphics::GlGraphics,
        window_size: (f64, f64),
    ) {
        let square = self.get_square(window_size);
        // println!("Draw bird {:?}", square);
        rectangle(self.color, square, con.transform, g)
    }
}
