use crate::{material::Color, math::Vector3D};

use super::Light;

pub struct Ambiant {
    color: Color,
    intensity: f32,
}

impl Ambiant {
    pub fn new(color: Color, intensity: f32) -> Self {
        Ambiant { color, intensity }
    }
}

impl Light for Ambiant {
    fn color(&self) -> Color {
        self.color
    }
    fn intensity(&self) -> f32 {
        self.intensity
    }
    fn direction_from(&self, _point: Vector3D) -> Vector3D {
        Vector3D::new(0.0, 0.0, 0.0)
    }
}
