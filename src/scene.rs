use super::Vector3D;
use crate::light::{self, Dot, Light};
use crate::primitive::{Intersection, Sphere};
use crate::{camera::Camera, material::Color, primitive::Primitive};
use crate::{material::Material, ray::Ray};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, io::Write};

pub struct Scene {
    bg_color: Color,
    ambiant_light: light::Ambiant,
    camera: Camera,
    primitives: Vec<Box<dyn Primitive>>,
    lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn new(
        bg_color: Color,
        ambiant_light: light::Ambiant,
        camera: Camera,
        primitives: Vec<Box<dyn Primitive>>,
        lights: Vec<Box<dyn Light>>,
    ) -> Self {
        Scene {
            bg_color,
            ambiant_light,
            camera,
            primitives,
            lights,
        }
    }
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}

impl Scene {
    pub fn bake(&mut self) {
        let mut file = File::create(self.camera().image().file_path()).unwrap();
        writeln!(
            file,
            "P3\n{} {}\n255",
            self.camera().image().width(),
            self.camera().image().height()
        )
        .expect("writeln");

        let image_height = self.camera.image().height();
        let image_width = self.camera.image().width();
        for y in 0..image_height {
            let v: f64 = y as f64 / (image_height - 1) as f64;
            for x in (0..image_width).rev() {
                let u: f64 = x as f64 / (image_width - 1) as f64;
                let ray = self.camera.at(u, v);
                let intersection = ray.intersect(&self.primitives);
                let color = self.lighting(intersection);
                // self.camera.image_as_mut().write(&color);
                writeln!(file, "{} {} {}", color.r(), color.g(), color.b()).expect("writeln");
            }
        }
    }

    fn lighting(&self, i: Option<Intersection>) -> Color {
        match i {
            Some(i) => {
                let i = Intersection::new(i.point() + i.normal(), i.primitive());
                let mut color = self.ambiant(&i);
                for l in &self.lights {
                    let ray = Ray::new(i.point(), l.direction_from(i.point()));
                    let mut is_shadow: bool = false;
                    for p in &self.primitives {
                        if p.hits(&ray).len() != 0 {
                            is_shadow = true;
                            break;
                        }
                    }
                    if !is_shadow {
                        color = color + self.diffuse(&i, &l);
                        // color = color + self.specular(&i);
                    }
                }
                color
            }
            None => self.bg_color,
        }
    }

    fn ambiant(&self, i: &Intersection) -> Color {
        i.material().color() * self.ambiant_light.color() * self.ambiant_light.intensity() as f64
    }
    fn diffuse(&self, i: &Intersection, l: &Box<dyn Light>) -> Color {
        let cos_theta = i.normal().dot(l.direction_from(i.point())).max(0.0);
        l.color() * l.intensity() as f64 * cos_theta
    }
    fn specular(&self, _i: &Intersection) -> Color {
        todo!()
    }
}
