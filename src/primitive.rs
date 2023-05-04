mod sphere;
use std::error::Error;

use serde_json::Value;
pub use sphere::Sphere;

use crate::{material::Material, math::Vector3D, ray::Ray};
use serde::{Deserialize, Serialize};

pub trait Primitive {
    fn material(&self) -> Material;
    fn hits(&self, ray: &Ray) -> Vec<(Vector3D, Material)>;
}
