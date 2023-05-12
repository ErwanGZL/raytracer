use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::ops::{Add, Mul};

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
    pub fn blend(&self, other: Self, weight: f32) -> Self {
        let weight = weight.clamp(0.0, 1.0);
        let r = (self.r as f32 * (1. - weight) + other.r as f32 * weight) as u8;
        let g = (self.g as f32 * (1. - weight) + other.g as f32 * weight) as u8;
        let b = (self.b as f32 * (1. - weight) + other.b as f32 * weight) as u8;
        Color { r, g, b }
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: ((self.r as u16 * rhs.r as u16) / 255) as u8,
            g: ((self.g as u16 * rhs.g as u16) / 255) as u8,
            b: ((self.b as u16 * rhs.b as u16) / 255) as u8,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: (self.r as f64 * rhs) as u8,
            g: (self.g as f64 * rhs) as u8,
            b: (self.b as f64 * rhs) as u8,
        }
    }
}

impl Add<Color> for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
        }
    }
}
