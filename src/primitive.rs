mod sphere;
pub use sphere::Sphere;

use crate::{material::Material, math::Vector3D, ray::Ray};

pub trait Primitive {
    fn material(&self) -> Material;
    fn hits(&self, ray: &Ray) -> Vec<(Vector3D, Material)>;
}
