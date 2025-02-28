use super::quantity::{Quantity, Unit};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Distance {
    Meters,
    Millimeters,
    Centimeters,
    Kilometers,
    Inches,
    Feet,
    Yards,
    Miles,
    NauticalMiles,
}

impl Unit for Distance {
    fn multiplier_to_base(&self) -> f32 {
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

// Define some quantities for easier usage
pub const METER: Quantity<Distance> = Quantity {
    amount: 1.0,
    units: Distance::Meters,
};

pub const MILLIMETER: Quantity<Distance> = Quantity {
    amount: 1.0,
    units: Distance::Millimeters,
};

pub const CENTIMETER: Quantity<Distance> = Quantity {
    amount: 1.0,
    units: Distance::Centimeters,
};

pub const KILOMETER: Quantity<Distance> = Quantity {
    amount: 1.0,
    units: Distance::Kilometers,
};

pub const INCH: Quantity<Distance> = Quantity {
    amount: 1.0,
    units: Distance::Inches,
};

pub const FOOT: Quantity<Distance> = Quantity {
    amount: 1.0,
    units: Distance::Feet,
};

pub const YARD: Quantity<Distance> = Quantity {
    amount: 1.0,
    units: Distance::Yards,
};

pub const MILE: Quantity<Distance> = Quantity {
    amount: 1.0,
    units: Distance::Miles,
};

pub const NAUTICAL_MILE: Quantity<Distance> = Quantity {
    amount: 1.0,
    units: Distance::NauticalMiles,
};

#[cfg(test)]
mod tests {
    use crate::units::{
        distance::{
            CENTIMETER, Distance, FOOT, INCH, KILOMETER, METER, MILE, MILLIMETER, NAUTICAL_MILE,
            YARD,
        },
        quantity::{Convertable, Quantity},
    };

    #[test]
    fn can_convert_between_distance_units() {
        assert_quantities_eq(KILOMETER.convert(Distance::Meters), METER * 1000.0);
        assert_quantities_eq(METER.convert(Distance::Kilometers), KILOMETER * 0.001);
        assert_quantities_eq(CENTIMETER.convert(Distance::Meters), METER * 0.01);
        assert_quantities_eq(MILLIMETER.convert(Distance::Meters), METER * 0.001);
        assert_quantities_eq(MILLIMETER.convert(Distance::Centimeters), CENTIMETER * 0.1);
        assert_quantities_eq(MILLIMETER.convert(Distance::Millimeters), MILLIMETER * 1.0);
        assert_quantities_eq(
            (CENTIMETER * 5.0).convert(Distance::Kilometers),
            KILOMETER * 0.00005,
        );

        // Imperial unit conversions
        assert_quantities_eq(FOOT.convert(Distance::Inches), INCH * 12.0);
        assert_quantities_eq(YARD.convert(Distance::Feet), FOOT * 3.0);
        assert_quantities_eq(MILE.convert(Distance::Feet), FOOT * 5280.0);

        // Imperial to metric conversions
        assert_quantities_eq(FOOT.convert(Distance::Meters), METER * 0.3048);
        assert_quantities_eq(YARD.convert(Distance::Meters), METER * 0.9144);
        assert_quantities_eq(MILE.convert(Distance::Kilometers), KILOMETER * 1.609344);

        // Nautical mile conversions
        assert_quantities_eq(
            NAUTICAL_MILE.convert(Distance::Kilometers),
            KILOMETER * 1.852,
        );
        assert_quantities_eq(NAUTICAL_MILE.convert(Distance::Miles), MILE * 1.15078);

        // Zero conversions
        assert_quantities_eq((METER * 0.0).convert(Distance::Feet), FOOT * 0.0);
        assert_quantities_eq((FOOT * 0.0).convert(Distance::Meters), METER * 0.0);
    }

    fn assert_quantities_eq(q1: Quantity<Distance>, q2: Quantity<Distance>) {
        assert!(
            (q1.amount - q2.amount).abs() <= 0.0001,
            "quantities not equal - expected: {} {:?}, actual: {} {:?}",
            q1.amount,
            q1.units,
            q2.amount,
            q2.units
        );
        assert_eq!(q1.units, q2.units);
    }
}
