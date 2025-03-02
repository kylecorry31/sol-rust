use super::quantity::{Quantity, Unit};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Angle {
    Radians = 1,
    Degrees = 2,
}

impl Angle {
    pub fn degrees(value: f32) -> Quantity<Angle> {
        Quantity {
            amount: value,
            units: Angle::Degrees,
        }
    }
}

impl Unit for Angle {
    fn multiplier_to_base(&self) -> f32 {
        match self {
            Angle::Radians => 1.0,
            Angle::Degrees => 0.017453292,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx_eq,
        units::{
            angle::Angle,
            quantity::{Convertable, Quantity},
        },
    };

    use rstest::rstest;

    #[rstest]
    #[case(1.0, Angle::Radians, Angle::Radians, 1.0)]
    #[case(1.0, Angle::Radians, Angle::Degrees, 57.29578)]
    #[case(1.0, Angle::Degrees, Angle::Radians, 0.017453292)]
    #[case(180.0, Angle::Degrees, Angle::Radians, 3.14159)]
    #[case(360.0, Angle::Degrees, Angle::Radians, 6.28319)]
    #[case(90.0, Angle::Degrees, Angle::Radians, 1.5708)]
    fn can_convert_between_angle_units(
        #[case] amount: f32,
        #[case] units: Angle,
        #[case] to_units: Angle,
        #[case] expected_amount: f32,
    ) {
        let actual = Quantity { amount, units }.convert(to_units);
        assert_approx_eq!(actual.amount, expected_amount);
        assert_eq!(actual.units, to_units);
    }
}
