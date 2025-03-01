pub trait Abs {
    fn abs(&self) -> Self;
}

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
