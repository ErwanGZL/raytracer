use super::{Circle, Intersection, Primitive};
use crate::{material::Material, math::Vector3D, ray::Ray};

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
        let mut vec_i: Vec<Intersection> = Vec::new();

        // Calculate intersection with the top circle
        let top_center = self.center + Vector3D::new(0.0, self.height, 0.0);
        let top_circle = Circle::new(top_center, self.radius, self.material);
        let top_intersections = top_circle.hits(ray);
        for i in top_intersections.iter() {
            vec_i.push(Intersection::new((*i).point(), self));
        }

        // Calculate intersection with the bottom circle
        let bottom_center = self.center + Vector3D::new(0.0, 0., 0.0);
        let bottom_circle = Circle::new(bottom_center, self.radius, self.material);
        let bottom_intersections = bottom_circle.hits(ray);
        for i in bottom_intersections.iter() {
            vec_i.push(Intersection::new((*i).point(), self));
        }

        let dir = ray.direction();
        let pos = ray.origin();

        let a = (dir.x() * dir.x()) + (dir.z() * dir.z());
        let b =
            2. * (dir.x() * (pos.x() - self.center.x()) + dir.z() * (pos.z() - self.center.z()));
        let c = (pos.x() - self.center.x()) * (pos.x() - self.center.x())
            + (pos.z() - self.center.z()) * (pos.z() - self.center.z())
            - (self.radius * self.radius);

        let delta = b * b - 4. * (a * c);
        if delta.abs() < 0.001 {
            return vec_i;
        }
        if delta < 0.0 {
            return vec_i;
        }

        let t1 = (-b - delta.sqrt()) / (2. * a);
        let t2 = (-b + delta.sqrt()) / (2. * a);

        let t = if t1 > t2 { t2 } else { t1 };

        let r = pos.y() + t * dir.y();

        // vec_i.extend(bottom_intersections.iter().cloned());

        // remove = infinite
        // Check intersection with the side of the cylinder
        if r >= self.center.y() && r <= self.center.y() + self.height {
            vec_i.push(Intersection::new(ray.origin() + ray.direction() * t, self));
        }
        vec_i
    }
}
