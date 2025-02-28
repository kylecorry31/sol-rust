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
    use crate::units::distance::Distance;

    use super::*;

    #[test]
    fn can_get_sea_level_pressure() {
        assert_eq!(
            Quantity {
                amount: 0.0,
                units: Pressure::Hectopascals
            },
            get_sea_level_pressure(
                &Quantity {
                    amount: 0.0,
                    units: Pressure::Hectopascals
                },
                &Quantity {
                    amount: 0.0,
                    units: Distance::Meters
                }
            )
        );

        assert_eq!(
            Quantity {
                amount: 988.229,
                units: Pressure::Hectopascals
            },
            get_sea_level_pressure(
                &Quantity {
                    amount: 1000.0,
                    units: Pressure::Hectopascals
                },
                &Quantity {
                    amount: -100.0,
                    units: Distance::Meters
                }
            )
        );

        assert_eq!(
            Quantity {
                amount: 1003.56573,
                units: Pressure::Hectopascals
            },
            get_sea_level_pressure(
                &Quantity {
                    amount: 980.0,
                    units: Pressure::Hectopascals
                },
                &Quantity {
                    amount: 200.0,
                    units: Distance::Meters
                }
            )
        );
    }
}
