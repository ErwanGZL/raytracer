#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod camera;
mod material;
mod math;
mod primitive;
mod ray;
mod scene;
mod shape;

use camera::Camera;
use material::{Color, Material};
use math::Rectangle3D;
use math::Vector3D;
use primitive::Primitive;
use primitive::Sphere;
use scene::Scene;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::shape::from_json_prim;

pub fn render_image(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let data: serde_json::Value = serde_json::from_str(&buff).unwrap();

    let bg_color = Color::from_json(&data["color"]);
    let camera = Camera::from_json(&data["camera"]);
    let mut primitives = Vec::new();
    for primitive_data in data["primitive"].as_array().unwrap() {
        let primitive: Box<dyn Primitive> = from_json_prim(primitive_data);
        primitives.push(primitive);
    }

    let mut out = File::create("out.ppm").expect("create");
    const RESOLUTION_WIDTH: i32 = 500;
    const RESOLUTION_HEIGHT: i32 = 500;

    writeln!(out, "P3\n{} {}\n255", RESOLUTION_WIDTH, RESOLUTION_HEIGHT).expect("writeln");
    let scene = Scene::new(bg_color, camera, primitives);
    for y in 0..RESOLUTION_HEIGHT {
        let v = y as f64 * (1. / RESOLUTION_HEIGHT as f64);
        for x in 0..RESOLUTION_WIDTH {
            let u = x as f64 * (1. / RESOLUTION_WIDTH as f64);
            casting(&scene, &mut out, u, v)
        }
    }
}

fn get_closest_point(
    camera: &Camera,
    v: &Vec<(Vector3D, Material)>,
) -> Option<(Vector3D, Material)> {
    match v
        .iter()
        .map(|&x| (x.0.distance(camera.origin()), x))
        .min_by(|x, y| x.0.total_cmp(&y.0))
    {
        Some((_, point)) => Some(point),
        None => None,
    }
}

fn casting(scene: &Scene, output: &mut File, u: f64, v: f64) {
    let ray = scene.camera().ray(u, v);
    let mut v: Vec<(Vector3D, Material)> = Vec::new();
    for prim in scene.primitives() {
        v.extend(prim.hits(&ray));
    }
    match get_closest_point(scene.camera(), &v) {
        Some((_, material)) => material.color().write(output),
        None => scene.bg_color().write(output),
    };
}
