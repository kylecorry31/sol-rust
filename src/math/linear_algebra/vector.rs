use super::{RawTensor, Tensor};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub tensor: RawTensor,
}

impl Tensor for Vector {
    fn get_tensor(&self) -> &RawTensor {
        &self.tensor
    }
}

impl Vector {
    pub fn new_2d(x: f32, y: f32) -> Self {
        Vector::new(x, y, 0.0)
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector {
            x,
            y,
            z,
            tensor: vec![vec![x], vec![y], vec![z]],
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
        let vec = Vector::new(1.0, 2.0, 3.0);
        let tensor = vec.get_tensor();
        assert_approx_eq!(tensor[0][0], 1.0);
        assert_approx_eq!(tensor[1][0], 2.0);
        assert_approx_eq!(tensor[2][0], 3.0);
    }
}
