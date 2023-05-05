mod image;
pub use image::Image;

use crate::{
    math::{Rectangle3D, Vector3D},
    ray::Ray, material::Color,
};

#[derive(Debug)]
pub struct Camera {
    eye: Vector3D,
    screen: Rectangle3D,
    focal_length: f64,
    image: Image,
    viewport_width: f64,
    viewport_height: f64,
}

impl Camera {
    pub fn new(eye: Vector3D, image: Image, focal_length: f64) -> Self {
        let viewport_width = image.width() as f64;
        let viewport_height = image.height() as f64;
        let horizontal = Vector3D::new(viewport_width, 0., 0.);
        let vertical = Vector3D::new(0., viewport_height, 0.);
        let lower_left_corner = eye - horizontal / 2.0 - vertical / 2.0 - Vector3D::new(0.0, 0.0, focal_length);
        let screen = Rectangle3D::new(lower_left_corner, horizontal + vertical);
        Self {
            eye,
            screen,
            focal_length,
            image,
            viewport_width,
            viewport_height,
        }
    }
    pub fn eye(&self) -> Vector3D {
        self.eye
    }
    pub fn at(&self, u: f64, v: f64) -> Ray {
        let p = self.screen.point_at(u, v);
        Ray::new(self.eye, Vector3D::new(p.x(), p.y(), p.z()))
    }
    pub fn write(&mut self, color: Color) {
        self.image.write(&color);
    }
    pub fn viewport_height(&self) -> f64 {
        self.viewport_height
    }
    pub fn viewport_width(&self) -> f64 {
        self.viewport_width
    }
    pub fn image(&self) -> &Image {
        &self.image
    }
    pub fn image_as_mut(&mut self) -> &mut Image {
        &mut self.image
    }
}
