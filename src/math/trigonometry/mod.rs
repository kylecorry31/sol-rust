pub fn sin_degrees(degrees: f32) -> f32 {
    degrees.to_radians().sin()
}

pub fn cos_degrees(degrees: f32) -> f32 {
    degrees.to_radians().cos()
}

pub fn tan_degrees(degrees: f32) -> f32 {
    degrees.to_radians().tan()
}

pub fn asin_degrees(value: f32) -> f32 {
    value.asin().to_degrees()
}

pub fn acos_degrees(value: f32) -> f32 {
    value.acos().to_degrees()
}

pub fn atan_degrees(value: f32) -> f32 {
    value.atan().to_degrees()
}

pub fn atan2_degrees(y: f32, x: f32) -> f32 {
    y.atan2(x).to_degrees()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(30.0, 0.5)]
    #[case(90.0, 1.0)]
    #[case(45.0, 0.7071)]
    fn test_sin_degrees(#[case] input: f32, #[case] expected: f32) {
        assert!((sin_degrees(input) - expected).abs() < 0.0001);
    }

    #[rstest]
    #[case(0.0, 1.0)]
    #[case(60.0, 0.5)]
    #[case(45.0, 0.7071)]
    fn test_cos_degrees(#[case] input: f32, #[case] expected: f32) {
        assert!((cos_degrees(input) - expected).abs() < 0.0001);
    }

    #[rstest]
    #[case(45.0, 1.0)]
    #[case(0.0, 0.0)]
    fn test_tan_degrees(#[case] input: f32, #[case] expected: f32) {
        assert!((tan_degrees(input) - expected).abs() < 0.0001);
    }

    #[rstest]
    #[case(1.0, 90.0)]
    #[case(0.0, 0.0)]
    #[case(0.5, 30.0)]
    fn test_asin_degrees(#[case] input: f32, #[case] expected: f32) {
        assert!((asin_degrees(input) - expected).abs() < 0.0001);
    }

    #[rstest]
    #[case(1.0, 0.0)]
    #[case(0.5, 60.0)]
    #[case(0.0, 90.0)]
    fn test_acos_degrees(#[case] input: f32, #[case] expected: f32) {
        assert!((acos_degrees(input) - expected).abs() < 0.0001);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(1.0, 45.0)]
    fn test_atan_degrees(#[case] input: f32, #[case] expected: f32) {
        assert!((atan_degrees(input) - expected).abs() < 0.0001);
    }

    #[rstest]
    #[case(1.0, 1.0, 45.0)]
    #[case(0.0, 1.0, 0.0)]
    #[case(1.0, 0.0, 90.0)]
    fn test_atan2_degrees(#[case] y: f32, #[case] x: f32, #[case] expected: f32) {
        assert!((atan2_degrees(y, x) - expected).abs() < 0.0001);
    }
}
