use super::Point3D;

#[derive(Debug)]
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
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
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

impl std::ops::Add<Point3D> for Vector3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        other + self
    }
}
