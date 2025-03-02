use crate::units::{distance::Distance, pressure::Pressure, quantity::Quantity};

// TODO: Factor in temperature
pub fn get_sea_level_pressure(
    pressure: &Quantity<Pressure>,
    altitude: &Quantity<Distance>,
) -> Quantity<Pressure> {
    let hpa = pressure.amount;
    let meters = altitude.amount;
    let sea_level_hpa = hpa * (1.0 - meters / 44330.0).powf(-5.255);
    Quantity {
        amount: sea_level_hpa,
        units: Pressure::Hectopascals,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_approx_eq, units::distance::Distance};

    use rstest::rstest;

    #[rstest]
    #[case(
        Quantity { amount: 0.0, units: Pressure::Hectopascals },
        Quantity { amount: 0.0, units: Distance::Meters },
        Quantity { amount: 0.0, units: Pressure::Hectopascals }
    )]
    #[case(
        Quantity { amount: 1000.0, units: Pressure::Hectopascals },
        Quantity { amount: -100.0, units: Distance::Meters },
        Quantity { amount: 988.229, units: Pressure::Hectopascals }
    )]
    #[case(
        Quantity { amount: 980.0, units: Pressure::Hectopascals },
        Quantity { amount: 200.0, units: Distance::Meters },
        Quantity { amount: 1003.56573, units: Pressure::Hectopascals }
    )]
    fn can_get_sea_level_pressure(
        #[case] pressure: Quantity<Pressure>,
        #[case] altitude: Quantity<Distance>,
        #[case] expected: Quantity<Pressure>,
    ) {
        let actual = get_sea_level_pressure(&pressure, &altitude);
        assert_approx_eq!(expected.amount, actual.amount, 0.01);
        assert_eq!(expected.units, actual.units);
    }
}
