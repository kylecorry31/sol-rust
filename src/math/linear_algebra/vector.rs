use super::{RawTensor, Tensor, transpose};

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
}
