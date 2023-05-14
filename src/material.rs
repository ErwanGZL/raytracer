pub mod color;
pub use color::Color;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]

pub struct Material {
    color: Color,
    alpha: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn new(color: Color, alpha: f64, specular: f64, shininess: f64) -> Material {
        Material {
            color,
            alpha,
            specular,
            shininess,
        }
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn specular(&self) -> f64 {
        self.specular
    }
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
    pub fn shininess(&self) -> f64 {
        self.shininess.clamp(0.0, 128.0)
    }

    pub fn from_json(data: &Value) -> Material {
        let color = Color::from_json(&data["color"]);
        let alpha = data["alpha"].as_f64().unwrap();
        let specular = data["specular"].as_f64().unwrap();
        let shininess = data["shininess"].as_f64().unwrap();
        return Material::new(color, alpha, specular, shininess);
    }
}
