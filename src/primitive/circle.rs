use crate::{material::Material, math::Vector3D, ray::Ray};

use super::{Cylinder, Intersection, Plane, Primitive};

pub struct Circle {
    center: Vector3D,
    radius: f64,
    material: Material,
}

impl Circle {
    pub fn new(center: Vector3D, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Primitive for Circle {
    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, _point: Vector3D) -> Vector3D {
        Vector3D::new(0., 0., 1.)
    }

    fn hits(&self, ray: &Ray) -> Vec<Intersection> {
        let mut v = Vec::new();

        let plane_circle = Plane::new(self.center, Vector3D::new(0., 1., 0.), self.material);
        let hits = plane_circle.hits(ray);

        if hits.len() <= 0 {
            return v;
        }

        if ((hits[0].point().x() - self.center.x()).powi(2)
            + (hits[0].point().y() - self.center.y()).powi(2)
            + (hits[0].point().z() - self.center.z()).powi(2))
        .sqrt()
            > self.radius
        {
            return v;
        }
        v.push(Intersection::new(hits[0].point(), self));
        v
    }
}
