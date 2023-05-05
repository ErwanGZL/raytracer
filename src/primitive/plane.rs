use crate::{
    material::{self, Color, Material},
    math::Vector3D,
    ray::Ray,
};

use super::Primitive;

#[derive(Debug)]
pub struct Plane {
    origin: Vector3D,
    normal: Vector3D,
    material: Material,
}

impl Plane {
    pub fn new(origin: Vector3D, normal: Vector3D, material: Material) -> Self {
        Plane {
            origin,
            normal,
            material,
        }
    }
}

impl Primitive for Plane {
    fn material(&self) -> Material {
        self.material
    }

    fn hits(&self, ray: &Ray) -> Vec<(Vector3D, &dyn Primitive)> {
        let mut v: Vec<(Vector3D, &dyn Primitive)> = Vec::new();
        let delta = self.normal.dot(ray.direction());
        if delta.abs() < 1e-6 {
            return v;
        }
        let t = (self.origin - ray.origin()).dot(self.normal) / delta;
        if t > 0.0 {
            v.push((ray.origin() + ray.direction() * t, self));
        }
        v
    }

    fn normal(&self, _point: Vector3D) -> Vector3D {
        self.normal
    }
}
