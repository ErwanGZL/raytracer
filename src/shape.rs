use super::Vector3D;
use crate::primitive::Sphere;
use crate::{camera::Camera, material::Color, primitive::Primitive};
use crate::{material::Material, ray::Ray};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn from_json_prim(data: &Value) -> Box<dyn Primitive> {
    let material = Material::from_json(&data["material"]);
    let shape = match data["type"].as_str().unwrap() {
        "sphere" => Sphere::new(
            Vector3D::from_json(&data["position"]),
            data["radius"].as_f64().unwrap() as f64,
            material,
        ),
        _ => unimplemented!(), // add other shapes here
    };
    return Box::new(shape);
}
