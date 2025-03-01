use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

pub trait Abs {
    fn abs(&self) -> Self;
}

pub trait Round {
    fn round(&self) -> Self;
}

pub trait Power {
    fn powi(&self, exp: i32) -> Self;
    fn powf(&self, exp: Self) -> Self;
}

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
    + Abs
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
pub trait Real: Number + Round + Power {}

impl Abs for f32 {
    fn abs(&self) -> Self {
        (*self).abs()
    }
}

impl Abs for f64 {
    fn abs(&self) -> Self {
        (*self).abs()
    }
}

impl Abs for i32 {
    fn abs(&self) -> Self {
        (*self).abs()
    }
}

impl Abs for i64 {
    fn abs(&self) -> Self {
        (*self).abs()
    }
}

impl Round for f32 {
    fn round(&self) -> Self {
        (*self).round()
    }
}

impl Round for f64 {
    fn round(&self) -> Self {
        (*self).round()
    }
}

impl Power for f32 {
    fn powi(&self, exp: i32) -> Self {
        (*self).powi(exp)
    }
    fn powf(&self, exp: Self) -> Self {
        (*self).powf(exp)
    }
}

impl Power for f64 {
    fn powi(&self, exp: i32) -> Self {
        (*self).powi(exp)
    }
    fn powf(&self, exp: Self) -> Self {
        (*self).powf(exp)
    }
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

impl Real for f32 {}
impl Real for f64 {}
