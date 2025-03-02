pub type RawTensor = Vec<Vec<f32>>;

fn rows(tensor: &RawTensor) -> usize {
    tensor.len()
}

fn columns(tensor: &RawTensor) -> usize {
    tensor[0].len()
}

fn is_empty(tensor: &RawTensor) -> bool {
    rows(tensor) == 0 || tensor.iter().all(|row| row.is_empty())
}

fn empty() -> RawTensor {
    vec![vec![]]
}

fn create_tensor(rows: usize, columns: usize) -> RawTensor {
    vec![vec![0.0; columns]; rows]
}

pub fn dot(tensor1: &RawTensor, tensor2: &RawTensor) -> RawTensor {
    if is_empty(tensor1) || is_empty(tensor2) {
        return empty();
    }

    if columns(tensor1) != rows(tensor2) {
        panic!("Matrix 1 columns must be the same size as matrix 2 rows");
    }

    let mut product = create_tensor(rows(tensor1), columns(tensor2));

    for (r, product_row) in product.iter_mut().enumerate() {
        for (other_c, product_value) in product_row.iter_mut().enumerate() {
            let mut sum = 0.0;
            for c in 0..columns(tensor1) {
                sum += tensor1[r][c] * tensor2[c][other_c];
            }
            *product_value = sum;
        }
    }

    product
}

pub fn add(tensor1: &RawTensor, tensor2: &RawTensor) -> RawTensor {
    combine(tensor1, tensor2, |x1, x2| x1 + x2)
}

pub fn subtract(tensor1: &RawTensor, tensor2: &RawTensor) -> RawTensor {
    combine(tensor1, tensor2, |x1, x2| x1 - x2)
}

pub fn cross(tensor1: &RawTensor, tensor2: &RawTensor) -> RawTensor {
    combine(tensor1, tensor2, |x1, x2| x1 * x2)
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
    cross(tensor1, tensor2)
}

pub fn divide(tensor1: &RawTensor, tensor2: &RawTensor) -> RawTensor {
    combine(tensor1, tensor2, |x1, x2| x1 / x2)
}

pub fn map(tensor: &RawTensor, operation: impl Fn(f32) -> f32) -> RawTensor {
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

pub fn combine(
    tensor1: &RawTensor,
    tensor2: &RawTensor,
    operation: impl Fn(f32, f32) -> f32,
) -> RawTensor {
    let mut result = Vec::new();

    for (row1, row2) in tensor1.iter().zip(tensor2) {
        let mut combined_row = Vec::new();
        for (element1, element2) in row1.iter().zip(row2) {
            combined_row.push(operation(*element1, *element2));
        }
        result.push(combined_row);
    }

    result
}

pub fn aggregate(
    tensor: &RawTensor,
    initial_value: f32,
    operation: impl Fn(f32, f32) -> f32,
) -> f32 {
    let mut result = initial_value;
    for row in tensor {
        for element in row {
            result = operation(result, *element);
        }
    }
    result
}

pub fn aggregate_rows(
    tensor: &RawTensor,
    initial_value: f32,
    operation: impl Fn(f32, f32) -> f32,
) -> RawTensor {
    if is_empty(tensor) {
        return empty();
    }

    let mut result = Vec::new();
    for row in tensor {
        let mut row_result = initial_value;
        for element in row {
            row_result = operation(row_result, *element);
        }
        result.push(vec![row_result]);
    }
    result
}

pub fn aggregate_columns(
    tensor: &RawTensor,
    initial_value: f32,
    operation: impl Fn(f32, f32) -> f32,
) -> RawTensor {
    if is_empty(tensor) {
        return empty();
    }

    let mut result = Vec::new();
    for col_idx in 0..tensor[0].len() {
        let mut col_result = initial_value;
        for row in tensor {
            col_result = operation(col_result, row[col_idx]);
        }
        result.push(vec![col_result]);
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
    #[case(&vec![vec![42.0]], &vec![vec![2.0]], vec![vec![84.0]])]
    #[case(&vec![vec![1.0], vec![2.0]], &vec![vec![3.0, 4.0]], vec![vec![3.0, 4.0], vec![6.0, 8.0]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], &vec![vec![5.0, 6.0], vec![7.0, 8.0]], vec![vec![19.0, 22.0], vec![43.0, 50.0]])]
    #[case(&vec![vec![]], &vec![vec![]], vec![vec![]])]
    fn test_dot(
        #[case] tensor1: &RawTensor,
        #[case] tensor2: &RawTensor,
        #[case] expected: RawTensor,
    ) {
        assert_tensor_eq!(dot(tensor1, tensor2), expected);
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
    #[case(&vec![vec![1.0], vec![2.0]], &vec![vec![3.0], vec![4.0]], vec![vec![3.0], vec![8.0]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], &vec![vec![5.0, 6.0], vec![7.0, 8.0]], vec![vec![5.0, 12.0], vec![21.0, 32.0]])]
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
    fn test_map(#[case] tensor: &RawTensor, #[case] scalar: f32, #[case] expected: RawTensor) {
        assert_tensor_eq!(map(tensor, |x| x * scalar), expected);
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

    #[rstest]
    #[case(&vec![vec![42.0]], 0.0, vec![vec![42.0]])]
    #[case(&vec![vec![1.0], vec![2.0]], 0.0, vec![vec![1.0], vec![2.0]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], 0.0, vec![vec![3.0], vec![7.0]])]
    #[case(&vec![vec![]], 0.0, vec![vec![]])]
    fn test_aggregate_rows(
        #[case] tensor: &RawTensor,
        #[case] initial: f32,
        #[case] expected: RawTensor,
    ) {
        assert_tensor_eq!(aggregate_rows(tensor, initial, |acc, x| acc + x), expected);
    }

    #[rstest]
    #[case(&vec![vec![42.0]], 0.0, vec![vec![42.0]])]
    #[case(&vec![vec![1.0], vec![2.0]], 0.0, vec![vec![3.0]])]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], 0.0, vec![vec![4.0], vec![6.0]])]
    #[case(&vec![vec![]], 0.0, vec![vec![]])]
    fn test_aggregate_columns(
        #[case] tensor: &RawTensor,
        #[case] initial: f32,
        #[case] expected: RawTensor,
    ) {
        assert_tensor_eq!(
            aggregate_columns(tensor, initial, |acc, x| acc + x),
            expected
        );
    }

    #[rstest]
    #[case(&vec![vec![42.0]], 0.0, 42.0)]
    #[case(&vec![vec![1.0], vec![2.0]], 0.0, 3.0)]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], 0.0, 10.0)]
    #[case(&vec![vec![]], 0.0, 0.0)]
    fn test_aggregate(#[case] tensor: &RawTensor, #[case] initial: f32, #[case] expected: f32) {
        assert_approx_eq!(aggregate(tensor, initial, |acc, x| acc + x), expected);
    }
}
