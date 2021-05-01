extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::*;

mod game;
mod basic;
mod bird;
mod draw;
mod pipes;

use crate::basic::SCREEN_SIZE;
use game::Game;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let (screen_width, screen_height) = SCREEN_SIZE;
    let mut window: Window = WindowSettings::new("Flappy Bird", [screen_width, screen_height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut game = Game::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        let mut gl = GlGraphics::new(opengl);

        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }

        if let Some(args) = e.render_args() {
            game.render(&mut gl, &args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}
