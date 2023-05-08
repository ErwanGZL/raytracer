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
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image.aspect_ratio();

        let lower_left_corner = eye - Vector3D::new(viewport_width / 2.0, viewport_height / 2.0, focal_length);
        let dimensions = eye + Vector3D::new(viewport_width, viewport_height, 0.0);

        let screen = Rectangle3D::new(lower_left_corner, dimensions);
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
        Ray::new(self.eye, self.screen.point_at(u, v))
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
