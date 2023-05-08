use std::{fs::File, io::Write};

use crate::material::Color;

#[derive(Debug)]
pub struct Image {
    width: i32,
    height: i32,
    file: File,
}

impl Image {
    pub fn new(width: i32, height: i32, path: &str) -> Self {
        let mut file = File::create(path).unwrap();
        writeln!(file, "P3\n{} {}\n255", width, height).expect("writeln");
        Self {
            width,
            height,
            file,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
    pub fn write(&mut self, color: &Color) {
        writeln!(self.file, "{} {} {}", color.r(), color.g(), color.b()).expect("writeln");
    }
}
