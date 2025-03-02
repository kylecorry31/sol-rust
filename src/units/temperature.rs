use super::quantity::Unit;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Temperature {
    Fahrenheit = 1,
    Celsius = 2,
}

impl Unit for Temperature {
    fn multiplier_to_base(&self) -> f64 {
        match self {
            Temperature::Fahrenheit => 5.0 / 9.0,
            Temperature::Celsius => 1.0,
        }
    }

    fn offset_from_base(&self) -> f64 {
        match self {
            Temperature::Fahrenheit => -32.0,
            Temperature::Celsius => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use crate::units::{
        quantity::{Convertable, Quantity},
        temperature::Temperature,
    };

    #[rstest]
    #[case(1.0, Temperature::Celsius, Temperature::Celsius, 1.0)]
    #[case(1.0, Temperature::Celsius, Temperature::Fahrenheit, 33.8)]
    #[case(1.0, Temperature::Fahrenheit, Temperature::Celsius, -17.2222)]
    #[case(1.0, Temperature::Fahrenheit, Temperature::Fahrenheit, 1.0)]
    #[case(-40.0, Temperature::Fahrenheit, Temperature::Celsius, -40.0)]
    #[case(-40.0, Temperature::Celsius, Temperature::Fahrenheit, -40.0)]
    fn can_convert_between_temperature_units(
        #[case] amount: f64,
        #[case] units: Temperature,
        #[case] to_units: Temperature,
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
