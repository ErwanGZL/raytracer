use crate::{material::Color, math::Vector3D};

use super::Light;

pub struct Directional {
    color: Color,
    intensity: f32,
    direction: Vector3D,
}

impl Directional {
    pub fn new(color: Color, intensity: f32, direction: Vector3D) -> Self {
        Directional {
            color,
            intensity,
            direction,
        }
    }
}

impl Light for Directional {
    fn color(&self) -> Color {
        self.color
    }
    fn intensity(&self) -> f32 {
        self.intensity
    }
    fn direction_from(&self, _point: Vector3D) -> Vector3D {
        -self.direction
    }
}
