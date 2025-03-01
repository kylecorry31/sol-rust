use super::numbers::Number;

pub fn constrain<T: PartialOrd>(value: T, min: T, max: T) -> T {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(5.0, 0.0, 10.0, 5.0)]
    #[case(15.0, 0.0, 10.0, 10.0)]
    #[case(-5.0, 0.0, 10.0, 0.0)]
    #[case(10.0, 0.0, 10.0, 10.0)]
    #[case(0.0, 0.0, 10.0, 0.0)]
    #[case(5.0, 5.0, 5.0, 5.0)]
    fn test_constrain(
        #[case] value: f32,
        #[case] min: f32,
        #[case] max: f32,
        #[case] expected: f32,
    ) {
        assert_eq!(constrain(value, min, max), expected);
    }

    #[rstest]
    #[case(5.0, 0.0, 10.0, 5.0)]
    #[case(15.0, 0.0, 10.0, 5.0)]
    #[case(-5.0, 0.0, 10.0, 5.0)]
    #[case(10.0, 0.0, 10.0, 10.0)]
    #[case(0.0, 0.0, 10.0, 0.0)]
    #[case(5.0, 5.0, 5.0, 5.0)]
    fn test_wrap(#[case] value: f32, #[case] min: f32, #[case] max: f32, #[case] expected: f32) {
        assert_eq!(wrap(value, min, max), expected);
    }
}
