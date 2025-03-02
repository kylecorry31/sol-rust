pub trait Unit: Copy + Clone + PartialEq + Eq {
    /// Returns the multiplier to convert the unit to the base unit.
    fn multiplier_to_base(&self) -> f64;
    /// Returns the offset to convert the unit to the base unit (done before multiplication).
    fn offset_from_base(&self) -> f64 {
        0.0
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Quantity<U: Unit> {
    pub amount: f64,
    pub units: U,
}

pub trait Convertable<U: Unit> {
    /// Converts a quantity to the given unit
    fn convert(&self, to: U) -> Self;
}

impl<U: Unit> Convertable<U> for Quantity<U> {
    fn convert(&self, to: U) -> Self {
        let base_amount =
            (self.amount + self.units.offset_from_base()) * self.units.multiplier_to_base();
        let new_amount = (base_amount / to.multiplier_to_base()) - to.offset_from_base();
        Quantity {
            amount: new_amount,
            units: to,
        }
    }
}

// Implement multiplication operator for Quantity
impl<U: Unit> std::ops::Mul<f64> for Quantity<U> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Quantity {
            amount: self.amount * rhs,
            units: self.units,
        }
    }
}

impl<U: Unit> std::ops::Mul<Quantity<U>> for f64 {
    type Output = Quantity<U>;

    fn mul(self, rhs: Quantity<U>) -> Quantity<U> {
        Quantity {
            amount: self * rhs.amount,
            units: rhs.units,
        }
    }
}

impl<U: Unit> std::ops::Div<f64> for Quantity<U> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Quantity {
            amount: self.amount / rhs,
            units: self.units,
        }
    }
}

impl<U: Unit> std::ops::Add for Quantity<U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let converted = rhs.convert(self.units);
        Quantity {
            amount: self.amount + converted.amount,
            units: self.units,
        }
    }
}

impl<U: Unit> std::ops::Sub for Quantity<U> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let converted = rhs.convert(self.units);
        Quantity {
            amount: self.amount - converted.amount,
            units: self.units,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum TestUnit {
        Base,
    }

    impl Unit for TestUnit {
        fn multiplier_to_base(&self) -> f64 {
            return 1.0;
        }
    }

    #[test]
    fn test_quantity_mul_f64() {
        let q = Quantity {
            amount: 10.0,
            units: TestUnit::Base,
        };
        assert_eq!(
            q * 2.0,
            Quantity {
                amount: 20.0,
                units: TestUnit::Base,
            }
        );
    }

    #[test]
    fn test_f64_mul_quantity() {
        let q = Quantity {
            amount: 10.0,
            units: TestUnit::Base,
        };
        assert_eq!(
            2.0 * q,
            Quantity {
                amount: 20.0,
                units: TestUnit::Base,
            }
        );
    }

    #[test]
    fn test_quantity_div_f64() {
        let q = Quantity {
            amount: 10.0,
            units: TestUnit::Base,
        };
        assert_eq!(
            q / 2.0,
            Quantity {
                amount: 5.0,
                units: TestUnit::Base,
            }
        );
    }

    #[test]
    fn test_quantity_add() {
        let q1 = Quantity {
            amount: 10.0,
            units: TestUnit::Base,
        };
        let q2 = Quantity {
            amount: 20.0,
            units: TestUnit::Base,
        };
        assert_eq!(
            q1 + q2,
            Quantity {
                amount: 30.0,
                units: TestUnit::Base,
            }
        );
    }

    #[test]
    fn test_quantity_add_different_units() {
        let q1 = Quantity {
            amount: 10.0,
            units: TestUnit::Base,
        };
        let q2 = Quantity {
            amount: 20.0,
            units: TestUnit::Base,
        };
        assert_eq!(
            q1 + q2,
            Quantity {
                amount: 30.0,
                units: TestUnit::Base,
            }
        );
    }

    #[test]
    fn test_quantity_sub() {
        let q1 = Quantity {
            amount: 30.0,
            units: TestUnit::Base,
        };
        let q2 = Quantity {
            amount: 20.0,
            units: TestUnit::Base,
        };
        assert_eq!(
            q1 - q2,
            Quantity {
                amount: 10.0,
                units: TestUnit::Base,
            }
        );
    }

    #[test]
    fn test_quantity_sub_different_units() {
        let q1 = Quantity {
            amount: 30.0,
            units: TestUnit::Base,
        };
        let q2 = Quantity {
            amount: 20.0,
            units: TestUnit::Base,
        };
        assert_eq!(
            q1 - q2,
            Quantity {
                amount: 10.0,
                units: TestUnit::Base,
            }
        );
    }
}
