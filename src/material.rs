pub mod color;
pub use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    color: Color,
    transparency: f64,
}

impl Material {
    pub fn new(color: Color, transparency: f64) -> Material {
        Material {
            color,
            transparency,
        }
    }
    pub fn color(&self) -> Color {
        self.color
    }
}
