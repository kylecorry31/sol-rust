pub trait Number:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Neg<Output = Self>
    + PartialEq
    + PartialOrd
    + Copy
    + Default
{
    fn from_i32(value: i32) -> Self;
}

pub trait Integer: Number {}
pub trait Real: Number {}

impl Number for f32 {
    fn from_i32(value: i32) -> Self {
        value as f32
    }
}
impl Number for f64 {
    fn from_i32(value: i32) -> Self {
        value as f64
    }
}
impl Number for i32 {
    fn from_i32(value: i32) -> Self {
        value
    }
}
impl Number for i64 {
    fn from_i32(value: i32) -> Self {
        value as i64
    }
}

impl Integer for i32 {}
impl Integer for i64 {}

impl Real for f32 {}
impl Real for f64 {}
