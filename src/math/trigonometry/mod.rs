use crate::units::{
    angle::Angle,
    quantity::{Convertable, Quantity},
};

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

// Trig functions that operator on Quantity<Angle>
pub fn sin(angle: Quantity<Angle>) -> f32 {
    angle.convert(Angle::Radians).amount.sin()
}

pub fn cos(angle: Quantity<Angle>) -> f32 {
    angle.convert(Angle::Radians).amount.cos()
}

pub fn tan(angle: Quantity<Angle>) -> f32 {
    angle.convert(Angle::Radians).amount.tan()
}

pub fn asin(value: f32) -> Quantity<Angle> {
    Quantity::new(value.asin(), Angle::Radians)
}

pub fn acos(value: f32) -> Quantity<Angle> {
    Quantity::new(value.acos(), Angle::Radians)
}

pub fn atan(value: f32) -> Quantity<Angle> {
    Quantity::new(value.atan(), Angle::Radians)
}

pub fn atan2(y: f32, x: f32) -> Quantity<Angle> {
    Quantity::new(y.atan2(x), Angle::Radians)
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_eq;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(30.0, 0.5)]
    #[case(90.0, 1.0)]
    #[case(45.0, 0.7071)]
    fn test_sin_degrees(#[case] input: f32, #[case] expected: f32) {
        assert_approx_eq!(sin_degrees(input), expected);
    }

    #[rstest]
    #[case(0.0, 1.0)]
    #[case(60.0, 0.5)]
    #[case(45.0, 0.7071)]
    fn test_cos_degrees(#[case] input: f32, #[case] expected: f32) {
        assert_approx_eq!(cos_degrees(input), expected);
    }

    #[rstest]
    #[case(45.0, 1.0)]
    #[case(0.0, 0.0)]
    fn test_tan_degrees(#[case] input: f32, #[case] expected: f32) {
        assert_approx_eq!(tan_degrees(input), expected);
    }

    #[rstest]
    #[case(1.0, 90.0)]
    #[case(0.0, 0.0)]
    #[case(0.5, 30.0)]
    fn test_asin_degrees(#[case] input: f32, #[case] expected: f32) {
        assert_approx_eq!(asin_degrees(input), expected);
    }

    #[rstest]
    #[case(1.0, 0.0)]
    #[case(0.5, 60.0)]
    #[case(0.0, 90.0)]
    fn test_acos_degrees(#[case] input: f32, #[case] expected: f32) {
        assert_approx_eq!(acos_degrees(input), expected);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(1.0, 45.0)]
    #[case(0.5, 26.565052)]
    fn test_atan_degrees(#[case] input: f32, #[case] expected: f32) {
        assert_approx_eq!(atan_degrees(input), expected);
    }

    #[rstest]
    #[case(1.0, 1.0, 45.0)]
    #[case(0.0, 1.0, 0.0)]
    #[case(1.0, 0.0, 90.0)]
    fn test_atan2_degrees(#[case] y: f32, #[case] x: f32, #[case] expected: f32) {
        assert_approx_eq!(atan2_degrees(y, x), expected);
    }

    #[rstest]
    #[case(30.0, 0.5)]
    #[case(90.0, 1.0)]
    #[case(45.0, 0.7071)]
    fn test_sin_quantity(#[case] input: f32, #[case] expected: f32) {
        let angle = Quantity::new(input, Angle::Degrees);
        assert_approx_eq!(sin(angle), expected);
    }

    #[rstest]
    #[case(0.0, 1.0)]
    #[case(60.0, 0.5)]
    #[case(45.0, 0.7071)]
    fn test_cos_quantity(#[case] input: f32, #[case] expected: f32) {
        let angle = Quantity::new(input, Angle::Degrees);
        assert_approx_eq!(cos(angle), expected);
    }

    #[rstest]
    #[case(45.0, 1.0)]
    #[case(0.0, 0.0)]
    fn test_tan_quantity(#[case] input: f32, #[case] expected: f32) {
        let angle = Quantity::new(input, Angle::Degrees);
        assert_approx_eq!(tan(angle), expected);
    }

    #[rstest]
    #[case(1.0, 90.0)]
    #[case(0.0, 0.0)]
    #[case(0.5, 30.0)]
    fn test_asin_quantity(#[case] input: f32, #[case] expected: f32) {
        let result = asin(input).convert(Angle::Degrees);
        assert_approx_eq!(result.amount, expected);
    }

    #[rstest]
    #[case(1.0, 0.0)]
    #[case(0.5, 60.0)]
    #[case(0.0, 90.0)]
    fn test_acos_quantity(#[case] input: f32, #[case] expected: f32) {
        let result = acos(input).convert(Angle::Degrees);
        assert_approx_eq!(result.amount, expected);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(1.0, 45.0)]
    #[case(0.5, 26.565052)]
    fn test_atan_quantity(#[case] input: f32, #[case] expected: f32) {
        let result = atan(input).convert(Angle::Degrees);
        assert_approx_eq!(result.amount, expected);
    }

    #[rstest]
    #[case(1.0, 1.0, 45.0)]
    #[case(0.0, 1.0, 0.0)]
    #[case(1.0, 0.0, 90.0)]
    fn test_atan2_quantity(#[case] y: f32, #[case] x: f32, #[case] expected: f32) {
        let result = atan2(y, x).convert(Angle::Degrees);
        assert_approx_eq!(result.amount, expected);
    }
}
