use super::arithmetic::round_places;

pub const EPSILON: f32 = 1e-5;

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value > max {
        return max;
    }
    if value < min {
        return min;
    }
    value
}

pub fn wrap(value: f32, min: f32, max: f32) -> f32 {
    let range = max - min;
    if value < min {
        return max - (min - value) % range;
    }
    if value > max {
        return min + (value - min) % range;
    }
    value
}

pub fn lerp(percent: f32, start: f32, end: f32, should_clamp: bool) -> f32 {
    let value = start + (end - start) * percent;

    if should_clamp {
        clamp(value, start, end)
    } else {
        value
    }
}

pub fn norm(value: f32, minimum: f32, maximum: f32, should_clamp: bool) -> f32 {
    let range = maximum - minimum;
    if range == 0.0 {
        return 0.0;
    }
    let normal = (value - minimum) / range;

    if should_clamp {
        clamp(normal, 0.0, 1.0)
    } else {
        normal
    }
}

pub fn map(
    value: f32,
    original_min: f32,
    original_max: f32,
    new_min: f32,
    new_max: f32,
    should_clamp: bool,
) -> f32 {
    let normal = norm(value, original_min, original_max, should_clamp);
    lerp(normal, new_min, new_max, should_clamp)
}

pub fn is_approximately_equal(value1: f32, value2: f32, precision: Option<f32>) -> bool {
    let actual_precision = match precision {
        Some(precision) => precision,
        None => EPSILON,
    };
    (value1 - value2).abs() <= actual_precision
}

pub fn is_approximately_zero(value: f32) -> bool {
    value.abs() <= EPSILON
}

