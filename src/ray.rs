use crate::math::Vector3D;

#[derive(Debug)]
pub struct Ray {
    origin: Vector3D,
    direction: Vector3D,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Vector3D::default(),
            direction: Vector3D::default(),
        }
    }
}

impl Ray {
    pub fn new(origin: Vector3D, direction: Vector3D) -> Self {
        Self { origin, direction }
    }
    pub fn origin(&self) -> &Vector3D {
        &self.origin
    }
    pub fn direction(&self) -> &Vector3D {
        &self.direction
    }
}
