#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod camera;
mod material;
mod math;
mod primitive;
mod ray;
mod scene;

use camera::Camera;
use material::{Color, Material};
use math::Rectangle3D;
use math::Vector3D;
use primitive::Sphere;
use scene::Scene;
use std::fs::File;
use std::io::Write;

pub fn render_image() {
    let mut out = File::create("out.ppm").expect("create");
    const RESOLUTION_WIDTH: i32 = 500;
    const RESOLUTION_HEIGHT: i32 = 500;

    writeln!(out, "P3\n{} {}\n255", RESOLUTION_WIDTH, RESOLUTION_HEIGHT).expect("writeln");

    let scene = Scene::new(
        Color::green(),
        Camera::new(
            Vector3D::default(),
            Rectangle3D::new(Vector3D::new(-0.5, -0.5, 0.5), Vector3D::new(1., 1., 0.)),
        ),
        vec![
            Box::new(Sphere::new(
                Vector3D::new(0.2, 0., 1.),
                0.1,
                Material::new(Color::red(), 100.),
            )),
            Box::new(Sphere::new(
                Vector3D::new(-0.2, 0., 1.),
                0.1,
                Material::new(Color::blue(), 100.),
            )),
        ],
    );
    for y in 0..RESOLUTION_HEIGHT {
        let v = y as f64 * (1. / RESOLUTION_HEIGHT as f64);
        for x in 0..RESOLUTION_WIDTH {
            let u = x as f64 * (1. / RESOLUTION_WIDTH as f64);
            casting(&scene, &mut out, u, v)
        }
    }
}

fn get_closest_point(camera: &Camera, v: &Vec<Vector3D>) -> Option<Color> {
    match v
        .iter()
        .map(|&x| (x.distance(camera.origin()), x))
        .min_by(|x, y| x.0.total_cmp(&y.0))
    {
        Some((_, _p)) => todo!(),
        None => None,
    }
}

fn casting(scene: &Scene, output: &mut File, u: f64, v: f64) {
    let ray = scene.camera().ray(u, v);
    let mut v: Vec<Vector3D> = Vec::new();
    for prim in scene.primitives() {
        v.extend(prim.hits(&ray));
    }
    match get_closest_point(scene.camera(), &v) {
        Some(_) => todo!(),
        None => scene.bg_color().write(output),
    };
}
