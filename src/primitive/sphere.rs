use crate::{material::Material, math::Vector3D, ray::Ray};

use super::{Intersection, Primitive};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vector3D,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f64, color: Material) -> Self {
        Sphere {
            center,
            radius,
            material: color,
        }
    }
}

impl Primitive for Sphere {
    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, point: Vector3D) -> Vector3D {
        (point - self.center).normalize()
    }

    fn hits(&self, ray: &Ray) -> Vec<Intersection> {
        let d = ray.direction();
        let o = ray.origin();

        let a: f64 = d.x().powi(2) + d.y().powi(2) + d.z().powi(2);
        let b: f64 = 2. * (d.x() * (o.x() - self.center.x()))
            + 2. * (d.y() * (o.y() - self.center.y()))
            + 2. * (d.z() * (o.z() - self.center.z()));
        let c: f64 = o.x().powi(2)
            + o.y().powi(2)
            + o.z().powi(2)
            + self.center.x().powi(2)
            + self.center.y().powi(2)
            + self.center.z().powi(2)
            - 2. * (self.center.x() * o.x() + self.center.y() * o.y() + self.center.z() * o.z())
            - self.radius.powi(2);

        let delta = b.powi(2) - 4. * a * c;
        let mut v: Vec<Intersection> = Vec::new();
        if delta > 0. {
            let x1 = (-b + delta.sqrt()) / (2. * a);
            let x2 = (-b - delta.sqrt()) / (2. * a);
            if x1 > 0. {
                v.push(Intersection::new(
                    Vector3D::new(o.x() + x1 * d.x(), o.y() + x1 * d.y(), o.z() + x1 * d.z()),
                    self,
                ));
            }
            if x2 > 0. {
                v.push(Intersection::new(
                    Vector3D::new(o.x() + x2 * d.x(), o.y() + x2 * d.y(), o.z() + x2 * d.z()),
                    self,
                ));
            }
        } else if delta == 0. {
            let x0 = -b / (2. * a);
            if x0 > 0. {
                v.push(Intersection::new(
                    Vector3D::new(o.x() + x0 * d.x(), o.y() + x0 * d.y(), o.z() + x0 * d.z()),
                    self,
                ));
            }
        }
        return v;
    }
}
