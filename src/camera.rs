use crate::{
    math::{Rectangle3D, Vector3D},
    ray::Ray,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
    origin: Vector3D,
    screen: Rectangle3D,
}

impl Camera {
    pub fn new(origin: Vector3D, screen: Rectangle3D) -> Self {
        Self { origin, screen }
    }

    pub fn origin(&self) -> Vector3D {
        self.origin
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let p = self.screen.point_at(u, v);
        Ray::new(self.origin, Vector3D::new(p.x(), p.y(), p.z()))
    }

    pub fn from_json(data: &Value) -> Camera {
        let origin = Vector3D::from_json(&data["position"]);
        let screen = Rectangle3D::from_json(&data["viewport"]);
        Camera::new(origin, screen)
    }
}
