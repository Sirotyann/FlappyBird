use graphics::rectangle;
use rand::{thread_rng, Rng};
use std::collections::LinkedList;

use crate::basic::{
    get_bird_square, get_pipe_color, PIPE_GAP_HEIGHT, PIPE_GAP_WIDTH, PIPE_HEIGHT_MAX,
    PIPE_HEIGHT_MIN, PIPE_WIDTH, SCREEN_SIZE
};
use crate::draw::Drawable;

#[derive(Copy, Clone, Debug)]
pub struct Pipe {
    width: f64,
    x: f64,
    y: f64,
}

pub struct Pipes {
    color: [f32; 4],
    pipes: LinkedList<Pipe>,
    offset_x: f64,
    hittable_pipe: Option<Pipe>,
}

impl Pipe {
    pub fn top_square(&self) -> [f64; 4] {
        [self.x, 0.0, self.width, self.y]
    }

    pub fn bottom_square(&self) -> [f64; 4] {
        let (_, screen_height) = SCREEN_SIZE;
        [self.x, self.y + PIPE_GAP_HEIGHT, self.width, screen_height]
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
            hittable_pipe: None,
        }
    }

    fn generate_pipes() -> LinkedList<Pipe> {
        let (screen_width, _) = SCREEN_SIZE;
        let mut x: f64 = screen_width;
        let mut list = LinkedList::new();
        while x < screen_width * 3.0 {
            let pipe = Pipes::generate_pipe(x);
            x = x + pipe.width + PIPE_GAP_WIDTH;
            list.push_back(pipe);
        }
        list
    }

    fn generate_pipe(x: f64) -> Pipe {
        let mut rng = thread_rng();
        Pipe {
            width: PIPE_WIDTH,
            x: x,
            y: rng.gen_range(PIPE_HEIGHT_MIN..PIPE_HEIGHT_MAX),
        }
    }

    fn move_offscreen_item(&mut self) {
        let mut first = self.pipes.pop_front().unwrap();
        let last = self.pipes.back().unwrap();
        first.x = last.x + last.width + PIPE_GAP_WIDTH;
        self.pipes.push_back(first);
    }

    pub fn move_forward(&mut self) {
        let [bird_x, _bird_y, bird_width, _bird_heigth] = get_bird_square();

        self.hittable_pipe = None;
        for pipe in self.pipes.iter_mut() {
            pipe.move_forward(self.offset_x);

            if pipe.x <= (bird_x + bird_width) && (pipe.x + pipe.width) >= bird_x {
                self.hittable_pipe = Some(*pipe);
            }
        }

        let first = self.pipes.front().unwrap();
        if first.x + first.width < 0.0 {
            self.move_offscreen_item();
        }
    }

    pub fn is_hit(&self, square: [f64; 4]) -> bool {
        let [_x, y, _width, height] = square;
        let (_, screen_height) = SCREEN_SIZE;
        if y > screen_height {
            return true;
        }
        match self.hittable_pipe {
            Some(pipe) => {
                y <= pipe.y || y + height > (pipe.y + PIPE_GAP_HEIGHT)
            }
            None => false,
        }
    }
}

impl Drawable for Pipes {
    fn draw(
        &mut self,
        con: &graphics::Context,
        g: &mut opengl_graphics::GlGraphics,
    ) {
        for pipe in self.pipes.iter_mut() {
            rectangle(self.color, pipe.top_square(), con.transform, g);
            rectangle(self.color, pipe.bottom_square(), con.transform, g);
        }
    }
}
