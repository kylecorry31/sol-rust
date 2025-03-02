use super::quantity::Unit;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Volume {
    Liters = 1,
    Milliliter = 2,
    USCups = 3,
    USPints = 4,
    USQuarts = 5,
    USOunces = 6,
    USGallons = 7,
    ImperialCups = 8,
    ImperialPints = 9,
    ImperialQuarts = 10,
    ImperialOunces = 11,
    ImperialGallons = 12,
    USTeaspoons = 13,
    USTablespoons = 14,
    ImperialTeaspoons = 15,
    ImperialTablespoons = 16,
}

impl Unit for Volume {
    fn multiplier_to_base(&self) -> f64 {
        match self {
            Volume::Liters => 1.0,
            Volume::Milliliter => 0.001,
            Volume::USCups => 0.236588,
            Volume::USPints => 0.473176,
            Volume::USQuarts => 0.946353,
            Volume::USOunces => 0.0295735,
            Volume::USGallons => 3.78541,
            Volume::ImperialCups => 0.284131,
            Volume::ImperialPints => 0.568261,
            Volume::ImperialQuarts => 1.13652,
            Volume::ImperialOunces => 0.0284131,
            Volume::ImperialGallons => 4.54609,
            Volume::USTeaspoons => 0.004928922,
            Volume::USTablespoons => 0.014786765,
            Volume::ImperialTeaspoons => 0.00591939,
            Volume::ImperialTablespoons => 0.0177582,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::units::{
        quantity::{Convertable, Quantity},
        volume::Volume,
    };

    #[rstest]
    #[case(2.0, Volume::Liters, Volume::USGallons, 0.528344)]
    #[case(3.0, Volume::USGallons, Volume::Liters, 11.3562)]
    #[case(3.0, Volume::ImperialGallons, Volume::USOunces, 461.16525)]
    #[case(4.0, Volume::Milliliter, Volume::ImperialCups, 0.014078)]
    #[case(4.0, Volume::USPints, Volume::USQuarts, 2.0)]
    #[case(4.0, Volume::USCups, Volume::ImperialCups, 3.3307)]
    #[case(4.0, Volume::ImperialOunces, Volume::ImperialPints, 0.2)]
    #[case(1.0, Volume::ImperialQuarts, Volume::Milliliter, 1136.52)]
    #[case(1.0, Volume::USTablespoons, Volume::USTeaspoons, 3.0)]
    #[case(1.0, Volume::ImperialTablespoons, Volume::ImperialTeaspoons, 3.0)]
    fn can_convert_between_volume_units(
        #[case] amount: f64,
        #[case] units: Volume,
        #[case] to_units: Volume,
        #[case] expected_amount: f64,
    ) {
        let actual = Quantity { amount, units }.convert(to_units);
        assert!(
            (actual.amount - expected_amount).abs() <= 0.01,
            "quantities not equal - expected: {} {:?}, actual: {} {:?}",
            actual.amount,
            actual.units,
            expected_amount,
            to_units
        );

        assert_eq!(actual.units, to_units);
    }
}
