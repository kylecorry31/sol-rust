pub type RawTensor = Vec<Vec<f32>>;

pub fn dot(tensor1: &RawTensor, tensor2: &RawTensor) -> f32 {
    let mut result = 0.0;

    for i in 0..tensor1.len() {
        for j in 0..tensor1[i].len() {
            result += tensor1[i][j] * tensor2[i][j];
        }
    }

    result
}

pub fn add(tensor1: &RawTensor, tensor2: &RawTensor) -> RawTensor {
    let mut result = Vec::new();

    for i in 0..tensor1.len() {
        let mut row = Vec::new();
        for j in 0..tensor1[i].len() {
            row.push(tensor1[i][j] + tensor2[i][j]);
        }
        result.push(row);
    }

    result
}

pub fn subtract(tensor1: &RawTensor, tensor2: &RawTensor) -> RawTensor {
    let mut result = Vec::new();

    for i in 0..tensor1.len() {
        let mut row = Vec::new();
        for j in 0..tensor1[i].len() {
            row.push(tensor1[i][j] - tensor2[i][j]);
        }
        result.push(row);
    }

    result
}

pub fn cross(tensor1: &RawTensor, tensor2: &RawTensor) -> RawTensor {
    let mut result = Vec::new();

    for i in 0..tensor1.len() {
        let mut row = Vec::new();
        for j in 0..tensor1[i].len() {
            row.push(tensor1[i][j] * tensor2[i][j]);
        }
        result.push(row);
    }

    result
}

pub fn magnitude(tensor: &RawTensor) -> f32 {
    // Using f64 to retain intermediate precision
    let mut sum: f64 = 0.0;

    for row in tensor {
        for &value in row {
            sum += (value as f64).powi(2);
        }
    }

    sum.sqrt() as f32
}

#[cfg(test)]
mod tests {
    // test for 1 element, vec 2, 2x2, empty

    use crate::{assert_approx_eq, assert_tensor_eq};

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&vec![vec![42.0]], &vec![vec![2.0]], 84.0)]
    #[case(&vec![vec![1.0], vec![2.0]], &vec![vec![3.0], vec![4.0]], 11.0)]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], &vec![vec![5.0, 6.0], vec![7.0, 8.0]], 70.0)]
    #[case(&vec![vec![]], &vec![vec![]], 0.0)]
    fn test_dot(#[case] tensor1: &RawTensor, #[case] tensor2: &RawTensor, #[case] expected: f32) {
        assert_approx_eq!(dot(tensor1, tensor2), expected);
    }

    #[rstest]
    #[case(&vec![vec![42.0]], &vec![vec![2.0]], vec![vec![44.0]])]
    #[case(&vec![vec![1.0], vec![2.0]], &vec![vec![3.0], vec![4.0]], vec![vec![4.0], vec![6.0]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], &vec![vec![5.0, 6.0], vec![7.0, 8.0]], vec![vec![6.0, 8.0], vec![10.0, 12.0]])]
    #[case(&vec![vec![]], &vec![vec![]], vec![vec![]])]
    fn test_add(
        #[case] tensor1: &RawTensor,
        #[case] tensor2: &RawTensor,
        #[case] expected: RawTensor,
    ) {
        assert_tensor_eq!(add(tensor1, tensor2), expected);
    }

    #[rstest]
    #[case(&vec![vec![42.0]], &vec![vec![2.0]], vec![vec![40.0]])]
    #[case(&vec![vec![1.0], vec![2.0]], &vec![vec![3.0], vec![4.0]], vec![vec![-2.0], vec![-2.0]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], &vec![vec![5.0, 6.0], vec![7.0, 8.0]], vec![vec![-4.0, -4.0], vec![-4.0, -4.0]])]
    #[case(&vec![vec![]], &vec![vec![]], vec![vec![]])]
    fn test_subtract(
        #[case] tensor1: &RawTensor,
        #[case] tensor2: &RawTensor,
        #[case] expected: RawTensor,
    ) {
        assert_tensor_eq!(subtract(tensor1, tensor2), expected);
    }

    #[rstest]
    #[case(&vec![vec![42.0]], &vec![vec![2.0]], vec![vec![84.0]])]
    #[case(&vec![vec![1.0], vec![2.0]], &vec![vec![3.0], vec![4.0]], vec![vec![3.0], vec![8.0]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], &vec![vec![5.0, 6.0], vec![7.0, 8.0]], vec![vec![5.0, 12.0], vec![21.0, 32.0]])]
    #[case(&vec![vec![]], &vec![vec![]], vec![vec![]])]
    fn test_cross(
        #[case] tensor1: &RawTensor,
        #[case] tensor2: &RawTensor,
        #[case] expected: RawTensor,
    ) {
        assert_tensor_eq!(cross(tensor1, tensor2), expected);
    }

    #[rstest]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], 5.477225575051661)]
    #[case(&vec![vec![1.0], vec![2.0]], 2.23606797749979)]
    #[case(&vec![vec![42.0]], 42.0)]
    #[case(&vec![vec![]], 0.0)]
    fn test_magnitude(#[case] tensor: &RawTensor, #[case] expected: f32) {
        assert_approx_eq!(magnitude(tensor), expected);
    }
}
