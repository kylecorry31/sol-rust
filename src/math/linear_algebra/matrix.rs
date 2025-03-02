use super::{RawTensor, Tensor, Vector};

pub struct Matrix {
    pub tensor: RawTensor,
}

impl Tensor for Matrix {
    fn get_tensor(&self) -> &RawTensor {
        &self.tensor
    }
}

impl Matrix {
    pub fn new(tensor: RawTensor) -> Self {
        Matrix { tensor }
    }

    pub fn to_vector2d(&self) -> Vector {
        if self.tensor.len() == 1 {
            // Row vector
            Vector::new(
                self.tensor[0].first().copied().unwrap_or(0.0),
                self.tensor[0].get(1).copied().unwrap_or(0.0),
                self.tensor[0].get(2).copied().unwrap_or(0.0),
            )
        } else {
            // Column vector
            Vector::new(
                self.tensor
                    .first()
                    .and_then(|row| row.first())
                    .copied()
                    .unwrap_or(0.0),
                self.tensor
                    .get(1)
                    .and_then(|row| row.first())
                    .copied()
                    .unwrap_or(0.0),
                self.tensor
                    .get(2)
                    .and_then(|row| row.first())
                    .copied()
                    .unwrap_or(0.0),
            )
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
        let matrix = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let tensor = matrix.get_tensor();
        assert_approx_eq!(tensor[0][0], 1.0);
        assert_approx_eq!(tensor[0][1], 2.0);
        assert_approx_eq!(tensor[1][0], 3.0);
        assert_approx_eq!(tensor[1][1], 4.0);
    }

    #[rstest]
    #[case(vec![vec![1.0, 2.0]], Vector::new(1.0, 2.0, 0.0))]
    #[case(vec![vec![1.0], vec![2.0]], Vector::new(1.0, 2.0, 0.0))]
    #[case(vec![vec![1.0, 2.0, 3.0]], Vector::new(1.0, 2.0, 3.0))]
    #[case(vec![vec![1.0], vec![2.0], vec![3.0]], Vector::new(1.0, 2.0, 3.0))]
    fn test_to_vector(#[case] input: RawTensor, #[case] expected: Vector) {
        let matrix = Matrix::new(input);
        let result = matrix.to_vector2d();
        assert_approx_eq!(result.x, expected.x);
        assert_approx_eq!(result.y, expected.y);
        assert_approx_eq!(result.z, expected.z);
    }
}
