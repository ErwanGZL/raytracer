use crate::{material::Color, math::Vector3D};

use super::Light;

#[derive(Debug, Clone, Copy)]
pub struct Dot {
    pub position: Vector3D,
    pub color: Color,
    pub intensity: f32,
}

impl Dot {
    pub fn new(position: Vector3D, color: Color, intensity: f32) -> Self {
        Dot {
            position,
            color,
            intensity,
        }
    }
}

impl Light for Dot {
    fn color(&self) -> Color {
        self.color
    }
    fn intensity(&self) -> f32 {
        self.intensity
    }
    fn direction_from(&self, point: Vector3D) -> Vector3D {
        (self.position - point).normalize()
    }
}
