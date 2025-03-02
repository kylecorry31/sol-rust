use super::{RawTensor, Tensor, Vector};

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: RawTensor,
}

impl Tensor for Matrix {
    fn get_tensor(&self) -> &RawTensor {
        &self.data
    }
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn from(tensor: RawTensor) -> Self {
        let rows = tensor.len();
        let cols = tensor.first().map_or(0, |row| row.len());
        Matrix {
            rows,
            cols,
            data: tensor,
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        (0..size).for_each(|i| data[i][i] = 1.0);
        Matrix {
            rows: size,
            cols: size,
            data,
        }
    }

    pub fn to_vector(&self) -> Vector {
        if self.data.len() == 1 {
            // Row vector
            Vector::new_3d(
                self.data[0].first().copied().unwrap_or(0.0),
                self.data[0].get(1).copied().unwrap_or(0.0),
                self.data[0].get(2).copied().unwrap_or(0.0),
            )
        } else {
            // Column vector
            Vector::new_3d(
                self.data
                    .first()
                    .and_then(|row| row.first())
                    .copied()
                    .unwrap_or(0.0),
                self.data
                    .get(1)
                    .and_then(|row| row.first())
                    .copied()
                    .unwrap_or(0.0),
                self.data
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
        let matrix = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let tensor = matrix.get_tensor();
        assert_eq!(matrix.cols, 2);
        assert_eq!(matrix.rows, 2);
        assert_approx_eq!(tensor[0][0], 1.0);
        assert_approx_eq!(tensor[0][1], 2.0);
        assert_approx_eq!(tensor[1][0], 3.0);
        assert_approx_eq!(tensor[1][1], 4.0);
    }

    #[rstest]
    #[case(vec![vec![1.0, 2.0]], Vector::new_3d(1.0, 2.0, 0.0))]
    #[case(vec![vec![1.0], vec![2.0]], Vector::new_3d(1.0, 2.0, 0.0))]
    #[case(vec![vec![1.0, 2.0, 3.0]], Vector::new_3d(1.0, 2.0, 3.0))]
    #[case(vec![vec![1.0], vec![2.0], vec![3.0]], Vector::new_3d(1.0, 2.0, 3.0))]
    fn test_to_vector(#[case] input: RawTensor, #[case] expected: Vector) {
        let matrix = Matrix::from(input);
        let result = matrix.to_vector();
        assert_approx_eq!(result.x, expected.x);
        assert_approx_eq!(result.y, expected.y);
        assert_approx_eq!(result.z, expected.z);
    }

    #[rstest]
    fn test_identity_matrix() {
        let matrix = Matrix::identity(3);
        assert_eq!(matrix.rows, 3);
        assert_eq!(matrix.cols, 3);

        // Check diagonal elements are 1
        assert_approx_eq!(matrix.data[0][0], 1.0);
        assert_approx_eq!(matrix.data[1][1], 1.0);
        assert_approx_eq!(matrix.data[2][2], 1.0);

        // Check non-diagonal elements are 0
        assert_approx_eq!(matrix.data[0][1], 0.0);
        assert_approx_eq!(matrix.data[0][2], 0.0);
        assert_approx_eq!(matrix.data[1][0], 0.0);
        assert_approx_eq!(matrix.data[1][2], 0.0);
        assert_approx_eq!(matrix.data[2][0], 0.0);
        assert_approx_eq!(matrix.data[2][1], 0.0);
    }
}
