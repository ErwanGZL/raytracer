mod image;
pub use image::Image;

use crate::{
    material::Color,
    math::Vector3D,
    ray::Ray,
};

#[derive(Debug)]
pub struct Camera {
    eye: Vector3D,
    direction: Vector3D,
    fov: f64,
    image: Image,
}

impl Camera {
    pub fn new(eye: Vector3D, direction: Vector3D, fov: f64, image: Image) -> Self {
        let direction = direction.normalize();
        Camera {
            eye,
            direction,
            fov,
            image,
        }
    }
    pub fn eye(&self) -> Vector3D {
        self.eye
    }
    pub fn at(&self, x: f64, y: f64) -> Ray {
        let half_height = (self.fov / 2.).tan();
        let half_width = self.image.aspect_ratio() * half_height;

        let right = self.direction.cross(Vector3D::new(0., 1., 0.)).normalize();
        let up = self.direction.cross(right).normalize();

        let x_dir = right * (2. * x - 1.0) * half_width;
        let y_dir = up * (2. * y - 1.0) * half_height;

        Ray::new(self.eye, (self.direction + x_dir + y_dir).normalize())
    }
    pub fn write(&mut self, color: Color) {
        self.image.write(&color);
    }
    pub fn image(&self) -> &Image {
        &self.image
    }
    pub fn image_as_mut(&mut self) -> &mut Image {
        &mut self.image
    }
}
