use super::quantity::Unit;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Time {
    Milliseconds = 1,
    Seconds = 2,
    Minutes = 3,
    Hours = 4,
    Days = 5,
}

impl Unit for Time {
    fn multiplier_to_base(&self) -> f32 {
        match self {
            Time::Milliseconds => 0.001,
            Time::Seconds => 1.0,
            Time::Minutes => 60.0,
            Time::Hours => 3600.0,
            Time::Days => 86400.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::units::{
        quantity::{Convertable, Quantity},
        time::Time,
    };

    #[rstest]
    #[case(1.0, Time::Seconds, Time::Milliseconds, 1000.0)]
    #[case(1.0, Time::Seconds, Time::Seconds, 1.0)]
    #[case(1.0, Time::Minutes, Time::Seconds, 60.0)]
    #[case(1.0, Time::Hours, Time::Seconds, 3600.0)]
    #[case(1.0, Time::Days, Time::Seconds, 86400.0)]
    #[case(60.0, Time::Seconds, Time::Minutes, 1.0)]
    #[case(3600.0, Time::Seconds, Time::Hours, 1.0)]
    #[case(86400.0, Time::Seconds, Time::Days, 1.0)]
    #[case(1.0, Time::Hours, Time::Minutes, 60.0)]
    #[case(24.0, Time::Hours, Time::Days, 1.0)]
    #[case(1.0, Time::Days, Time::Hours, 24.0)]
    #[case(1.0, Time::Minutes, Time::Milliseconds, 60000.0)]
    fn can_convert_between_time_units(
        #[case] amount: f32,
        #[case] units: Time,
        #[case] to_units: Time,
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
