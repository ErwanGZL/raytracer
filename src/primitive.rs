mod sphere;
mod plane;
pub use sphere::Sphere;
pub use plane::Plane;
use crate::{material::Material, math::Vector3D, ray::Ray};

pub trait Primitive {
    fn material(&self) -> Material;
    fn hits(&self, ray: &Ray) -> Vec<(Vector3D, &dyn Primitive)>;
    fn normal(&self, point: Vector3D) -> Vector3D;
}
