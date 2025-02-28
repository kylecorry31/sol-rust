use super::quantity::{Quantity, Unit};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pressure {
    Hectopascals,
    Pascals,
    Bars,
    Millibars,
    InchesHg,
    PoundsPerSquareInch,
    MillimetersHg,
    Atmospheres,
}

impl Unit for Pressure {
    fn multiplier_to_base(&self) -> f32 {
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

// Define some quantities for easier usage
pub const HECTOPASCAL: Quantity<Pressure> = Quantity {
    amount: 1.0,
    units: Pressure::Hectopascals,
};

pub const PASCAL: Quantity<Pressure> = Quantity {
    amount: 1.0,
    units: Pressure::Pascals,
};

pub const BAR: Quantity<Pressure> = Quantity {
    amount: 1.0,
    units: Pressure::Bars,
};

pub const MILLIBAR: Quantity<Pressure> = Quantity {
    amount: 1.0,
    units: Pressure::Millibars,
};

pub const INCH_HG: Quantity<Pressure> = Quantity {
    amount: 1.0,
    units: Pressure::InchesHg,
};

pub const PSI: Quantity<Pressure> = Quantity {
    amount: 1.0,
    units: Pressure::PoundsPerSquareInch,
};

pub const MM_HG: Quantity<Pressure> = Quantity {
    amount: 1.0,
    units: Pressure::MillimetersHg,
};

pub const ATM: Quantity<Pressure> = Quantity {
    amount: 1.0,
    units: Pressure::Atmospheres,
};

#[cfg(test)]
mod tests {
    use crate::units::{
        pressure::{ATM, BAR, HECTOPASCAL, INCH_HG, MILLIBAR, MM_HG, PASCAL, PSI, Pressure},
        quantity::{Convertable, Quantity},
    };

    #[test]
    fn can_convert_between_pressure_units() {
        assert_quantities_eq(HECTOPASCAL.convert(Pressure::Pascals), PASCAL * 100.0);
        assert_quantities_eq(PASCAL.convert(Pressure::Hectopascals), HECTOPASCAL * 0.01);
        assert_quantities_eq(PASCAL.convert(Pressure::Pascals), PASCAL);
        assert_quantities_eq(BAR.convert(Pressure::Pascals), PASCAL * 100000.0);
        assert_quantities_eq(MILLIBAR.convert(Pressure::Pascals), PASCAL * 100.0);
        assert_quantities_eq(INCH_HG.convert(Pressure::Pascals), PASCAL * 3386.389);
        assert_quantities_eq(PSI.convert(Pressure::Pascals), PASCAL * 6894.757);
        assert_quantities_eq(MM_HG.convert(Pressure::Pascals), PASCAL * 133.322);
        assert_quantities_eq(ATM.convert(Pressure::Pascals), PASCAL * 101325.0);

        // Test conversions between non-base units
        assert_quantities_eq(BAR.convert(Pressure::Hectopascals), HECTOPASCAL * 1000.0);
        assert_quantities_eq(ATM.convert(Pressure::Bars), BAR * 1.01325);
        assert_quantities_eq(PSI.convert(Pressure::InchesHg), INCH_HG * 2.036);
        assert_quantities_eq(MM_HG.convert(Pressure::Millibars), MILLIBAR * 1.33322);
    }

    fn assert_quantities_eq(q1: Quantity<Pressure>, q2: Quantity<Pressure>) {
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
