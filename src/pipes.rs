use graphics::rectangle;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use rand::{thread_rng, Rng};
use std::collections::LinkedList;

use crate::basic::{
    get_pipe_color, Direction, PIPE_GAP_HEIGHT, PIPE_GAP_WIDTH, PIPE_HEIGHT_MAX, PIPE_HEIGHT_MIN,
    PIPE_WIDTH_MAX, PIPE_WIDTH_MIN, SCREEN_HEIGHT, SCREEN_WIDTH,
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

    pub fn move_forward(&mut self, offset: f64) {
        self.x = self.x - offset;
    }
}

impl Pipes {
    pub fn new() -> Self {
        Pipes {
            color: get_pipe_color(),
            pipes: Pipes::generate_pipes(),
            offset_x: 8.0,
        }
    }

    fn generate_pipes() -> LinkedList<Pipe> {
        // let mut rng = thread_rng();
        let mut x: f64 = SCREEN_WIDTH;
        let mut list = LinkedList::new();
        while x < SCREEN_WIDTH * 3.0 {
            let pipe = Pipes::generate_pipe(x);
            x = x + pipe.width + PIPE_GAP_WIDTH;
            list.push_back(pipe);
        }
        list
    }

    fn generate_pipe(x: f64) -> Pipe {
        let mut rng = thread_rng();
        Pipe {
            width: PIPE_WIDTH_MIN,
            x: x,
            y: rng.gen_range((PIPE_HEIGHT_MIN..PIPE_HEIGHT_MAX)),
        }
    }

    fn move_offscreen_item(&mut self) {
        let mut first = self.pipes.pop_front().unwrap();
        let last = self.pipes.back().unwrap();
        first.x = last.x + last.width + PIPE_GAP_WIDTH;
        self.pipes.push_back(first);
    }

    pub fn move_forward(&mut self) {
        for pipe in self.pipes.iter_mut() {
            pipe.move_forward(self.offset_x);
        }
        let first = self.pipes.front();
        match first {
            Some(p) => {
                if p.x + p.width < 0.0 {
                    self.move_offscreen_item();
                }
            }
            None => {}
        }
    }

    pub fn is_hit(&self, square: [f64; 4]) -> bool {
        let [x, y, width, height] = square;
        let square_edge_x = x + width;
        let square_edge_y = y + height;
        for pipe in self.pipes.iter() {
            if (pipe.x + pipe.width) < x {
                continue;
            }
            if pipe.x > square_edge_x {
                break;
            }
            if y < pipe.y || square_edge_y > (pipe.y + PIPE_GAP_HEIGHT) {
                return true;
            }
        }
        false
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
        for pipe in self.pipes.iter_mut() {
            rectangle(self.color, pipe.top_square(), con.transform, g);
            rectangle(self.color, pipe.bottom_square(), con.transform, g);
        }
    }
}
