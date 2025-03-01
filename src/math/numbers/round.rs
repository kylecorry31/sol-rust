pub trait Round {
    fn round(&self) -> Self;
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
