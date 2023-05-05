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
use math::Rectangle3D;
use math::Vector3D;
use primitive::Primitive;
use primitive::Sphere;
use ray::Ray;
use scene::Scene;

const IMAGE_HEIGHT: i32 = 720;
const IMAGE_WIDTH: i32 = 1280;

const AMBIANT_COEFFICIENT: f32 = 1.0;

pub fn render_image() {
    Scene::new(
        Color::blue(),
        light::Ambiant::new(Color::white(), AMBIANT_COEFFICIENT),
        Camera::new(
            Vector3D::default(),
            Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, "out.ppm"),
            1.0,
        ),
        vec![
            Box::new(Sphere::new(
                Vector3D::new(0.1, 0., -1.1),
                0.1,
                Material::new(Color::red(), 100.),
            )),
            Box::new(Sphere::new(
                Vector3D::new(0., 0., -1.2),
                0.1,
                Material::new(Color::blue(), 100.),
            )),
        ],
        vec![light::Dot::new(
            Vector3D::new(6., 4., 3.),
            Color::green(),
            0.3,
        )],
    ).bake();
}
