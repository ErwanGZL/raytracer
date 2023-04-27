use crate::{camera::Camera, primitives::Primitive};

pub struct Scene {
    camera: Camera,
    primitives: Vec<Box<dyn Primitive>>,
}

impl Scene {
    pub fn new(camera: Camera, primitives: Vec<Box<dyn Primitive>>) -> Self {
        Scene { camera, primitives }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn primitives(&self) -> &Vec<Box<dyn Primitive>> {
        &self.primitives
    }
}
