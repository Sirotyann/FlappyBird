use piston::input::{RenderArgs, UpdateArgs};

pub trait Drawable {
    fn render(&mut self, args: &RenderArgs);
    fn update(&mut self, args: &UpdateArgs);
}