pub const EPSILON: f64 = 1e-7;

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value > max {
        return max;
    }
    if value < min {
        return min;
    }
    value
}

pub fn wrap(value: f64, min: f64, max: f64) -> f64 {
    let range = max - min;
    if value < min {
        return max - (min - value) % range;
    }
    if value > max {
        return min + (value - min) % range;
    }
    value
}

pub fn lerp(percent: f64, start: f64, end: f64, should_clamp: bool) -> f64 {
    let value = start + (end - start) * percent;

    if should_clamp {
        clamp(value, start, end)
    } else {
        value
    }
}

pub fn norm(value: f64, minimum: f64, maximum: f64, should_clamp: bool) -> f64 {
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
    value: f64,
    original_min: f64,
    original_max: f64,
    new_min: f64,
    new_max: f64,
    should_clamp: bool,
) -> f64 {
    let normal = norm(value, original_min, original_max, should_clamp);
    lerp(normal, new_min, new_max, should_clamp)
}

pub fn is_approximately_equal(value1: f64, value2: f64, precision: Option<f64>) -> bool {
    let actual_precision = match precision {
        Some(precision) => precision,
        None => EPSILON,
    };
    (value1 - value2).abs() <= actual_precision
}

pub fn is_approximately_zero(value: f64) -> bool {
    value.abs() <= EPSILON
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
    fn test_clamp(#[case] value: f64, #[case] min: f64, #[case] max: f64, #[case] expected: f64) {
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
    fn test_wrap(#[case] value: f64, #[case] min: f64, #[case] max: f64, #[case] expected: f64) {
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
        #[case] value: f64,
        #[case] min: f64,
        #[case] max: f64,
        #[case] should_clamp: bool,
        #[case] expected: f64,
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
        #[case] percent: f64,
        #[case] start: f64,
        #[case] end: f64,
        #[case] should_clamp: bool,
        #[case] expected: f64,
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
        #[case] value: f64,
        #[case] original_min: f64,
        #[case] original_max: f64,
        #[case] new_min: f64,
        #[case] new_max: f64,
        #[case] should_clamp: bool,
        #[case] expected: f64,
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
}
