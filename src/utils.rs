pub fn rgb_to_color(r: u16, g: u16, b: u16, opacity: f32) -> [f32; 4] {
    [
        ((r as f32) / 255.0) as f32,
        ((g as f32) / 255.0) as f32,
        ((b as f32) / 255.0) as f32,
        opacity,
    ]
}
