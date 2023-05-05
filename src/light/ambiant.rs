use crate::material::Color;

pub struct Ambiant {
    color: Color,
    intensity: f32,
}

impl Ambiant {
    pub fn new(color: Color, intensity: f32) -> Self {
        Ambiant { color, intensity }
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn intensity(&self) -> f32 {
        self.intensity
    }
}
