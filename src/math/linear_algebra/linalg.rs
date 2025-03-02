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

pub fn transpose(tensor: &RawTensor) -> RawTensor {
    let mut result = Vec::new();

    for j in 0..tensor[0].len() {
        let mut row = Vec::new();
        for item in tensor {
            row.push(item[j]);
        }
        result.push(row);
    }

    if result.is_empty() {
        result.push(vec![]);
    }

    result
}

pub fn multiply(tensor1: &RawTensor, tensor2: &RawTensor) -> RawTensor {
    let mut result = Vec::new();

    for i in 0..tensor1.len() {
        let mut row = Vec::new();
        for j in 0..tensor2[0].len() {
            let mut sum = 0.0;
            for (k, _) in tensor1[0].iter().enumerate() {
                sum += tensor1[i][k] * tensor2[k][j];
            }
            row.push(sum);
        }
        result.push(row);
    }

    result
}

pub fn divide(tensor1: &RawTensor, tensor2: &RawTensor) -> RawTensor {
    let mut result = Vec::new();

    for (row1, row2) in tensor1.iter().zip(tensor2) {
        let mut divided_row = Vec::new();
        for (element1, element2) in row1.iter().zip(row2) {
            divided_row.push(element1 / element2);
        }
        result.push(divided_row);
    }

    result
}

pub fn apply(tensor: &RawTensor, operation: impl Fn(f32) -> f32) -> RawTensor {
    let mut result = Vec::new();

    for row in tensor {
        let mut transformed_row = Vec::new();
        for element in row {
            transformed_row.push(operation(*element));
        }
        result.push(transformed_row);
    }

    result
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

    #[rstest]
    #[case(&vec![vec![42.0]], vec![vec![42.0]])]
    #[case(&vec![vec![1.0], vec![2.0]], vec![vec![1.0, 2.0]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], vec![vec![1.0, 3.0], vec![2.0, 4.0]])]
    #[case(&vec![vec![]], vec![vec![]])]
    fn test_transpose(#[case] tensor: &RawTensor, #[case] expected: RawTensor) {
        assert_tensor_eq!(transpose(tensor), expected);
    }

    #[rstest]
    #[case(&vec![vec![42.0]], &vec![vec![2.0]], vec![vec![84.0]])]
    #[case(&vec![vec![1.0], vec![2.0]], &vec![vec![3.0, 4.0]], vec![vec![3.0, 4.0], vec![6.0, 8.0]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], &vec![vec![5.0, 6.0], vec![7.0, 8.0]], vec![vec![19.0, 22.0], vec![43.0, 50.0]])]
    #[case(&vec![vec![]], &vec![vec![]], vec![vec![]])]
    fn test_multiply(
        #[case] tensor1: &RawTensor,
        #[case] tensor2: &RawTensor,
        #[case] expected: RawTensor,
    ) {
        assert_tensor_eq!(multiply(tensor1, tensor2), expected);
    }

    #[rstest]
    #[case(&vec![vec![42.0]], 2.0, vec![vec![84.0]])]
    #[case(&vec![vec![1.0], vec![2.0]], 3.0, vec![vec![3.0], vec![6.0]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], 2.0, vec![vec![2.0, 4.0], vec![6.0, 8.0]])]
    #[case(&vec![vec![]], 2.0, vec![vec![]])]
    fn test_operation(
        #[case] tensor: &RawTensor,
        #[case] scalar: f32,
        #[case] expected: RawTensor,
    ) {
        assert_tensor_eq!(apply(tensor, |x| x * scalar), expected);
    }

    #[rstest]
    #[case(&vec![vec![42.0]], &vec![vec![2.0]], vec![vec![21.0]])]
    #[case(&vec![vec![1.0], vec![2.0]], &vec![vec![3.0], vec![4.0]], vec![vec![0.3333333333333333], vec![0.5]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], &vec![vec![5.0, 6.0], vec![7.0, 8.0]], vec![vec![0.2, 0.3333333333333333], vec![0.42857142857142855, 0.5]])]
    #[case(&vec![vec![]], &vec![vec![]], vec![vec![]])]
    fn test_divide(
        #[case] tensor1: &RawTensor,
        #[case] tensor2: &RawTensor,
        #[case] expected: RawTensor,
    ) {
        assert_tensor_eq!(divide(tensor1, tensor2), expected);
    }
}
