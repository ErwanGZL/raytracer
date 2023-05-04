use serde_json::Value;
use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn from_json(data: &Value) -> Color {
        let r = data["r"].as_u64().unwrap() as u8;
        let g = data["g"].as_u64().unwrap() as u8;
        let b = data["b"].as_u64().unwrap() as u8;
        Color::new(r, g, b)
    }

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
    pub fn white() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }
    pub fn red() -> Self {
        Color { r: 255, g: 0, b: 0 }
    }
    pub fn green() -> Self {
        Color { r: 0, g: 255, b: 0 }
    }
    pub fn blue() -> Self {
        Color { r: 0, g: 0, b: 255 }
    }
    pub fn yellow() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 0,
        }
    }
    pub fn cyan() -> Self {
        Color {
            r: 0,
            g: 255,
            b: 255,
        }
    }
    pub fn magenta() -> Self {
        Color {
            r: 255,
            g: 0,
            b: 255,
        }
    }
    pub fn gray() -> Self {
        Color {
            r: 128,
            g: 128,
            b: 128,
        }
    }
}

impl Color {
    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn write(&self, target: &mut File) {
        writeln!(target, "{} {} {}", self.r, self.g, self.b).expect("write")
    }
}
