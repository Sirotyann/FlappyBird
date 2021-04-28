#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn rgb_to_color(r: u16, g: u16, b: u16, opacity: f32) -> [f32; 4] {
    [
        ((r as f32) / 255.0) as f32,
        ((g as f32) / 255.0) as f32,
        ((b as f32) / 255.0) as f32,
        opacity,
    ]
}

pub fn rgbs_to_color(rgbs: [f32; 4]) -> [f32; 4] {
    [rgbs[0] / 255.0, rgbs[1] / 255.0, rgbs[2] / 255.0, rgbs[3]]
}

pub const SCREEN_WIDTH: f64 = 512.0;
pub const SCREEN_HEIGHT: f64 = 512.0;

pub const BIRD_SIZE: f64 = 30.0;

pub const PIPE_WIDTH_MAX: f64 = 100.0;
pub const PIPE_WIDTH_MIN: f64 = 80.0;
pub const PIPE_HEIGHT_MAX: f64 = 252.0;
pub const PIPE_HEIGHT_MIN: f64 = 140.0;
pub const PIPE_GAP_HEIGHT: f64 = 120.0;
pub const PIPE_GAP_WIDTH: f64 = 100.0;

pub const PIPE_COLOR: [f32; 4] = [69.0, 130.0, 0.0, 1.0];
pub const EARTH_COLOR: [f32; 4] = [222.0, 216.0, 139.0, 1.0];
pub const SKY_COLOR: [f32; 4] = [78.0, 199.0, 207.0, 1.0];
pub const ERROR_COLOR: [f32; 4] = [153.0, 0.0, 0.0, 0.7];

pub fn get_pipe_color() -> [f32; 4] {
    rgbs_to_color(PIPE_COLOR)
}

pub fn get_earth_color() -> [f32; 4] {
    rgbs_to_color(EARTH_COLOR)
}

pub fn get_sky_color() -> [f32; 4] {
    rgbs_to_color(SKY_COLOR)
}

pub fn get_error_color() -> [f32; 4] {
    rgbs_to_color(ERROR_COLOR)
}

pub fn get_bird_square() -> [f64; 4] {
    [
        (SCREEN_WIDTH - BIRD_SIZE) / 2.0,
        (SCREEN_HEIGHT - BIRD_SIZE) / 2.0,
        BIRD_SIZE,
        BIRD_SIZE,
    ]
}
