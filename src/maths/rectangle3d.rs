use super::{Point3D, Vector3D};

#[derive(Debug)]
pub struct Rectangle3D {
    origin: Point3D, // bottom-left corner
    bottom_side: Vector3D,
    left_side: Vector3D,
}

impl Rectangle3D {
    pub fn new(origin: Point3D, bottom_side: Vector3D, left_side: Vector3D) -> Self {
        Self {
            origin,
            bottom_side,
            left_side,
        }
    }
    pub fn point_at(&self, u: f64, v: f64) -> Point3D {
        Point3D::new(
            self.origin.x() + self.bottom_side.x() * u,
            self.origin.y() + self.left_side.y() * v,
            self.origin.z(),
        )
    }
}
