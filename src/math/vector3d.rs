use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Matrix;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]

pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Default for Vector3D {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Vector3D {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn distance(&self, other: Self) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2))
            .sqrt()
    }

    pub fn from_json(data: &Value) -> Vector3D {
        let x = data["x"].as_f64().unwrap();
        let y = data["y"].as_f64().unwrap();
        let z = data["z"].as_f64().unwrap();
        Vector3D::new(x, y, z)
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn direction_to(&self, other: Self) -> Self {
        (other - *self).normalize()
    }

    // transform the vector to a matrix
    pub fn to_matrix(&self) -> Matrix<f64, 3, 1> {
        Matrix::new([[self.x], [self.y], [self.z]])
    }
    // transform the result of an applied rotation matrix to a vector
    pub fn from_matrix(matrix: Matrix<f64, 3, 1>) -> Self {
        Self::new(matrix[0][0], matrix[1][0], matrix[2][0])
    }
}

impl std::ops::Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul for Vector3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Mul<Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, vector: Vector3D) -> Vector3D {
        vector * self
    }
}

impl std::ops::Div for Vector3D {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Div<f64> for Vector3D {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
