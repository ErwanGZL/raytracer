use super::Primitive;
use crate::maths::*;
use crate::Ray;

#[derive(Debug)]
pub struct Sphere {
    center: Vector3D,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Primitive for Sphere {
    fn hits(&self, ray: &Ray) -> bool {
        let d = ray.direction();
        let o = ray.origin();

        let a = d.x().powi(2) + d.y().powi(2) + d.z().powi(2);
        let b = 2. * (d.x() * (o.x() - self.center.x()))
            + 2. * (d.y() * (o.y() - self.center.y()))
            + 2. * (d.z() * (o.z() - self.center.z()));
        let c = o.x().powi(2)
            + o.y().powi(2)
            + o.z().powi(2)
            + self.center.x().powi(2)
            + self.center.y().powi(2)
            + self.center.z().powi(2)
            - 2. * (self.center.x() * o.x() + self.center.y() * o.y() + self.center.z() * o.z())
            - self.radius.powi(2);

        let delta = b.powi(2) - 4. * a * c;
        delta >= 0.
    }
}