/// This will round to the desired number of places if the value is approximately equal at 2 places below the desired. This can be used to correct floating point errors.
pub fn approximate_round(value: f32, desired_places: i32) -> f32 {
    let rounded = round_places(value, desired_places);
    let extra_rounded = round_places(value, desired_places + 2);
    if is_approximately_equal(extra_rounded, rounded, None) {
        rounded
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_eq;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0.1, 0.0, 1.0, 0.1)]
    #[case(0.0, 0.0, 1.0, 0.0)]
    #[case(1.0, 0.0, 1.0, 1.0)]
    #[case(1.2, 0.0, 1.0, 1.0)]
    #[case(-0.1, 0.0, 1.0, 0.0)]
    #[case(4.0, 2.0, 5.0, 4.0)]
    #[case(1.0, 2.0, 5.0, 2.0)]
    #[case(6.0, 2.0, 5.0, 5.0)]
    fn test_clamp(#[case] value: f32, #[case] min: f32, #[case] max: f32, #[case] expected: f32) {
        assert_approx_eq!(expected, clamp(value, min, max));
    }

    #[rstest]
    #[case(0.2, 0.0, 1.0, 0.2)]
    #[case(1.0, 0.0, 1.0, 1.0)]
    #[case(0.0, 0.0, 1.0, 0.0)]
    #[case(1.5, 0.0, 1.0, 0.5)]
    #[case(-0.75, 0.0, 1.0, 0.25)]
    #[case(0.0, 1.0, 4.0, 3.0)]
    #[case(5.0, 1.0, 4.0, 2.0)]
    #[case(6.0, 5.0, 4.0, 5.0)]
    #[case(0.0, 0.0, 0.0, 0.0)]
    #[case(1800.0, 0.0, 360.0, 0.0)]
    #[case(-1800.0, 0.0, 360.0, 360.0)]
    #[case(1799.0, 0.0, 360.0, 359.0)]
    #[case(-1799.0, 0.0, 360.0, 1.0)]
    fn test_wrap(#[case] value: f32, #[case] min: f32, #[case] max: f32, #[case] expected: f32) {
        assert_approx_eq!(expected, wrap(value, min, max));
    }

    #[rstest]
    #[case(0.1, 0.0, 1.0, false, 0.1)]
    #[case(0.0, 0.0, 1.0, false, 0.0)]
    #[case(1.0, 0.0, 1.0, false, 1.0)]
    #[case(1.2, 0.0, 1.0, false, 1.2)]
    #[case(-0.1, 0.0, 1.0, false, -0.1)]
    #[case(4.0, 2.0, 6.0, false, 0.5)]
    #[case(1.0, 2.0, 6.0, false, -0.25)]
    #[case(6.0, 2.0, 6.0, false, 1.0)]
    #[case(2.0, 2.0, 6.0, false, 0.0)]
    #[case(-1.0, 0.0, 1.0, true, 0.0)]
    #[case(0.5, 0.0, 1.0, true, 0.5)]
    #[case(2.0, 0.0, 1.0, true, 1.0)]
    fn test_norm(
        #[case] value: f32,
        #[case] min: f32,
        #[case] max: f32,
        #[case] should_clamp: bool,
        #[case] expected: f32,
    ) {
        assert_approx_eq!(expected, norm(value, min, max, should_clamp));
    }

    #[rstest]
    #[case(0.1, 0.0, 1.0, false, 0.1)]
    #[case(0.0, 0.0, 1.0, false, 0.0)]
    #[case(1.0, 0.0, 1.0, false, 1.0)]
    #[case(1.2, 0.0, 1.0, false, 1.2)]
    #[case(-0.1, 0.0, 1.0, false, -0.1)]
    #[case(0.5, 2.0, 6.0, false, 4.0)]
    #[case(-0.25, 2.0, 6.0, false, 1.0)]
    #[case(1.0, 2.0, 6.0, false, 6.0)]
    #[case(0.0, 2.0, 6.0, false, 2.0)]
    #[case(-1.0, 0.0, 1.0, true, 0.0)]
    #[case(0.5, 0.0, 1.0, true, 0.5)]
    #[case(2.0, 0.0, 1.0, true, 1.0)]
    fn test_lerp(
        #[case] percent: f32,
        #[case] start: f32,
        #[case] end: f32,
        #[case] should_clamp: bool,
        #[case] expected: f32,
    ) {
        assert_approx_eq!(expected, lerp(percent, start, end, should_clamp));
    }

    #[rstest]
    #[case(0.1, 0.0, 1.0, 2.0, 4.0, false, 2.2)]
    #[case(0.0, 0.0, 1.0, 2.0, 4.0, false, 2.0)]
    #[case(1.0, 0.0, 1.0, 2.0, 4.0, false, 4.0)]
    #[case(1.2, 0.0, 1.0, 2.0, 4.0, false, 4.4)]
    #[case(-0.1, 0.0, 1.0, 2.0, 4.0, false, 1.8)]
    #[case(4.0, 2.0, 6.0, 0.0, 4.0, false, 2.0)]
    #[case(1.0, 2.0, 6.0, 0.0, 4.0, false, -1.0)]
    #[case(6.0, 2.0, 6.0, 0.0, 4.0, false, 4.0)]
    #[case(2.0, 2.0, 6.0, 0.0, 4.0, false, 0.0)]
    #[case(-1.0, 0.0, 1.0, 2.0, 4.0, true, 2.0)]
    #[case(0.5, 0.0, 1.0, 2.0, 4.0, true, 3.0)]
    #[case(2.0, 0.0, 1.0, 2.0, 4.0, true, 4.0)]
    fn test_map(
        #[case] value: f32,
        #[case] original_min: f32,
        #[case] original_max: f32,
        #[case] new_min: f32,
        #[case] new_max: f32,
        #[case] should_clamp: bool,
        #[case] expected: f32,
    ) {
        assert_approx_eq!(
            expected,
            map(
                value,
                original_min,
                original_max,
                new_min,
                new_max,
                should_clamp
            )
        );
    }

    #[rstest]
    #[case(0.1, 0.11, Some(0.1), true)]
    #[case(0.1, 0.11, Some(0.01), true)]
    #[case(0.1, 0.11, Some(0.001), false)]
    #[case(0.1, 0.1 + EPSILON / 2.0, None, true)]
    #[case(0.1, 0.1 + EPSILON * 2.0, None, false)]
    #[case(10000.0, 10000.0 + EPSILON / 2.0, None, true)]
    #[case(-0.1, -0.11, Some(0.1), true)]
    fn test_is_approximately_equal(
        #[case] value1: f32,
        #[case] value2: f32,
        #[case] precision: Option<f32>,
        #[case] expected: bool,
    ) {
        assert_eq!(expected, is_approximately_equal(value1, value2, precision));
    }

    #[rstest]
    #[case(0.0, true)]
    #[case(EPSILON / 2.0, true)]
    #[case(EPSILON * 2.0, false)]
    #[case(-EPSILON / 2.0, true)]
    #[case(-EPSILON * 2.0, false)]
    fn test_is_approximately_zero(#[case] value: f32, #[case] expected: bool) {
        assert_eq!(expected, is_approximately_zero(value));
    }

    #[rstest]
    #[case(0.700000000000000001, 2, 0.7)]
    #[case(0.700000000000000001, 1, 0.7)]
    #[case(0.799999999999999999, 1, 0.8)]
    #[case(0.799999999999999999, 2, 0.8)]
    #[case(0.799992331, 2, 0.8)]
    #[case(1.234567, 2, 1.234567)]
    #[case(1.234567, 3, 1.234567)]
    fn test_approximate_round(#[case] value: f32, #[case] places: i32, #[case] expected: f32) {
        assert_approx_eq!(expected, approximate_round(value, places));
    }
}
