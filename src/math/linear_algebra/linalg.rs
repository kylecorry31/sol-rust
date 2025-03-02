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
    combine(tensor1, tensor2, |x1, x2| x1 * x2)
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
    if tensor1.len() != tensor2.len() || tensor1[0].len() != tensor2[0].len() {
        panic!("Tensors must have the same dimensions");
    }

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

pub fn inverse(tensor: &RawTensor) -> RawTensor {
    if rows(tensor) != columns(tensor) {
        panic!("Matrix must be square to calculate inverse");
    }

    let det = determinant(tensor);
    if det == 0.0 {
        // No inverse exists
        return create_tensor(rows(tensor), columns(tensor));
    }

    let adj = adjugate(tensor);
    let trans = transpose(&adj);
    let scalar = 1.0 / det;
    map(&trans, |x| x * scalar)
}

pub fn adjugate(tensor: &RawTensor) -> RawTensor {
    if rows(tensor) != columns(tensor) {
        panic!("Matrix must be square to adjugate");
    }

    let mut result = create_tensor(rows(tensor), columns(tensor));

    for r in 0..rows(tensor) {
        for c in 0..columns(tensor) {
            let row_multiplier = if r % 2 == 0 { 1.0 } else { -1.0 };
            let col_multiplier = if c % 2 == 0 { 1.0 } else { -1.0 };
            let cofactor_det = determinant(&cofactor(tensor, r, c));
            result[r][c] = cofactor_det * col_multiplier * row_multiplier;
        }
    }

    result
}

pub fn determinant(tensor: &RawTensor) -> f32 {
    if rows(tensor) != columns(tensor) {
        panic!("Matrix must be square to calculate determinant");
    }

    if rows(tensor) == 1 && columns(tensor) == 1 {
        tensor[0][0]
    } else if rows(tensor) == 2 && columns(tensor) == 2 {
        tensor[0][0] * tensor[1][1] - tensor[0][1] * tensor[1][0]
    } else {
        let mut multiplier = 1.0;
        let mut sum = 0.0;
        for c in 0..columns(tensor) {
            sum += tensor[0][c] * determinant(&cofactor(tensor, 0, c)) * multiplier;
            multiplier *= -1.0;
        }
        sum
    }
}

pub fn cofactor(tensor: &RawTensor, r: usize, c: usize) -> RawTensor {
    let mut result = create_tensor(rows(tensor) - 1, columns(tensor) - 1);

    for r1 in 0..rows(tensor) - 1 {
        for c1 in 0..columns(tensor) - 1 {
            let sr = if r1 < r { r1 } else { r1 + 1 };
            let sc = if c1 < c { c1 } else { c1 + 1 };
            result[r1][c1] = tensor[sr][sc];
        }
    }

    result
}

pub fn solve_linear(tensor: &RawTensor, vector: &RawTensor) -> RawTensor {
    if rows(tensor) != columns(tensor) {
        panic!("Matrix must be square");
    }
    if rows(tensor) != rows(vector) {
        panic!("Matrix rows must be the same size as the vector");
    }

    let n = columns(tensor);
    let mut augmented = Vec::new();
    for (row, b) in tensor.iter().zip(vector.iter()) {
        let mut new_row = row.clone();
        new_row.extend(b);
        augmented.push(new_row);
    }

    // Convert to row echelon form
    for i in 0..n {
        let mut max_row = i;
        for j in i + 1..n {
            if augmented[j][i].abs() > augmented[max_row][i].abs() {
                max_row = j;
            }
        }

        augmented.swap(i, max_row);

        for j in i + 1..n {
            let factor = augmented[j][i] / augmented[i][i];
            for k in i..n + 1 {
                augmented[j][k] -= augmented[i][k] * factor;
            }
        }
    }

    // Back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = augmented[i][n] / augmented[i][i];
        for j in (0..i).rev() {
            augmented[j][n] -= augmented[j][i] * x[i];
        }
    }

    vec![x]
}

pub fn least_norm(tensor: &RawTensor, vector: &[f32]) -> Vec<f32> {
    let q_r = qr(&transpose(tensor));
    let q = q_r.0;
    let r = q_r.1;

    let r_inv = inverse(&r);
    let r_inv_t = transpose(&r_inv);
    let q_r_inv_t = dot(&q, &r_inv_t);

    let b_matrix = transpose(&vec![vector.to_vec()]);
    let y = dot(&q_r_inv_t, &b_matrix);

    transpose(&y)[0].clone()
}

