use crate::{camera::Camera, material::Color, primitive::Primitive, dot_light::DotLight};

pub struct Scene {
    bg_color: Color,
    camera: Camera,
    primitives: Vec<Box<dyn Primitive>>,
    lights: Vec<DotLight>,
}

impl Scene {
    pub fn new(bg_color: Color, camera: Camera, primitives: Vec<Box<dyn Primitive>>, lights: Vec<DotLight>) -> Self {
        Scene {
            bg_color,
            camera,
            primitives,
            lights,
        }
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

    pub fn lights(&self) -> &Vec<DotLight> {
        &self.lights
    }
}
