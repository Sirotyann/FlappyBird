use graphics::rectangle;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use rand::{thread_rng, Rng};
use std::collections::LinkedList;

use crate::basic::{
    get_pipe_color, Direction, PIPE_GAP_HEIGHT, PIPE_GAP_WIDTH, PIPE_HEIGHT_MAX, PIPE_HEIGHT_MIN,
    PIPE_WIDTH_MAX, PIPE_WIDTH_MIN, SCREEN_HEIGHT, SCREEN_WIDTH
};
use crate::draw::Drawable;

pub struct Pipe {
    width: f64,
    x: f64,
    y: f64,
}

pub struct Pipes {
    color: [f32; 4],
    pipes: LinkedList<Pipe>,
    offset_x: f64,
}

impl Pipe {
    pub fn top_square(&self) -> [f64; 4] {
        [self.x, 0.0, self.width, self.y]
    }

    pub fn bottom_square(&self) -> [f64; 4] {
        [self.x, self.y + PIPE_GAP_HEIGHT, self.width, SCREEN_HEIGHT]
    }
}

impl Pipes {
    pub fn new() -> Self {
        Pipes {
            color: get_pipe_color(),
            pipes: Pipes::generate_pipes(),
            offset_x: 0.0,
        }
    }

    fn generate_pipes() -> LinkedList<Pipe> {
        let mut rng = thread_rng();
        let mut x: f64 = 0.0;
        let mut list = LinkedList::new();
        while x < SCREEN_WIDTH * 2.0 {
            let pipe = Pipes::generate_pipe(x);
            x = x  + pipe.width + PIPE_GAP_WIDTH;
            list.push_back(pipe);
        }
        list
    }

    fn generate_pipe(x: f64) -> Pipe {
        let mut rng = thread_rng();
        // Pipe {
        //     width: rng.gen_range((PIPE_WIDTH_MIN..PIPE_WIDTH_MAX)),
        //     x: x,
        //     y: rng.gen_range((PIPE_HEIGHT_MIN..PIPE_HEIGHT_MAX)),
        // }
        Pipe {
            width: PIPE_WIDTH_MIN,
            x: x,
            y: rng.gen_range((PIPE_HEIGHT_MIN..PIPE_HEIGHT_MAX)),
        }
    }
}

impl Drawable for Pipes {
    fn draw(
        &mut self,
        con: &graphics::Context,
        g: &mut opengl_graphics::GlGraphics,
        window_size: (f64, f64),
    ) {
        let (window_width, window_height) = window_size;

        let mut pipe_iter = self.pipes.iter();
        loop {
            let pipe = pipe_iter.next();
            match pipe {
                Some(p) => {
                    // println!("top:    {:?}", p.top_square());
                    // println!("bottom: {:?}", p.bottom_square());
                    rectangle(self.color, p.top_square(), con.transform, g);
                    rectangle(self.color, p.bottom_square(), con.transform, g);
                }
                None => {
                    break;
                }
            }
        }
    }
}
