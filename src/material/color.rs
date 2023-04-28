use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0, a: 100 }
    }
    pub fn white() -> Self {
        Color { r: 255, g: 255, b: 255, a: 100 }
    }
    pub fn red() -> Self {
        Color { r: 255, g: 0, b: 0, a: 100 }
    }
    pub fn green() -> Self {
        Color { r: 0, g: 255, b: 0, a: 100 }
    }
    pub fn blue() -> Self {
        Color { r: 0, g: 0, b: 255, a: 100 }
    }
    pub fn yellow() -> Self {
        Color { r: 255, g: 255, b: 0, a: 100 }
    }
    pub fn cyan() -> Self {
        Color { r: 0, g: 255, b: 255, a: 100 }
    }
    pub fn magenta() -> Self {
        Color { r: 255, g: 0, b: 255, a: 100 }
    }
    pub fn gray() -> Self {
        Color { r: 128, g: 128, b: 128, a: 100 }
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
    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn write(&self, target: &mut File) {
        writeln!(target, "{} {} {}", self.r, self.g, self.b).expect("write")
    }
}
