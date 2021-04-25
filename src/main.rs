extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::*;

mod app;
mod basic;
mod bird;
mod draw;
mod pipes;

// use crate::bird::Bird;
use crate::basic::SCREEN_HEIGHT;
use crate::basic::SCREEN_WIDTH;
use app::App;
use draw::Drawable;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Flappy Bird", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        let mut gl = GlGraphics::new(opengl);

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.key_pressed(key);
        }

        if let Some(args) = e.render_args() {
            app.render(&mut gl, &args);
        }

        if let Some(args) = e.update_args() {
            // println!("{:}?", args);
            app.update(&args);
        }
    }
}
