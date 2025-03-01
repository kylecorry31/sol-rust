use super::numbers::{Number, Real};

pub fn round_places<T: Real>(value: T, places: i32) -> T {
    let power = 10.0_f64.powi(places);
    let value_f64 = value.as_f64();
    T::from_f64((value_f64 * power).round() / power)
}

pub fn round_nearest<T: Number>(value: T, nearest: T) -> T {
    let raw = (value.as_f64() / nearest.as_f64()).round() * nearest.as_f64();
    T::from_f64(raw)
}

pub fn power<T: Real>(value: T, power: T) -> T {
    T::from_f64(value.as_f64().powf(power.as_f64()))
}

pub fn integer_power<T: Number>(value: T, power: i32) -> T {
    T::from_f64(value.as_f64().powi(power))
}

pub fn square<T: Number>(value: T) -> T {
    value * value
}

pub fn cube<T: Number>(value: T) -> T {
    value * value * value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // Floor
    #[case(1.1111, 0, 1.0)]
    #[case(1.1111, 1, 1.1)]
    #[case(1.1111, 2, 1.11)]
    #[case(1.1111, 3, 1.111)]
    #[case(1.1111, 4, 1.1111)]
    #[case(1.1111, 5, 1.1111)]
    // Ceil
    #[case(1.6666, 0, 2.0)]
    #[case(1.6666, 1, 1.7)]
    #[case(1.6666, 2, 1.67)]
    #[case(1.6666, 3, 1.667)]
    #[case(1.6666, 4, 1.6666)]
    #[case(1.6666, 5, 1.6666)]
    // Middle
    #[case(1.5555, 0, 2.0)]
    #[case(1.5555, 1, 1.6)]
    #[case(1.5555, 2, 1.56)]
    #[case(1.5555, 3, 1.556)]
    #[case(1.5555, 4, 1.5555)]
    #[case(1.5555, 5, 1.5555)]
    // Negative
    #[case(15.11, -1, 20.0)]
    #[case(15.11, -2, 0.0)]
    #[case(55.11, -2, 100.0)]
    #[case(155.11, -2, 200.0)]
    // Large
    #[case(8000000.0, 5, 8000000.0)]
    #[case(8000000.125555, 5, 8000000.12556)]
    #[case(8000000.125555555555, 8, 8000000.12555556)]
    fn test_round_places(#[case] input: f64, #[case] places: i32, #[case] expected: f64) {
        assert_eq!(round_places(input, places), expected);
    }

    #[rstest]
    #[case(1.0, 1.0, 1.0)]
    #[case(1.5, 1.0, 2.0)]
    #[case(1.4, 1.0, 1.0)]
    #[case(20.0, 10.0, 20.0)]
    #[case(20.0, 15.0, 15.0)]
    #[case(25.0, 15.0, 30.0)]
    #[case(0.0, 1.0, 0.0)]
    #[case(-1.0, 1.0, -1.0)]
    #[case(-1.5, 1.0, -2.0)]
    #[case(-1.6, 1.0, -2.0)]
    fn test_round_nearest_float(#[case] input: f32, #[case] nearest: f32, #[case] expected: f32) {
        assert_eq!(round_nearest(input, nearest), expected);
    }

    #[rstest]
    #[case(1, 1, 1)]
    #[case(2, 1, 2)]
    #[case(0, 2, 0)]
    #[case(3, 2, 4)]
    #[case(20, 10, 20)]
    #[case(20, 16, 16)]
    #[case(25, 16, 32)]
    #[case(-1, 1, -1)]
    #[case(-2, 5, 0)]
    #[case(-3, 5, -5)]
    fn test_round_nearest_int(#[case] input: i32, #[case] nearest: i32, #[case] expected: i32) {
        assert_eq!(round_nearest(input, nearest), expected);
    }

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(2.0, 4.0)]
    #[case(3.0, 9.0)]
    #[case(4.0, 16.0)]
    #[case(-4.0, 16.0)]
    #[case(0.0, 0.0)]
    #[case(0.5, 0.25)]
    fn test_square(#[case] input: f64, #[case] expected: f64) {
        assert_eq!(square(input), expected);
    }

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(2.0, 8.0)]
    #[case(3.0, 27.0)]
    #[case(4.0, 64.0)]
    #[case(-4.0, -64.0)]
    #[case(0.0, 0.0)]
    #[case(0.5, 0.125)]
    fn test_cube(#[case] input: f64, #[case] expected: f64) {
        assert_eq!(cube(input), expected);
    }

    #[rstest]
    #[case(1.0, 2, 1.0)]
    #[case(1.0, 0, 1.0)]
    #[case(1.0, -1, 1.0)]
    #[case(3.0, -1, 1.0/3.0)]
    #[case(3.0, -2, 1.0/9.0)]
    #[case(3.0, 0, 1.0)]
    #[case(3.0, 1, 3.0)]
    #[case(3.0, 2, 9.0)]
    #[case(0.0, 2, 0.0)]
    #[case(-2.0, 2, 4.0)]
    #[case(-2.0, 3, -8.0)]
    #[case(0.5, 2, 0.25)]
    #[case(0.5, -2, 4.0)]
    fn test_integer_power_f32(#[case] value: f32, #[case] power: i32, #[case] expected: f32) {
        assert_eq!(integer_power(value, power), expected);
    }

    #[rstest]
    #[case(1, 2, 1)]
    #[case(1, 0, 1)]
    #[case(1, -1, 1)]
    #[case(3, -1, 0)]
    #[case(3, -2, 0)]
    #[case(3, 0, 1)]
    #[case(3, 1, 3)]
    #[case(3, 2, 9)]
    #[case(-2, 2, 4)]
    #[case(-2, 3, -8)]
    #[case(0, 3, 0)]
    fn test_integer_power_i32(#[case] value: i32, #[case] power: i32, #[case] expected: i32) {
        assert_eq!(integer_power(value, power), expected);
    }

    #[rstest]
    #[case(1.0, 1.0, 1.0)]
    #[case(1.0, 0.0, 1.0)]
    #[case(2.0, 0.5, 1.4142135623730951)]
    #[case(4.0, 0.5, 2.0)]
    #[case(8.0, 1.0/3.0, 2.0)]
    #[case(2.0, 2.0, 4.0)]
    #[case(2.0, 3.0, 8.0)]
    #[case(3.0, 2.0, 9.0)]
    #[case(4.0, 1.5, 8.0)]
    #[case(0.5, 2.0, 0.25)]
    #[case(0.5, -2.0, 4.0)]
    fn test_power(#[case] base: f64, #[case] exponent: f64, #[case] expected: f64) {
        let result = power(base, exponent);
        assert!((result - expected).abs() < 1e-10);
    }
}
