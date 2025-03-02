use super::quantity::Unit;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Distance {
    Centimeters = 1,
    Inches = 2,
    Miles = 3,
    Yards = 4,
    Feet = 5,
    Kilometers = 6,
    Meters = 7,
    NauticalMiles = 8,
    Millimeters = 9,
}

impl Unit for Distance {
    fn multiplier_to_base(&self) -> f64 {
        match self {
            Distance::Meters => 1.0,
            Distance::Kilometers => 1000.0,
            Distance::Centimeters => 0.01,
            Distance::Millimeters => 0.001,
            Distance::Inches => 0.0254,
            Distance::Feet => 0.3048,
            Distance::Yards => 0.9144,
            Distance::Miles => 1609.344,
            Distance::NauticalMiles => 1852.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::units::{
        distance::Distance,
        quantity::{Convertable, Quantity},
    };

    use rstest::rstest;

    #[rstest]
    #[case(1.0, Distance::Meters, Distance::Meters, 1.0)]
    #[case(1.0, Distance::Meters, Distance::Kilometers, 0.001)]
    #[case(1.0, Distance::Meters, Distance::Miles, 0.000621371)]
    #[case(1.0, Distance::Meters, Distance::NauticalMiles, 0.000539957)]
    #[case(1.0, Distance::Meters, Distance::Feet, 3.28084)]
    #[case(1.0, Distance::Meters, Distance::Inches, 39.3701)]
    #[case(1.0, Distance::Meters, Distance::Yards, 1.09361)]
    #[case(1.0, Distance::Meters, Distance::Centimeters, 100.0)]
    #[case(1.0, Distance::Meters, Distance::Millimeters, 1000.0)]
    #[case(1.0, Distance::Kilometers, Distance::Meters, 1000.0)]
    #[case(1.0, Distance::Miles, Distance::Meters, 1609.344)]
    #[case(1.0, Distance::NauticalMiles, Distance::Meters, 1852.0)]
    #[case(1.0, Distance::Feet, Distance::Meters, 0.3048)]
    #[case(1.0, Distance::Inches, Distance::Meters, 0.0254)]
    #[case(1.0, Distance::Yards, Distance::Meters, 0.9144)]
    #[case(1.0, Distance::Centimeters, Distance::Meters, 0.01)]
    #[case(1.0, Distance::Millimeters, Distance::Meters, 0.001)]
    #[case(1.0, Distance::Kilometers, Distance::Miles, 0.621371)]
    #[case(1.0, Distance::Miles, Distance::Kilometers, 1.60934)]
    #[case(1.0, Distance::Feet, Distance::Inches, 12.0)]
    #[case(1.0, Distance::Yards, Distance::Feet, 3.0)]
    #[case(1.0, Distance::Miles, Distance::Feet, 5280.0)]
    fn can_convert_between_distance_units(
        #[case] amount: f64,
        #[case] units: Distance,
        #[case] to_units: Distance,
        #[case] expected_amount: f64,
    ) {
        let actual = Quantity { amount, units }.convert(to_units);
        assert!(
            (actual.amount - expected_amount).abs() <= 0.0001,
            "quantities not equal - expected: {} {:?}, actual: {} {:?}",
            actual.amount,
            actual.units,
            expected_amount,
            to_units
        );

        assert_eq!(actual.units, to_units);
    }
}
