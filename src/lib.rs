#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod camera;
mod light;
mod material;
mod math;
mod primitive;
mod ray;
mod scene;

use camera::Camera;
use camera::Image;
use material::{Color, Material};
use math::Vector3D;
use primitive::Plane;
use primitive::Primitive;
use primitive::Sphere;
use ray::Ray;
use scene::Scene;

const IMAGE_HEIGHT: i32 = 480;

const AMBIANT_COEFFICIENT: f32 = 0.7;

pub fn render_image() {
    let mut scene = Scene::new(
        Color::black(),
        light::Ambiant::new(Color::white(), AMBIANT_COEFFICIENT),
        Camera::new(
            Vector3D::new(0., 0., 0.),
            Vector3D::new(0., 0., -1.),
            50.0,
            Image::new(IMAGE_HEIGHT * 3/2, IMAGE_HEIGHT, "out.ppm"),
        ),
        vec![
            Box::new(Sphere::new(
                Vector3D::new(0., 2., -3.),
                0.5,
                Material::new(Color::red(), 1.),
            )),
        ],
        vec![light::Dot::new(
            Vector3D::new(6., 4., 3.),
            Color::green(),
            0.3,
        )],
    );
    println!("{:#?}", scene.camera());
    scene.bake();
}
