use super::Vector3D;
use crate::primitive::Sphere;
use crate::{camera::Camera, material::Color, primitive::Primitive};
use crate::{material::Material, ray::Ray};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Scene {
    bg_color: Color,
    camera: Camera,
    primitives: Vec<Box<dyn Primitive>>,
}

impl Scene {
    pub fn new(bg_color: Color, camera: Camera, primitives: Vec<Box<dyn Primitive>>) -> Self {
        Scene {
            bg_color,
            camera,
            primitives,
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn bg_color(&self) -> Color {
        self.bg_color
    }

    pub fn primitives(&self) -> &Vec<Box<dyn Primitive>> {
        &self.primitives
    }
}
