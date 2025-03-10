use super::utils::is_approximately_zero;

pub fn round_places(value: f32, places: i32) -> f32 {
    let power = 10.0_f32.powi(places);
    (value * power).round() / power
}

pub fn round_nearest(value: f32, nearest: f32) -> f32 {
    (value / nearest).round() * nearest
}

pub fn power(value: f32, power: f32) -> f32 {
    value.powf(power)
}

pub fn integer_power(value: f32, power: i32) -> f32 {
    value.powi(power)
}

pub fn square(value: f32) -> f32 {
    value * value
}

pub fn cube(value: f32) -> f32 {
    value * value * value
}

pub fn greatest_common_divisor(a: f32, b: f32, precision: Option<f32>) -> f32 {
    let max_iterations = 1000;
    let precision = precision.unwrap_or(0.0001);
    let mut current_a = a;
    let mut current_b = b;
    let mut iterations = 0;

    while current_b.abs() > precision && iterations < max_iterations {
        let temp = current_b;
        current_b = current_a % current_b;
        current_a = temp;
        iterations += 1;
    }

    current_a
}

pub fn least_common_multiple(a: f32, b: f32) -> f32 {
    if is_approximately_zero(a) || is_approximately_zero(b) {
        return 0.0;
    }
    a.abs() * (b.abs() / greatest_common_divisor(a, b, None))
}

pub fn is_increasing(arr: &[f32]) -> bool {
    let mut has_greater = false;

    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }

        if arr[i] > arr[i - 1] {
            has_greater = true;
        }
    }
    has_greater
}

pub fn is_decreasing(arr: &[f32]) -> bool {
    let mut has_lesser = false;

    for i in 1..arr.len() {
        if arr[i] > arr[i - 1] {
            return false;
        }

        if arr[i] < arr[i - 1] {
            has_lesser = true;
        }
    }
    has_lesser
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_eq;

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
    fn test_round_places(#[case] input: f32, #[case] places: i32, #[case] expected: f32) {
        let result = round_places(input, places);
        assert_approx_eq!(expected, result);
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
        let result = round_nearest(input, nearest);
        assert_approx_eq!(expected, result);
    }

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(2.0, 4.0)]
    #[case(3.0, 9.0)]
    #[case(4.0, 16.0)]
    #[case(-4.0, 16.0)]
    #[case(0.0, 0.0)]
    #[case(0.5, 0.25)]
    fn test_square(#[case] input: f32, #[case] expected: f32) {
        let result = square(input);
        assert_approx_eq!(expected, result);
    }

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(2.0, 8.0)]
    #[case(3.0, 27.0)]
    #[case(4.0, 64.0)]
    #[case(-4.0, -64.0)]
    #[case(0.0, 0.0)]
    #[case(0.5, 0.125)]
    fn test_cube(#[case] input: f32, #[case] expected: f32) {
        let result = cube(input);
        assert_approx_eq!(expected, result);
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
        let result = integer_power(value, power);
        assert_approx_eq!(expected, result);
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
    fn test_power(#[case] base: f32, #[case] exponent: f32, #[case] expected: f32) {
        let result = power(base, exponent);
        assert_approx_eq!(expected, result);
    }

    #[rstest]
    #[case(1.0, 1.0, 1.0)]
    #[case(1.5, 1.0, 0.5)]
    #[case(1.4, 1.0, 0.2)]
    #[case(0.01, 3.8, 0.01)]
    #[case(0.0, 1.0, 1.0)]
    #[case(1.0, 0.0, 1.0)]
    #[case(0.0, 0.0, 0.0)]
    fn test_greatest_common_divisor(#[case] a: f32, #[case] b: f32, #[case] expected: f32) {
        let result = greatest_common_divisor(a, b, None);
        assert_approx_eq!(expected, result);
    }

    #[rstest]
    #[case(1.0, 1.0, 1.0)]
    #[case(1.5, 1.0, 3.0)]
    #[case(1.4, 1.0, 7.0)]
    #[case(0.01, 3.8, 3.8)]
    #[case(0.0, 1.0, 0.0)]
    #[case(1.0, 0.0, 0.0)]
    #[case(0.0, 0.0, 0.0)]
    fn test_least_common_multiple(#[case] a: f32, #[case] b: f32, #[case] expected: f32) {
        let result = least_common_multiple(a, b);
        assert_approx_eq!(expected, result);
    }

    #[rstest]
    #[case(&[1.0, 2.0, 3.0], true)]
    #[case(&[2.0, 2.0, 2.0, 3.0], true)]
    #[case(&[1.0, 1.0, 1.0], false)]
    #[case(&[1.0, 2.0, 1.9], false)]
    #[case(&[3.0, 2.0, 1.0], false)]
    #[case(&[1.0], false)]
    #[case(&[], false)]
    fn test_is_increasing(#[case] arr: &[f32], #[case] expected: bool) {
        let result = is_increasing(arr);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(&[3.0, 2.0, 1.0], true)]
    #[case(&[2.0, 2.0, 2.0, 1.0], true)]
    #[case(&[1.0, 1.0, 1.0], false)]
    #[case(&[1.0, 0.0, 0.1], false)]
    #[case(&[1.0, 2.0, 3.0], false)]
    #[case(&[1.0], false)]
    #[case(&[], false)]
    fn test_is_decreasing(#[case] arr: &[f32], #[case] expected: bool) {
        let result = is_decreasing(arr);
        assert_eq!(expected, result);
    }
}
