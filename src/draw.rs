pub trait Drawable {
    fn draw(
        &mut self,
        con: &graphics::Context,
        g: &mut opengl_graphics::GlGraphics,
    );
}