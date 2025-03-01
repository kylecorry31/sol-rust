pub trait Power {
    fn powi(&self, exp: i32) -> Self;
    fn powf(&self, exp: Self) -> Self;
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
