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

const AMBIANT_COEFFICIENT: f32 = 0.4;

pub fn render_image() {
    let mut scene = Scene::new(
        Color::black(),
        light::Ambiant::new(Color::white(), AMBIANT_COEFFICIENT),
        Camera::new(
            Vector3D::new(0., 20., -100.),
            Vector3D::new(0., 0., 1.),
            72.0,
            Image::new(IMAGE_HEIGHT * 3 / 2, IMAGE_HEIGHT, "out.ppm"),
        ),
        vec![
            Box::new(Sphere::new(
                Vector3D::new(60., 40., 5.),
                25.,
                Material::new(Color::red(), 100.),
            )),
            Box::new(Sphere::new(
                Vector3D::new(-40., -10., 20.),
                35.,
                Material::new(Color::green(), 100.),
            )),
            Box::new(Plane::new(
                Vector3D::new(0., -20., 0.),
                Vector3D::new(0., 1., 0.),
                Material::new(Color::blue(), 1.),
            )),
        ],
        vec![
            Box::new(light::Dot::new(Vector3D::new(400., 500., 100.), Color::white(), 0.6)),
            // Box::new(light::Dot::new(Vector3D::new(0., 500., 50.), Color::yellow(), 0.2)),
            Box::new(light::Directional::new(
                Color::white(),
                0.2,
                Vector3D::new(1., -1., 0.),
            )),
        ],
    );
    println!("{:#?}", scene.camera());
    scene.bake();
}
