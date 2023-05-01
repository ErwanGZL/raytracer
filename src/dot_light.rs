use crate::{material::Color, math::Vector3D};

#[derive(Debug, Clone, Copy)]
pub struct DotLight {
    pub position: Vector3D,
    pub color: Color,
    pub intensity: f32,
}

impl DotLight {
    pub fn new(position: Vector3D, color: Color, intensity: f32) -> Self {
        DotLight {
            position,
            color,
            intensity,
        }
    }
    pub fn position(&self) -> Vector3D {
        self.position
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn intensity(&self) -> f32 {
        self.intensity
    }
}
