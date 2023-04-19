pub mod maths;
pub mod ray;

use maths::*;
use ray::Ray;

fn main() {
    let v = Vector3D::new(2., 3., 4.);
    let p = Point3D::new(2., 3., 4.);
    println!("{:#?}", Ray::new(p, v));
}
