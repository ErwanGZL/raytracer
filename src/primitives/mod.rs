pub mod sphere;
pub use sphere::Sphere;

use crate::{ray::Ray, maths::Vector3D};

pub trait Primitive {
    fn hits(&self, ray: &Ray) -> Vec<Vector3D>;
}
