use serde_json::Value;

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
    // passe en dynamique ?
    pub fn from_json(data: &Value) -> Vec<Dot> {
        let mut all = vec![];
        // let type_found =
        let position_found = Vector3D::from_json(&data["position"]);
        let color_found = Color::from_json(&data["color"]);
        let intensity = data["intensity"].as_f64().unwrap() as f32;
        all.push(Dot::new(position_found, color_found, intensity));
        all
    }
}

impl Light for Dot {
    fn position(&self) -> Vector3D {
        self.position
    }
    fn color(&self) -> Color {
        self.color
    }
    fn intensity(&self) -> f32 {
        self.intensity
    }
}
