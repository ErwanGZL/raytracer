use std::ops::{Add, Div, Index, Mul, Sub, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn new(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
    }
    pub fn identity() -> Self
    where
        T: Default + Copy + Add<Output = T> + Mul<Output = T>,
    {
        let mut data = [[T::default(); COLS]; ROWS];
        for i in 0..ROWS {
            for j in 0..COLS {
                if i == j {
                    data[i][j] = T::default() + T::default();
                }
            }
        }
        Self { data }
    }
    pub fn transpose(&self) -> Matrix<T, COLS, ROWS>
    where
        T: Copy,
    {
        let mut data = [[self.data[0][0]; ROWS]; COLS];
        for i in 0..ROWS {
            for j in 0..COLS {
                data[j][i] = self.data[i][j];
            }
        }
        Matrix { data }
    }
}

impl Matrix<f64, 3, 3> {
    pub fn rotation_x(angle: f64) -> Self {
        let mut data = [[0.; 3]; 3];
        data[0][0] = 1.;
        data[1][1] = angle.cos();
        data[1][2] = -angle.sin();
        data[2][1] = angle.sin();
        data[2][2] = angle.cos();
        Self { data }
    }
    pub fn rotation_y(angle: f64) -> Self {
        let mut data = [[0.; 3]; 3];
        data[0][0] = angle.cos();
        data[0][2] = angle.sin();
        data[1][1] = 1.;
        data[2][0] = -angle.sin();
        data[2][2] = angle.cos();
        Self { data }
    }
    pub fn rotation_z(angle: f64) -> Self {
        let mut data = [[0.; 3]; 3];
        data[0][0] = angle.cos();
        data[0][1] = -angle.sin();
        data[1][0] = angle.sin();
        data[1][1] = angle.cos();
        data[2][2] = 1.;
        Self { data }
    }
    pub fn rotation(x: f64, y: f64, z: f64) -> Self {
        Self::rotation_x(x) * Self::rotation_y(y) * Self::rotation_z(z)
    }
}

impl<T, const ROWS: usize, const COLS: usize> std::fmt::Display for Matrix<T, ROWS, COLS>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.iter() {
            for col in row.iter() {
                write!(f, "{} ", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Neg for Matrix<T, ROWS, COLS>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut data = self.data;
        for row in data.iter_mut() {
            for col in row.iter_mut() {
                *col = -*col;
            }
        }
        Self { data }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS> {
    type Output = [T; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> Add<T> for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let mut data = self.data;
        for row in data.iter_mut() {
            for col in row.iter_mut() {
                *col = *col + rhs;
            }
        }
        Self { data }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Sub<T> for Matrix<T, ROWS, COLS>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let mut data = self.data;
        for row in data.iter_mut() {
            for col in row.iter_mut() {
                *col = *col - rhs;
            }
        }
        Self { data }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Mul<T> for Matrix<T, ROWS, COLS>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = self.data;
        for row in data.iter_mut() {
            for col in row.iter_mut() {
                *col = *col * rhs;
            }
        }
        Self { data }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Div<T> for Matrix<T, ROWS, COLS>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut data = self.data;
        for row in data.iter_mut() {
            for col in row.iter_mut() {
                *col = *col / rhs;
            }
        }
        Self { data }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Add for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        for (row, rhs_row) in data.iter_mut().zip(rhs.data.iter()) {
            for (col, rhs_col) in row.iter_mut().zip(rhs_row.iter()) {
                *col = *col + *rhs_col;
            }
        }
        Self { data }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Sub for Matrix<T, ROWS, COLS>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        for (row, rhs_row) in data.iter_mut().zip(rhs.data.iter()) {
            for (col, rhs_col) in row.iter_mut().zip(rhs_row.iter()) {
                *col = *col - *rhs_col;
            }
        }
        Self { data }
    }
}

impl<T, const ROWS: usize, const COLS: usize, const COLS2: usize> Mul<Matrix<T, COLS, COLS2>>
    for Matrix<T, ROWS, COLS>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    type Output = Matrix<T, ROWS, COLS2>;

    fn mul(self, rhs: Matrix<T, COLS, COLS2>) -> Self::Output {
        let mut data = [[T::default(); COLS2]; ROWS];
        for i in 0..ROWS {
            for j in 0..COLS2 {
                for k in 0..COLS {
                    data[i][j] = data[i][j] + self[i][k] * rhs[k][j];
                }
            }
        }
        Matrix { data }
    }
}
