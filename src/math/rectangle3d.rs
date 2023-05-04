use super::Vector3D;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle3D {
    origin: Vector3D, // bottom-left corner
    dimensions: Vector3D,
}

impl Rectangle3D {
    pub fn new(origin: Vector3D, dimensions: Vector3D) -> Self {
        Self { origin, dimensions }
    }
    pub fn point_at(&self, u: f64, v: f64) -> Vector3D {
        Vector3D::new(
            self.origin.x() + self.dimensions.x() * u,
            self.origin.y() + self.dimensions.y() * v,
            self.origin.z(),
        )
    }

    pub fn from_json(data: &Value) -> Rectangle3D {
        let origin = Vector3D::from_json(&data["position"]);
        let dimensions = Vector3D::from_json(&data["size"]);
        Rectangle3D::new(origin, dimensions)
    }
}
