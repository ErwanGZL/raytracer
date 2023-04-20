pub mod maths;
pub mod primitives;
pub mod ray;

use maths::*;
use ray::Ray;
use primitives::Sphere;

fn main() {
    let ray = Ray::new(Point3D::default(), Vector3D::new(1., 0., 0.));
    let sphere = Sphere::new(Point3D::new(10., 0., 0.), 5.);
    println!("{:#?}\n{:#?}\n{}", ray, sphere, sphere.hits(&ray));
}
