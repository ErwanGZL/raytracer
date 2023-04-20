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

fn main(){
    let mut out = File::create("out.ppm").expect("create");
    const SCREEN_WIDTH: i32 = 1920;
    const SCREEN_HEIGHT: i32 = 1080;

    writeln!(out, "P1\n{} {}", SCREEN_WIDTH, SCREEN_HEIGHT).expect("writeln");

    let camera = Camera::new(
        Point3D::default(),
        Rectangle3D::new(
            Point3D::new((SCREEN_WIDTH / 2).into(), (SCREEN_HEIGHT / 2).into(), 10.),
            Vector3D::new(SCREEN_WIDTH.into(), 0., 0.),
            Vector3D::new(0., SCREEN_WIDTH.into(), 0.),
        ),
    );
    let sphere = Sphere::new(Point3D::new(0., 0., -60.), 10.);
    for y in 0..=SCREEN_HEIGHT {
        for x in 0..=SCREEN_WIDTH {
            let u: f64 = (x / (SCREEN_WIDTH - 1)).into();
            let v: f64 = (y / (SCREEN_WIDTH - 1)).into();
            let ray = camera.ray(u, v);
            if sphere.hits(&ray) {
                write!(out, "1").expect("write");
            } else {
                write!(out, "0").expect("write");
            }
        }
    }
}
