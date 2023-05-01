#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod camera;
mod dot_light;
mod material;
mod math;
mod primitive;
mod ray;
mod scene;

use camera::Camera;
use dot_light::DotLight;
use material::{Color, Material};
use math::Rectangle3D;
use math::Vector3D;
use primitive::Primitive;
use primitive::Sphere;
use ray::Ray;
use scene::Scene;
use std::fs::File;
use std::io::Write;

const RESOLUTION_WIDTH: i32 = 500;
const RESOLUTION_HEIGHT: i32 = 500;
const AMBIANT_COEFFICIENT: f64 = 0.1;

pub fn render_image() {
    let mut out = File::create("out.ppm").expect("create");

    writeln!(out, "P3\n{} {}\n255", RESOLUTION_WIDTH, RESOLUTION_HEIGHT).expect("writeln");

    let scene = Scene::new(
        Color::black(),
        Camera::new(
            Vector3D::default(),
            Rectangle3D::new(Vector3D::new(-0.5, -0.5, -0.5), Vector3D::new(1., 1., 0.)),
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
        vec![DotLight::new(Vector3D::new(6., 4., 3.), Color::green(), 0.3)],
    );
    for y in 0..RESOLUTION_HEIGHT {
        let v = y as f64 * (1. / RESOLUTION_HEIGHT as f64);
        for x in 0..RESOLUTION_WIDTH {
            let u = x as f64 * (1. / RESOLUTION_WIDTH as f64);
            casting(&scene, &mut out, u, v)
        }
    }
}

fn get_closest_point<'a>(
    camera: &Camera,
    v: &Vec<(Vector3D, &'a dyn Primitive)>,
) -> Option<(Vector3D, &'a dyn Primitive)> {
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
    let mut v: Vec<(Vector3D, &dyn Primitive)> = Vec::new();
    for prim in scene.primitives() {
        v.extend(prim.hits(&ray));
    }
    match get_closest_point(scene.camera(), &v) {
        Some(intersection) => apply_light(scene, output, intersection),
        None => scene.bg_color().write(output),
    };
}

fn apply_light(scene: &Scene, output: &mut File, intersection: (Vector3D, &dyn Primitive)) {
    let (point, owner) = intersection;
    let point = point + owner.normal(point);
    for light in scene.lights() {
        let ray = Ray::from_points(point, light.position());
        let mut is_shadow = false;
        for prim in scene.primitives() {
            if prim.hits(&ray).len() > 0 {
                is_shadow = true;
                break;
            }
        }
        if !is_shadow {
            blend_light(intersection, light).write(output);
        } else {
            Color::black().write(output);
        }
    }
}

fn blend_light(intersection: (Vector3D, &dyn Primitive), light: &DotLight) -> Color {
    let (point, owner) = intersection;
    let surface_normal = owner.normal(point);
    let light_direction = (light.position() - point).normalize();
    let cos_theta = surface_normal.dot(&light_direction).max(0.0);
    let light_color = light.color() * light.intensity() as f64 * cos_theta;
    let ambient_color = owner.material().color() * AMBIANT_COEFFICIENT;
    ambient_color + light_color
}
