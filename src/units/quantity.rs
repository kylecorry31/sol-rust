pub trait Unit: Copy + Clone + PartialEq + Eq {
    fn multiplier_to_base(&self) -> f32;
    fn offset_from_base(&self) -> f32 {
        return 0.0;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Quantity<U: Unit> {
    pub amount: f32,
    pub units: U,
}

pub trait Convertable<U: Unit> {
    /// Converts a quantity's unit to the given unit
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
impl<U: Unit> std::ops::Mul<f32> for Quantity<U> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Quantity {
            amount: self.amount * rhs,
            units: self.units,
        }
    }
}

impl<U: Unit> std::ops::Mul<Quantity<U>> for f32 {
    type Output = Quantity<U>;

    fn mul(self, rhs: Quantity<U>) -> Quantity<U> {
        Quantity {
            amount: self * rhs.amount,
            units: rhs.units,
        }
    }
}
