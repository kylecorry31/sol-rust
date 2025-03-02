use super::utils::is_approximately_zero;

pub fn polynomial(x: f32, coefs: &[f32]) -> f32 {
    let mut running_total = 0.0;
    let mut x_power = 1.0;
    coefs.iter().for_each(|&coef| {
        running_total += x_power * coef;
        x_power *= x;
    });
    running_total
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Vec<f32> {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        Vec::new()
    } else if is_approximately_zero(discriminant) {
        vec![-b / (2.0 * a)]
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        vec![
            (-b - sqrt_discriminant) / (2.0 * a),
            (-b + sqrt_discriminant) / (2.0 * a),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_eq;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1.0, &[1.0, 2.0, 3.0], 6.0)]
    #[case(2.0, &[1.0, 2.0, 3.0], 17.0)]
    #[case(3.0, &[0.0, 1.0, 3.0, 1.0], 57.0)]
    #[case(3.0, &[0.0, 1.0, -3.0, 1.0], 3.0)]
    #[case(3.0, &[], 0.0)]
    #[case(3.0, &[1.0], 1.0)]
    fn test_polynomial(#[case] x: f32, #[case] coefs: &[f32], #[case] expected: f32) {
        let result = polynomial(x, coefs);
        assert_approx_eq!(expected, result);
    }

    #[rstest]
    #[case(1.0, 2.0, 1.0, vec![-1.0])]
    #[case(1.0, 0.0, -1.0, vec![-1.0, 1.0])]
    #[case(1.0, 0.0, 0.0, vec![0.0])]
    #[case(1.0, 0.0, 1.0, vec![])]
    fn test_solve_quadratic(
        #[case] a: f32,
        #[case] b: f32,
        #[case] c: f32,
        #[case] expected: Vec<f32>,
    ) {
        let mut result = solve_quadratic(a, b, c);
        result.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(expected, result);
    }
}