pub fn least_squares(tensor: &RawTensor, vector: &[f32]) -> Vec<f32> {
    let is_underdetermined = rows(tensor) < columns(tensor);

    if is_underdetermined {
        return least_norm(tensor, vector);
    }

    let jt = transpose(tensor);
    let jtj = dot(&jt, tensor);
    let jtr = dot(&jt, &transpose(&vec![vector.to_vec()]));

    solve_linear(&jtj, &jtr)[0].clone()
}

pub fn qr(tensor: &RawTensor) -> (RawTensor, RawTensor) {
    let r = rows(tensor);
    let c = columns(tensor);

    let mut q = create_tensor(r, c);
    let mut r_mat = create_tensor(c, c);

    for j in 0..c {
        let mut v = Vec::new();
        for i in 0..r {
            v.push(vec![tensor[i][j]]);
        }

        for i in 0..j {
            let mut qi = Vec::new();
            for k in 0..r {
                qi.push(vec![q[k][i]]);
            }

            r_mat[i][j] = dot(&transpose(&qi), &v)[0][0];

            let qi_scaled = map(&qi, |x| x * r_mat[i][j]);
            v = subtract(&v, &qi_scaled);
        }

        r_mat[j][j] = magnitude(&v);
        let v_normalized = map(&v, |x| x / r_mat[j][j]);

        for i in 0..r {
            q[i][j] = v_normalized[i][0];
        }
    }

    (q, r_mat)
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

    #[rstest]
    #[case(&vec![vec![2.0, -3.0, 1.0], vec![2.0, 0.0, -1.0], vec![1.0, 4.0, 5.0]], 49.0)]
    #[case(&vec![vec![1.0, 2.0], vec![3.0, 4.0]], -2.0)]
    fn test_determinant(#[case] tensor: &RawTensor, #[case] expected: f32) {
        assert_approx_eq!(determinant(tensor), expected);
    }

    #[rstest]
    #[case(
        &vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        vec![vec![-2.0, 1.0], vec![1.5, -0.5]]
    )]
    #[case(
        &vec![vec![1.0, 2.0, 3.0], vec![0.0, 1.0, 4.0], vec![5.0, 6.0, 0.0]],
        vec![vec![-24.0, 18.0, 5.0], vec![20.0, -15.0, -4.0], vec![-5.0, 4.0, 1.0]]
    )]
    fn test_inverse(#[case] tensor: &RawTensor, #[case] expected: RawTensor) {
        assert_tensor_eq!(inverse(tensor), expected);
    }

    #[rstest]
    fn test_solve_linear() {
        let test1_tensor = vec![vec![2.0, 1.0], vec![1.0, -1.0]];
        let test1_vector = vec![vec![-4.0], vec![-2.0]];
        let test1_expected = vec![vec![-2.0, 0.0]];
        assert_tensor_eq!(solve_linear(&test1_tensor, &test1_vector), test1_expected);

        let test2_tensor = vec![
            vec![2.0, -5.0, 3.0],
            vec![3.0, -1.0, 4.0],
            vec![1.0, 3.0, 2.0],
        ];
        let test2_vector = vec![vec![8.0], vec![7.0], vec![-3.0]];
        let test2_expected = vec![vec![6.0, -1.0, -3.0]];
        assert_tensor_eq!(solve_linear(&test2_tensor, &test2_vector), test2_expected);
    }

    #[rstest]
    fn test_least_squares() {
        // Well conditioned
        let a1 = vec![vec![2.0, 1.0], vec![1.0, -1.0]];
        let b1 = vec![7.0, -1.0];
        let expected1 = vec![2.0, 3.0];
        let actual1 = least_squares(&a1, &b1);
        assert_eq!(2, actual1.len());
        assert_approx_eq!(actual1[0], expected1[0]);
        assert_approx_eq!(actual1[1], expected1[1]);

        // Overdetermined
        let a2 = vec![vec![1.0, 1.0], vec![1.0, -1.0], vec![1.0, 0.0]];
        let b2 = vec![3.0, 1.0, 2.0];
        let expected2 = vec![2.0, 1.0];
        let actual2 = least_squares(&a2, &b2);
        assert_eq!(2, actual2.len());
        assert_approx_eq!(actual2[0], expected2[0]);
        assert_approx_eq!(actual2[1], expected2[1]);

        // Underdetermined
        let a3 = vec![vec![1.0, 1.0, 1.0], vec![0.0, 1.0, 2.0]];
        let b3 = vec![6.0, 5.0];
        let expected3 = vec![2.5, 2.0, 1.5];
        let actual3 = least_squares(&a3, &b3);
        assert_eq!(3, actual3.len());
        assert_approx_eq!(actual3[0], expected3[0]);
        assert_approx_eq!(actual3[1], expected3[1]);
        assert_approx_eq!(actual3[2], expected3[2]);
    }
}
