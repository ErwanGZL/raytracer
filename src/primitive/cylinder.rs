use crate::{material::Material, math::Vector3D, ray::Ray};

use super::{Intersection, Primitive};

#[derive(Debug, Clone, Copy)]
pub struct Cylinder {
    center: Vector3D,
    radius: f64,
    height: f64,
    material: Material,
}

impl Cylinder {
    pub fn new(center: Vector3D, radius: f64, height: f64, color: Material) -> Self {
        Cylinder {
            center,
            radius,
            height,
            material: color,
        }
    }
}

impl Primitive for Cylinder {
    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, point: Vector3D) -> Vector3D {
        (point - self.center).normalize()
    }

    fn hits(&self, ray: &Ray) -> Vec<Intersection> {
        let dir = ray.direction();
        let pos = ray.origin();

        let mut v: Vec<Intersection> = Vec::new();

        let a = (dir.x() * dir.x()) + (dir.z() * dir.z());
        let b =
            2. * (dir.x() * (pos.x() - self.center.x()) + dir.z() * (pos.z() - self.center.z()));
        let c = (pos.x() - self.center.x()) * (pos.x() - self.center.x())
            + (pos.z() - self.center.z()) * (pos.z() - self.center.z())
            - (self.radius * self.radius);

        let delta = b * b - 4. * (a * c);
        if delta.abs() < 0.001 {
            return v;
        }
        if delta < 0.0 {
            return v;
        }

        let t1 = (-b - delta.sqrt()) / (2. * a);
        let t2 = (-b + delta.sqrt()) / (2. * a);

        let t = if t1 > t2 { t2 } else { t1 };

        let r = pos.y() + t * dir.y();

        // remove = infinite
        if (r >= self.center.y()) && (r <= self.center.y() + self.height) {
            v.push(Intersection::new(ray.origin() + ray.direction() * t, self));
        }
        v
    }
}
