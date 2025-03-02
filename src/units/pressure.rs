use super::quantity::Unit;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pressure {
    Hectopascals = 1,
    Millibars = 2,
    InchesHg = 3,
    PoundsPerSquareInch = 4,
    MillimetersHg = 5,
    Pascals = 6,
    Bars = 7,
    Atmospheres = 8,
}

impl Unit for Pressure {
    fn multiplier_to_base(&self) -> f64 {
        match self {
            Pressure::Pascals => 1.0,
            Pressure::Hectopascals => 100.0,
            Pressure::Bars => 100000.0,
            Pressure::Millibars => 100.0,
            Pressure::InchesHg => 3386.389,
            Pressure::PoundsPerSquareInch => 6894.757,
            Pressure::MillimetersHg => 133.322,
            Pressure::Atmospheres => 101325.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::units::{
        pressure::Pressure,
        quantity::{Convertable, Quantity},
    };

    #[rstest]
    #[case(1.0, Pressure::Pascals, Pressure::Hectopascals, 0.01)]
    #[case(1.0, Pressure::Pascals, Pressure::Bars, 0.00001)]
    #[case(1.0, Pressure::Pascals, Pressure::Millibars, 0.01)]
    #[case(1.0, Pressure::Pascals, Pressure::InchesHg, 0.0002953)]
    #[case(1.0, Pressure::Pascals, Pressure::PoundsPerSquareInch, 0.000145038)]
    #[case(1.0, Pressure::Pascals, Pressure::MillimetersHg, 0.00750062)]
    #[case(1.0, Pressure::Pascals, Pressure::Atmospheres, 0.0000101325)]
    #[case(1.0, Pressure::Hectopascals, Pressure::Pascals, 100.0)]
    #[case(1.0, Pressure::Bars, Pressure::Pascals, 100000.0)]
    #[case(1.0, Pressure::Millibars, Pressure::Pascals, 100.0)]
    #[case(1.0, Pressure::InchesHg, Pressure::Pascals, 3386.389)]
    #[case(1.0, Pressure::PoundsPerSquareInch, Pressure::Pascals, 6894.757)]
    #[case(1.0, Pressure::MillimetersHg, Pressure::Pascals, 133.322)]
    #[case(1.0, Pressure::Atmospheres, Pressure::Pascals, 101325.0)]
    #[case(1.0, Pressure::Bars, Pressure::Hectopascals, 1000.0)]
    #[case(1.0, Pressure::Millibars, Pressure::Hectopascals, 1.0)]
    #[case(1.0, Pressure::InchesHg, Pressure::Bars, 0.033864)]
    #[case(1.0, Pressure::PoundsPerSquareInch, Pressure::Millibars, 68.9476)]
    #[case(1.0, Pressure::MillimetersHg, Pressure::InchesHg, 0.0393701)]
    #[case(1.0, Pressure::Atmospheres, Pressure::Bars, 1.01325)]
    fn can_convert_between_pressure_units(
        #[case] amount: f64,
        #[case] units: Pressure,
        #[case] to_units: Pressure,
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
