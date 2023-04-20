mod camera;
mod maths;
mod primitives;
mod ray;

use camera::Camera;
use maths::*;
use primitives::Sphere;
use ray::Ray;

use std::fs::File;
use std::io::Write;

fn main() {
    let mut out = File::create("out.ppm").expect("create");
    const RESOLUTION_WIDTH: i32 = 500;
    const RESOLUTION_HEIGHT: i32 = 500
    ;

    writeln!(out, "P1\n{} {}", RESOLUTION_WIDTH, RESOLUTION_HEIGHT).expect("writeln");

    let camera = Camera::new(
        Point3D::default(),
        Rectangle3D::new(
            Point3D::new(-0.5, -0.5, 0.5),
            Vector3D::new(1., 0., 0.),
            Vector3D::new(0., 1., 0.),
        ),
    );
    println!("{:#?}", camera);
    let sphere = Sphere::new(Point3D::new(0., 0., -1.), 0.5);
    for y in 0..RESOLUTION_HEIGHT {
        let v = y as f64 * (1. / RESOLUTION_HEIGHT as f64);
        for x in 0..RESOLUTION_WIDTH {
            let u = x as f64 * (1. / RESOLUTION_WIDTH as f64);
            let ray = camera.ray(u, v);
            if sphere.hits(&ray) {
                writeln!(out, "1").expect("write");
            } else {
                writeln!(out, "0").expect("write");
            }
        }
    }
}
