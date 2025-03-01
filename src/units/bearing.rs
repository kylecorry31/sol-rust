use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumIter)]
pub enum CompassDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl CompassDirection {
    fn azimuth(&self) -> f32 {
        match self {
            CompassDirection::North => 0.0,
            CompassDirection::NorthEast => 45.0,
            CompassDirection::East => 90.0,
            CompassDirection::SouthEast => 135.0,
            CompassDirection::South => 180.0,
            CompassDirection::SouthWest => 225.0,
            CompassDirection::West => 270.0,
            CompassDirection::NorthWest => 315.0,
        }
    }
}

pub struct Bearing {
    value: f32,
}

impl Bearing {
    pub fn new(value: f32) -> Self {
        let normalized = if value.is_nan() || !value.is_finite() {
            0.0
        } else {
            normalize_angle(value)
        };

        Bearing { value: normalized }
    }

    pub fn direction(&self) -> CompassDirection {
        let a = ((self.value / 45.0).round() * 45.0) % 360.0;
        CompassDirection::iter()
            .find(|d| a == d.azimuth())
            .unwrap_or(CompassDirection::North)
    }

    pub fn with_declination(&self, declination: f32) -> Bearing {
        Bearing::new(self.value + declination)
    }

    pub fn inverse(&self) -> Bearing {
        Bearing::new(self.value + 180.0)
    }

    pub fn from(direction: CompassDirection) -> Bearing {
        Bearing::new(direction.azimuth())
    }

    pub fn get_bearing(degrees: f32) -> f32 {
        if degrees.is_nan() || !degrees.is_finite() {
            0.0
        } else {
            normalize_angle(degrees)
        }
    }
}

// TODO: Extract to math
fn normalize_angle(angle: f32) -> f32 {
    ((angle % 360.0) + 360.0) % 360.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, CompassDirection::North)]
    #[case(90.0, CompassDirection::East)]
    #[case(180.0, CompassDirection::South)]
    #[case(270.0, CompassDirection::West)]
    #[case(45.0, CompassDirection::NorthEast)]
    #[case(135.0, CompassDirection::SouthEast)]
    #[case(225.0, CompassDirection::SouthWest)]
    #[case(315.0, CompassDirection::NorthWest)]
    #[case(10.0, CompassDirection::North)]
    #[case(350.0, CompassDirection::North)]
    #[case(f32::NAN, CompassDirection::North)]
    #[case(f32::NEG_INFINITY, CompassDirection::North)]
    #[case(f32::INFINITY, CompassDirection::North)]
    fn test_direction(#[case] azimuth: f32, #[case] expected: CompassDirection) {
        let bearing = Bearing::new(azimuth);
        assert_eq!(expected, bearing.direction());
    }

    #[test]
    fn test_from() {
        for direction in CompassDirection::iter() {
            let bearing = Bearing::from(direction);
            assert!(
                (direction.azimuth() - bearing.value).abs() < 0.01,
                "values not equal - expected: {}, actual: {}",
                direction.azimuth(),
                bearing.value
            );
        }
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(180.0, 180.0)]
    #[case(-10.0, 350.0)]
    #[case(-180.0, 180.0)]
    #[case(710.0, 350.0)]
    #[case(360.0, 0.0)]
    #[case(-710.0, 10.0)]
    #[case(f32::NAN, 0.0)]
    #[case(f32::NEG_INFINITY, 0.0)]
    #[case(f32::INFINITY, 0.0)]
    fn test_value(#[case] azimuth: f32, #[case] expected: f32) {
        let bearing = Bearing::new(azimuth);
        assert!(
            (expected - bearing.value).abs() < 0.01,
            "values not equal - expected: {}, actual: {}",
            expected,
            bearing.value
        );
    }

    #[rstest]
    #[case(0.0, 180.0)]
    #[case(180.0, 0.0)]
    #[case(90.0, 270.0)]
    #[case(270.0, 90.0)]
    fn test_inverse(#[case] azimuth: f32, #[case] expected: f32) {
        let bearing = Bearing::new(azimuth);
        assert!(
            (expected - bearing.inverse().value).abs() < 0.01,
            "values not equal - expected: {}, actual: {}",
            expected,
            bearing.inverse().value
        );
    }

    #[test]
    fn test_with_declination() {
        let bearing = Bearing::new(45.0);
        let dec = bearing.with_declination(-10.0);
        assert!(
            (35.0 - dec.value).abs() < 0.01,
            "values not equal - expected: {}, actual: {}",
            35.0,
            dec.value
        );
    }
}
