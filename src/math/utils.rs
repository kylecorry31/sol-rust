use super::numbers::Number;

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value > max {
        return max;
    }
    if value < min {
        return min;
    }
    value
}

pub fn wrap<T: Number>(value: T, min: T, max: T) -> T {
    let range = max - min;
    if value < min {
        return max - (min - value) % range;
    }
    if value > max {
        return min + (value - min) % range;
    }
    value
}

pub fn lerp<T: Number>(percent: T, start: T, end: T, should_clamp: bool) -> T {
    let value = start + (end - start) * percent;

    if should_clamp {
        clamp(value, start, end)
    } else {
        value
    }
}

pub fn norm<T: Number>(value: T, minimum: T, maximum: T, should_clamp: bool) -> T {
    let range = maximum - minimum;
    if range == T::from_i32(0) {
        return T::from_i32(0);
    }
    let normal = (value - minimum) / range;

    if should_clamp {
        clamp(normal, T::from_i32(0), T::from_i32(1))
    } else {
        normal
    }
}

pub fn map<T: Number>(
    value: T,
    original_min: T,
    original_max: T,
    new_min: T,
    new_max: T,
    should_clamp: bool,
) -> T {
    let normal = norm(value, original_min, original_max, should_clamp);
    lerp(normal, new_min, new_max, should_clamp)
}

#[cfg(test)]
mod tests {
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
        assert_eq!(clamp(value, min, max), expected);
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
        assert_eq!(wrap(value, min, max), expected);
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
        assert_eq!(norm(value, min, max, should_clamp), expected);
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
        assert_eq!(lerp(percent, start, end, should_clamp), expected);
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
        assert_eq!(
            map(
                value,
                original_min,
                original_max,
                new_min,
                new_max,
                should_clamp
            ),
            expected
        );
    }
}
