mod sphere;
pub use sphere::Sphere;

use crate::{material::Color, math::Vector3D, ray::Ray};

pub trait Primitive {
    fn color(&self) -> Color;
    fn hits(&self, ray: &Ray) -> Vec<Vector3D>;
}
