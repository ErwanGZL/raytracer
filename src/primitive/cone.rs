use super::{Circle, Intersection, Primitive};
use crate::{math::Matrix, Ray};
use std::f64::consts;

use crate::{material::Material, math::Vector3D};

#[derive(Debug)]
pub struct Cone {
    center: Vector3D,
    rotation: Vector3D,
    normal: Vector3D,
    height: f64,
    radius: f64,
    material: Material,
}

impl Cone {
    pub fn new(
        center: Vector3D,
        rotation: Vector3D,
        normal: Vector3D,
        height: f64,
        radius: f64,
        color: Material,
    ) -> Self {
        Cone {
            center,
            rotation,
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
        let m = Matrix::rotation(self.rotation.x(), self.rotation.y(), self.rotation.z());
        Vector3D::from_matrix(m * (point - self.center).normalize().to_matrix())
    }

    fn hits(&self, ray: &Ray) -> Vec<Intersection> {
        let ray = ray.rotate(Matrix::rotation(
            self.rotation.x(),
            self.rotation.y(),
            self.rotation.z(),
        ));
        let mut v: Vec<Intersection> = Vec::new();

        // Calculate intersection with the bottom circle
        let bottom_center = self.center + Vector3D::new(0.0, 0., 0.0);
        let bottom_circle = Circle::new(bottom_center, self.radius, self.material);
        let bottom_intersections = bottom_circle.hits(&ray);
        for i in bottom_intersections.iter() {
            let m = Matrix::rotation(-self.rotation.x(), -self.rotation.y(), -self.rotation.z());
            let p = (*i).point();
            let p = Vector3D::from_matrix(m * p.to_matrix());
            let i = Intersection::new(p, self);
            v.push(i);
        }

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
            let m = Matrix::rotation(-self.rotation.x(), -self.rotation.y(), -self.rotation.z());
            let p = Vector3D::new(pos.x() + t * dir.x(), r, pos.z() + t * dir.z());
            let p = Vector3D::from_matrix(m * p.to_matrix());
            let i = Intersection::new(p, self);
            v.push(i);
        }
        v
    }
}
