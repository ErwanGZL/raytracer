use crate::maths::*;

#[derive(Debug)]
pub struct Ray {
    origin: Point3D,
    direction: Vector3D,
}

impl Default for Ray {
    fn default() -> Self {
        Self { origin: Point3D::default(), direction: Vector3D::default() }
    }
}

impl Ray {
    pub fn new(origin: Point3D,  direction: Vector3D) -> Self {
        Self { origin, direction }
    }
}
