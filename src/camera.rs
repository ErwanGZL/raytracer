use crate::{
    maths::{Point3D, Rectangle3D, Vector3D},
    ray::Ray,
};
#[derive(Debug)]
pub struct Camera {
    origin: Point3D,
    screen: Rectangle3D,
}

impl Camera {
    pub fn new(origin: Point3D, screen: Rectangle3D) -> Self {
        Self { origin, screen }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let p = self.screen.point_at(u, v);
        Ray::new(self.origin, Vector3D::new(p.x(), p.y(), p.z()))
    }
}
