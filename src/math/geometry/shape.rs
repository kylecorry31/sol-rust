pub trait Shape2D {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn vertex_count(&self) -> usize;
}
