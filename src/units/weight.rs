use super::quantity::Unit;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Weight {
    Pounds = 1,
    Ounces = 2,
    Kilograms = 3,
    Grams = 4,
}

impl Unit for Weight {
    fn multiplier_to_base(&self) -> f32 {
        match self {
            Weight::Pounds => 453.592,
            Weight::Ounces => 28.3495,
            Weight::Kilograms => 1000.0,
            Weight::Grams => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::units::quantity::{Convertable, Quantity};

    use super::Weight;

    #[rstest]
    #[case(2.0, Weight::Pounds, Weight::Kilograms, 0.907185)]
    #[case(3.0, Weight::Kilograms, Weight::Grams, 3000.0)]
    #[case(4.0, Weight::Ounces, Weight::Pounds, 0.25)]
    #[case(4.0, Weight::Grams, Weight::Ounces, 0.141096)]
    fn can_convert_between_weight_units(
        #[case] amount: f32,
        #[case] units: Weight,
        #[case] to_units: Weight,
        #[case] expected_amount: f32,
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
