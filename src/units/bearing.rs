#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

const ALL_COMPASS_DIRECTIONS: [CompassDirection; 8] = [
    CompassDirection::North,
    CompassDirection::NorthEast,
    CompassDirection::East,
    CompassDirection::SouthEast,
    CompassDirection::South,
    CompassDirection::SouthWest,
    CompassDirection::West,
    CompassDirection::NorthWest,
];

impl CompassDirection {
    fn azimuth(&self) -> f64 {
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
    pub degrees: f64,
}

impl Bearing {
    pub fn new(value: f64) -> Self {
        let normalized = if value.is_nan() || !value.is_finite() {
            0.0
        } else {
            normalize_angle(value)
        };

        Bearing {
            degrees: normalized,
        }
    }

    pub fn direction(&self) -> CompassDirection {
        let a = ((self.degrees / 45.0).round() * 45.0) % 360.0;
        *ALL_COMPASS_DIRECTIONS
            .iter()
            .find(|d| a == d.azimuth())
            .unwrap_or(&CompassDirection::North)
    }

    pub fn with_declination(&self, declination: f64) -> Bearing {
        Bearing::new(self.degrees + declination)
    }

    pub fn inverse(&self) -> Bearing {
        Bearing::new(self.degrees + 180.0)
    }

    pub fn from(direction: CompassDirection) -> Bearing {
        Bearing::new(direction.azimuth())
    }

    pub fn get_bearing(degrees: f64) -> f64 {
        if degrees.is_nan() || !degrees.is_finite() {
            0.0
        } else {
            normalize_angle(degrees)
        }
    }
}

// TODO: Extract to math
fn normalize_angle(angle: f64) -> f64 {
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
    #[case(f64::NAN, CompassDirection::North)]
    #[case(f64::NEG_INFINITY, CompassDirection::North)]
    #[case(f64::INFINITY, CompassDirection::North)]
    fn test_direction(#[case] azimuth: f64, #[case] expected: CompassDirection) {
        let bearing = Bearing::new(azimuth);
        assert_eq!(expected, bearing.direction());
    }

    #[test]
    fn test_from() {
        for direction in ALL_COMPASS_DIRECTIONS {
            let bearing = Bearing::from(direction);
            assert!(
                (direction.azimuth() - bearing.degrees).abs() < 0.01,
                "values not equal - expected: {}, actual: {}",
                direction.azimuth(),
                bearing.degrees
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
    #[case(f64::NAN, 0.0)]
    #[case(f64::NEG_INFINITY, 0.0)]
    #[case(f64::INFINITY, 0.0)]
    fn test_value(#[case] azimuth: f64, #[case] expected: f64) {
        let bearing = Bearing::new(azimuth);
        assert!(
            (expected - bearing.degrees).abs() < 0.01,
            "values not equal - expected: {}, actual: {}",
            expected,
            bearing.degrees
        );
    }

    #[rstest]
    #[case(0.0, 180.0)]
    #[case(180.0, 0.0)]
    #[case(90.0, 270.0)]
    #[case(270.0, 90.0)]
    fn test_inverse(#[case] azimuth: f64, #[case] expected: f64) {
        let bearing = Bearing::new(azimuth);
        assert!(
            (expected - bearing.inverse().degrees).abs() < 0.01,
            "values not equal - expected: {}, actual: {}",
            expected,
            bearing.inverse().degrees
        );
    }

    #[test]
    fn test_with_declination() {
        let bearing = Bearing::new(45.0);
        let dec = bearing.with_declination(-10.0);
        assert!(
            (35.0 - dec.degrees).abs() < 0.01,
            "values not equal - expected: {}, actual: {}",
            35.0,
            dec.degrees
        );
    }
}
