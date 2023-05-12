#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod camera;
mod light;
mod material;
mod math;
mod primitive;
mod ray;
mod scene;
mod shape;

use camera::Camera;
use camera::Image;
use material::{Color, Material};
use math::Vector3D;
use primitive::Cone;
use primitive::Plane;
use primitive::Primitive;
use primitive::Sphere;
use ray::Ray;
use scene::Scene;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::light::Light;
use crate::shape::from_json_light;
use crate::shape::from_json_prim;

const IMAGE_HEIGHT: i32 = 480;

const AMBIANT_COEFFICIENT: f32 = 0.4;

pub fn render_image(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let data: serde_json::Value = serde_json::from_str(&buff).unwrap();

    let bg_color = Color::from_json(&data["color"]);
    let ambient_light = light::Ambiant::from_json(&data["ambient_light"]);
    let camera = Camera::from_json(&data["camera"]);

    let mut lights = Vec::new();
    for lights_data in data["lights"].as_array().unwrap() {
        let light: Box<dyn Light> = from_json_light(lights_data);
        lights.push(light);
    }

    let mut primitives = Vec::new();
    for primitive_data in data["objects"].as_array().unwrap() {
        let primitive: Box<dyn Primitive> = from_json_prim(primitive_data);
        primitives.push(primitive);
    }

    let mut scene = Scene::new(bg_color, ambient_light, camera, primitives, lights);

    println!("{:#?}", scene.camera());
    scene.bake();
}
