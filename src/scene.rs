use crate::{camera::Camera, primitive::Primitive, material::Color};

pub struct Scene {
    bg_color: Color,
    camera: Camera,
    primitives: Vec<Box<dyn Primitive>>,
}

impl Scene {
    pub fn new(bg_color: Color, camera: Camera, primitives: Vec<Box<dyn Primitive>>) -> Self {
        Scene { bg_color, camera, primitives }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn bg_color(&self) -> Color {
        self.bg_color
    }

    pub fn primitives(&self) -> &Vec<Box<dyn Primitive>> {
        &self.primitives
    }
}
