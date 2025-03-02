use crate::math::utils::is_approximately_zero;

use super::{Matrix, RawTensor, Tensor, dot, magnitude, map, transpose};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub data: RawTensor,
}

impl Tensor for Vector {
    fn get_tensor(&self) -> &RawTensor {
        &self.data
    }
}

impl Vector {
    pub fn new_2d(x: f32, y: f32) -> Self {
        Vector::new_3d(x, y, 0.0)
    }

    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        Vector {
            x,
            y,
            z,
            data: vec![vec![x], vec![y], vec![z]],
        }
    }

    pub fn from(data: Vec<f32>) -> Self {
        Vector {
            x: data.first().copied().unwrap_or(0.0),
            y: data.get(1).copied().unwrap_or(0.0),
            z: data.get(2).copied().unwrap_or(0.0),
            data: transpose(&vec![data]),
        }
    }

    pub fn to_matrix(&self) -> Matrix {
        Matrix::from(self.data.clone())
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        dot(&transpose(self.get_tensor()), other.get_tensor())[0][0]
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new_3d(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn magnitude(&self) -> f32 {
        magnitude(self.get_tensor())
    }

    pub fn map(&self, operation: impl Fn(f32) -> f32) -> Vector {
        Matrix::from(map(self.get_tensor(), operation)).to_vector()
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector::new_3d(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector::new_3d(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vector::new_3d(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl std::ops::Div for Vector {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vector::new_3d(
            if is_approximately_zero(self.x) && is_approximately_zero(other.x) {
                0.0
            } else {
                self.x / other.x
            },
            if is_approximately_zero(self.y) && is_approximately_zero(other.y) {
                0.0
            } else {
                self.y / other.y
            },
            if is_approximately_zero(self.z) && is_approximately_zero(other.z) {
                0.0
            } else {
                self.z / other.z
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_eq;

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_get_tensor() {
        let vec = Vector::new_3d(1.0, 2.0, 3.0);
        let tensor = vec.get_tensor();
        assert_approx_eq!(tensor[0][0], 1.0);
        assert_approx_eq!(tensor[1][0], 2.0);
        assert_approx_eq!(tensor[2][0], 3.0);
    }

    #[rstest]
    fn test_add() {
        let vec1 = Vector::new_3d(1.0, 2.0, 3.0);
        let vec2 = Vector::new_3d(4.0, 5.0, 6.0);
        let result = vec1 + vec2;
        assert_approx_eq!(result.x, 5.0);
        assert_approx_eq!(result.y, 7.0);
        assert_approx_eq!(result.z, 9.0);
    }

    #[rstest]
    fn test_subtract() {
        let vec1 = Vector::new_3d(4.0, 5.0, 6.0);
        let vec2 = Vector::new_3d(1.0, 2.0, 3.0);
        let result = vec1 - vec2;
        assert_approx_eq!(result.x, 3.0);
        assert_approx_eq!(result.y, 3.0);
        assert_approx_eq!(result.z, 3.0);
    }

    #[rstest]
    fn test_multiply() {
        let vec1 = Vector::new_3d(2.0, 3.0, 4.0);
        let vec2 = Vector::new_3d(3.0, 4.0, 5.0);
        let result = vec1 * vec2;
        assert_approx_eq!(result.x, 6.0);
        assert_approx_eq!(result.y, 12.0);
        assert_approx_eq!(result.z, 20.0);
    }

    #[rstest]
    fn test_divide() {
        let vec1 = Vector::new_3d(6.0, 8.0, 10.0);
        let vec2 = Vector::new_3d(2.0, 2.0, 2.0);
        let result = vec1 / vec2;
        assert_approx_eq!(result.x, 3.0);
        assert_approx_eq!(result.y, 4.0);
        assert_approx_eq!(result.z, 5.0);
    }

    #[rstest]
    fn test_divide_by_zero() {
        let vec1 = Vector::new_3d(0.0, 1.0, 0.0);
        let vec2 = Vector::new_3d(0.0, 2.0, 0.0);
        let result = vec1 / vec2;
        assert_approx_eq!(result.x, 0.0);
        assert_approx_eq!(result.y, 0.5);
        assert_approx_eq!(result.z, 0.0);
    }

    #[rstest]
    fn test_cross() {
        let vec1 = Vector::new_3d(1.0, 2.0, 3.0);
        let vec2 = Vector::new_3d(2.0, 3.0, 4.0);
        let expected = Vector::new_3d(-1.0, 2.0, -1.0);

        let cross = vec1.cross(&vec2);

        assert_eq!(expected, cross);
    }
}
