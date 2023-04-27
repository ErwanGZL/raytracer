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
    fn hits(&self, ray: &Ray) -> Vec<Vector3D> {
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
        let mut v: Vec<Vector3D> = Vec::new();
        if delta > 0. {
            let x1 = (-b + delta.sqrt()) / (2. * a);
            let x2 = (-b - delta.sqrt()) / (2. * a);
            if x1 > 0. {
                v.push(Vector3D::new(
                    o.x() + x1 * d.x(),
                    o.y() + x1 * d.y(),
                    o.z() + x1 * d.z(),
                ));
            }
            if x2 > 0. {
                v.push(Vector3D::new(
                    o.x() + x2 * d.x(),
                    o.y() + x2 * d.y(),
                    o.z() + x2 * d.z(),
                ));
            }
        } else if delta == 0. {
            let x0 = -b / (2. * a);
            if x0 > 0. {
                v.push(Vector3D::new(
                    o.x() + x0 * d.x(),
                    o.y() + x0 * d.y(),
                    o.z() + x0 * d.z(),
                ));
            }
        }
        return v;
    }
}
