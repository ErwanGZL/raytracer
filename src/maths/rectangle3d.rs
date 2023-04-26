use super::Vector3D;

#[derive(Debug)]
pub struct Rectangle3D {
    origin: Vector3D, // bottom-left corner
    dimensions: Vector3D,
}

impl Rectangle3D {
    pub fn new(origin: Vector3D, dimensions: Vector3D) -> Self {
        Self {
            origin,
            dimensions,
        }
    }
    pub fn point_at(&self, u: f64, v: f64) -> Vector3D {
        Vector3D::new(
            self.origin.x() + self.dimensions.x() * u,
            self.origin.y() + self.dimensions.y() * v,
            self.origin.z(),
        )
    }
}
