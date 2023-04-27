mod camera;
mod maths;
mod primitives;
mod ray;
mod scene;

use camera::Camera;
use maths::*;
use primitives::Sphere;
use ray::Ray;

use std::fs::File;
use std::io::Write;

use crate::scene::Scene;

fn main() {
    let mut out = File::create("out.ppm").expect("create");
    const RESOLUTION_WIDTH: i32 = 500;
    const RESOLUTION_HEIGHT: i32 = 500;

    writeln!(out, "P1\n{} {}", RESOLUTION_WIDTH, RESOLUTION_HEIGHT).expect("writeln");

    let scene = Scene::new(
        Camera::new(
            Vector3D::default(),
            Rectangle3D::new(Vector3D::new(-0.5, -0.5, 0.5), Vector3D::new(1., 1., 0.)),
        ),
        vec![
            Box::new(Sphere::new(Vector3D::new(0.2, 0., 1.), 0.1)),
            Box::new(Sphere::new(Vector3D::new(-0.2, 0., 1.), 0.1)),
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

fn get_closest_point(camera: &Camera, v: &Vec<Vector3D>) -> Option<Vector3D> {
    match v
        .iter()
        .map(|&x| (x.distance(camera.origin()), x))
        .min_by(|x, y| x.0.total_cmp(&y.0))
    {
        Some((_, p)) => Some(p),
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
        Some(_) => writeln!(output, "1").expect("write"),
        None => writeln!(output, "0").expect("write"),
    };
}
