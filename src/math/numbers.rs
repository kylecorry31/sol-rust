use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

pub trait Number:
    Add<Output = Self>
    + Sub<Output = Self>
    + Rem<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + PartialEq
    + PartialOrd
    + Copy
    + Default
{
    fn from_i32(value: i32) -> Self;
    fn from_i64(value: i64) -> Self;
    fn from_f32(value: f32) -> Self;
    fn from_f64(value: f64) -> Self;
    fn as_f32(&self) -> f32;
    fn as_f64(&self) -> f64;
    fn as_i32(&self) -> i32;
    fn as_i64(&self) -> i64;
}

pub trait Integer: Number {}
pub trait Real: Number {
    fn round(&self) -> Self;
}

impl Number for f32 {
    fn from_i32(value: i32) -> Self {
        value as f32
    }
    fn from_i64(value: i64) -> Self {
        value as f32
    }
    fn from_f32(value: f32) -> Self {
        value
    }
    fn from_f64(value: f64) -> Self {
        value as f32
    }
    fn as_f32(&self) -> f32 {
        *self
    }
    fn as_f64(&self) -> f64 {
        *self as f64
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
    fn as_i64(&self) -> i64 {
        *self as i64
    }
}
impl Number for f64 {
    fn from_i32(value: i32) -> Self {
        value as f64
    }
    fn from_i64(value: i64) -> Self {
        value as f64
    }
    fn from_f32(value: f32) -> Self {
        value as f64
    }
    fn from_f64(value: f64) -> Self {
        value
    }
    fn as_f32(&self) -> f32 {
        *self as f32
    }
    fn as_f64(&self) -> f64 {
        *self
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
    fn as_i64(&self) -> i64 {
        *self as i64
    }
}
impl Number for i32 {
    fn from_i32(value: i32) -> Self {
        value
    }
    fn from_i64(value: i64) -> Self {
        value as i32
    }
    fn from_f32(value: f32) -> Self {
        value as i32
    }
    fn from_f64(value: f64) -> Self {
        value as i32
    }
    fn as_f32(&self) -> f32 {
        *self as f32
    }
    fn as_f64(&self) -> f64 {
        *self as f64
    }
    fn as_i32(&self) -> i32 {
        *self
    }
    fn as_i64(&self) -> i64 {
        *self as i64
    }
}
impl Number for i64 {
    fn from_i32(value: i32) -> Self {
        value as i64
    }
    fn from_i64(value: i64) -> Self {
        value
    }
    fn from_f32(value: f32) -> Self {
        value as i64
    }
    fn from_f64(value: f64) -> Self {
        value as i64
    }
    fn as_f32(&self) -> f32 {
        *self as f32
    }
    fn as_f64(&self) -> f64 {
        *self as f64
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
    fn as_i64(&self) -> i64 {
        *self
    }
}

impl Integer for i32 {}
impl Integer for i64 {}

impl Real for f32 {
    fn round(&self) -> Self {
        (*self).round()
    }
}
impl Real for f64 {
    fn round(&self) -> Self {
        (*self).round()
    }
}
