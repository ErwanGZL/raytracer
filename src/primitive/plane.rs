//  ___ __ _ _  _ __ _ _ _ ___
// (_-</ _` | || / _` | '_/ -_)
// /__/\__, |\_,_\__,_|_| \___|
//        |_|

use super::Primitive;
use crate::color::Color;
use crate::maths::*;
use crate::Ray;

#[derive(Debug)]
pub struct Plane {
    center: Vector3D,
    normal: Vector3D,
    color: Color,
}

impl Plane {
    pub fn new(center: Vector3D, normal: Vector3D, color: Color) -> Self {
        Plane {
            center,
            normal,
            color,
        }
    }
}

impl Primitive for Plane {
    fn color(&self) -> crate::color::Color {
        self.color
    }

    fn hits(&self, ray: &Ray) -> Vec<Vector3D> {
        let mut v: Vec<Vector3D> = Vec::new();
        let denom = self.normal.dot(ray.direction());
        if denom.abs() < 1e-6 {
            return v;
        }
        let t = (self.center - *ray.origin()).dot(&self.normal) / denom;
        if t > 0.0 {
            v.push(*ray.origin() + *ray.direction() * t);
        }
        v
    }
}
