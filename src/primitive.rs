mod circle;
mod cylinder;
mod plane;
mod sphere;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

use crate::{material::Material, math::Vector3D, ray::Ray};

pub use circle::Circle;
pub use cylinder::Cylinder;
pub use plane::Plane;
pub use sphere::Sphere;

pub trait Primitive {
    fn material(&self) -> Material;
    fn hits(&self, ray: &Ray) -> Vec<Intersection>;
    fn normal(&self, point: Vector3D) -> Vector3D;
}

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    point: Vector3D,
    primitive: &'a dyn Primitive,
}

impl<'a> Intersection<'a> {
    pub fn new(point: Vector3D, primitive: &'a dyn Primitive) -> Self {
        Intersection { point, primitive }
    }
    pub fn point(&self) -> Vector3D {
        self.point
    }
    pub fn primitive(&self) -> &dyn Primitive {
        self.primitive
    }
    pub fn material(&self) -> Material {
        self.primitive.material()
    }
    pub fn normal(&self) -> Vector3D {
        self.primitive.normal(self.point).normalize()
    }
}
