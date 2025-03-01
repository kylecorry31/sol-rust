use super::numbers::Number;

pub fn polynomial<T: Number>(x: T, coefs: &[T]) -> T {
    let mut running_total = T::from_i32(0);
    let mut x_power = T::from_i32(1);
    coefs.iter().for_each(|&coef| {
        running_total += x_power * coef;
        x_power *= x;
    });
    running_total
}

#[cfg(test)]
mod tests {
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
        assert!((result - expected).abs() < 1e-10);
    }
}
