use crate::light::{self, Light};

use super::Vector3D;
use crate::primitive::{Circle, Cone, Cylinder, Plane, Sphere};
use crate::{camera::Camera, material::Color, primitive::Primitive};
use crate::{material::Material, ray::Ray};
use nannou::draw::properties::spatial::position;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn from_json_prim(data: &Value) -> Box<dyn Primitive> {
    let material = Material::from_json(&data["material"]);

    let shape: Box<dyn Primitive> = match data["type"].as_str().unwrap() {
        "sphere" => Box::new(Sphere::new(
            Vector3D::from_json(&data["position"]),
            data["radius"].as_f64().unwrap() as f64,
            material,
        )),
        "plane" => Box::new(Plane::new(
            Vector3D::from_json(&data["position"]),
            Vector3D::from_json(&data["normal"]),
            material,
        )),
        "cylinder" => Box::new(Cylinder::new(
            Vector3D::from_json(&data["position"]),
            data["radius"].as_f64().unwrap() as f64,
            data["height"].as_f64().unwrap() as f64,
            material,
        )),
        "circle" => Box::new(Circle::new(
            Vector3D::from_json(&data["position"]),
            data["radius"].as_f64().unwrap() as f64,
            material,
        )),
        "cone" => Box::new(Cone::new(
            Vector3D::from_json(&data["position"]),
            Vector3D::from_json(&data["rotation"]),
            Vector3D::from_json(&data["normal"]),
            data["height"].as_f64().unwrap() as f64,
            data["radius"].as_f64().unwrap() as f64,
            material,
        )),
        _ => unimplemented!(), // add other shapes here
    };
    return shape;
}

pub fn from_json_light(data: &Value) -> Box<dyn Light> {
    let position = Vector3D::from_json(&data["position"]);
    let color = Color::from_json(&data["color"]);
    let intensity = data["intensity"].as_f64().unwrap() as f32;

    let light_thing: Box<dyn Light> = match data["type"].as_str().unwrap() {
        "point" => Box::new(light::Dot::new(position, color, intensity)),
        "directional" => Box::new(light::Directional::new(color, intensity, position)),
        _ => unimplemented!(), // add other lighrs here
    };
    return light_thing;
}
