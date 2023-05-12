use super::{Intersection, Primitive};
use crate::Ray;
use std::f64::consts;

use crate::{material::Material, math::Vector3D};

#[derive(Debug)]
pub struct Cone {
    center: Vector3D,
    normal: Vector3D,
    height: f64,
    radius: f64,
    material: Material,
}

impl Cone {
    pub fn new(
        center: Vector3D,
        normal: Vector3D,
        height: f64,
        radius: f64,
        color: Material,
    ) -> Self {
        Cone {
            center,
            normal,
            height,
            radius,
            material: color,
        }
    }
}

// fn material(&self) -> Material;
// fn hits(&self, ray: &Ray) -> Vec<Intersection>;
// fn normal(&self, point: Vector3D) -> Vector3D;

impl Primitive for Cone {
    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, point: Vector3D) -> Vector3D {
        (point - self.center).normalize()
    }

    fn hits(&self, ray: &Ray) -> Vec<Intersection> {
        let mut v: Vec<Intersection> = Vec::new();

        // let mut v: Vec<(Vector3D, &dyn Primitive)> = Vec::new();

        let dir = ray.direction();
        let pos = ray.origin();

        let _a = pos.x() - self.center.x();
        let _b = pos.z() - self.center.z();
        let _d = self.height - pos.y() + self.center.y();

        let tan = (self.radius / self.height) * (self.radius / self.height);

        let a = (dir.x() * dir.x()) + (dir.z() * dir.z()) - (tan * (dir.y() * dir.y()));
        let b = (2. * _a * dir.x()) + (2. * _b * dir.z()) + (2. * tan * _d * dir.y());
        let c = (_a * _a) + (_b * _b) - (tan * (_d * _d));

        let delta = b * b - 4. * (a * c);
        if delta.abs() < 0.001 || delta < 0.0 {
            return v;
        }
        let t1 = (-b - (delta).sqrt()) / (2. * a);
        let t2 = (-b + (delta).sqrt()) / (2. * a);
        let t;

        if t1 > t2 {
            t = t2;
        } else {
            t = t1;
        }

        let r = pos.y() + t * dir.y();

        if (r > self.center.y()) && (r < self.center.y() + self.height) {
            v.push(Intersection::new(
                Vector3D::new(pos.x() + t * dir.x(), r, pos.z() + t * dir.z()),
                self,
            ));
        }
        v
    }
}
