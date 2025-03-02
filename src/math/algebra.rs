pub fn polynomial(x: f64, coefs: &[f64]) -> f64 {
    let mut running_total = 0.0;
    let mut x_power = 1.0;
    coefs.iter().for_each(|&coef| {
        running_total += x_power * coef;
        x_power *= x;
    });
    running_total
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
    fn test_polynomial(#[case] x: f64, #[case] coefs: &[f64], #[case] expected: f64) {
        let result = polynomial(x, coefs);
        assert_approx_eq!(expected, result, 1e-10);
    }
}
