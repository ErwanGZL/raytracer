pub mod color;
pub use color::Color;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]

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

    pub fn from_json(data: &Value) -> Material {
        let color = Color::from_json(&data["color"]);
        let transparency = data["specular_coefficient"].as_f64().unwrap();
        return Material::new(color, transparency);
    }
}
