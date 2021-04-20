use piston::input::{RenderArgs, UpdateArgs};

pub trait Drawable {
    fn render(&mut self, args: &RenderArgs);
    fn update(&mut self, args: &UpdateArgs);
}

// pub fn draw_rectangle(color:[f32; 4], x: i32, y: i32, width: i32, height: i32, con: &graphics::Context, g: &mut opengl_graphics::GlGraphics) {

//     graphics::rectangle(
//         color,
//         [x, y, width, height],
//         con.transform,
//         g,
//     )
// }