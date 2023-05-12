use std::{fs::File, io::Write};

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::material::Color;

#[derive(Debug, Serialize, Deserialize)]

pub struct Image {
    width: i32,
    height: i32,
    file_path: String,
}

impl Image {
    pub fn new(width: i32, height: i32, path: &str) -> Self {
        let mut file = File::create(path).unwrap();
        writeln!(file, "P3\n{} {}\n255", width, height).expect("writeln");
        let file_path = path.to_owned();
        Self {
            width,
            height,
            file_path,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn from_json(data: &Value) -> Image {
        let width = data["width"].as_i64().unwrap() as i32;
        let height = data["height"].as_i64().unwrap() as i32;
        let file = data["output_file"].as_str().unwrap();
        Image::new(width, height, file)
    }
}
