pub mod sphere;
pub use sphere::Sphere;

use crate::ray::Ray;

pub trait Primitive {
    fn hits(&self, ray: &Ray) -> bool;
}
