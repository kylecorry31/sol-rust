use super::{Matrix, RawTensor, add, cross, divide, dot, multiply, scale, subtract, transpose};

pub trait Tensor {
    fn get_tensor(&self) -> &RawTensor;
    fn dot<T: Tensor>(&self, other: &T) -> f32 {
        dot(self.get_tensor(), other.get_tensor())
    }
    fn cross<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::new(cross(self.get_tensor(), other.get_tensor()))
    }
    fn add<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::new(add(self.get_tensor(), other.get_tensor()))
    }
    fn transpose(&self) -> Matrix {
        Matrix::new(transpose(self.get_tensor()))
    }
    fn subtract<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::new(subtract(self.get_tensor(), other.get_tensor()))
    }
    fn multiply<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::new(multiply(self.get_tensor(), other.get_tensor()))
    }
    fn divide<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::new(divide(self.get_tensor(), other.get_tensor()))
    }
    fn scale(&self, factor: f32) -> Matrix {
        Matrix::new(scale(self.get_tensor(), factor))
    }
    fn value(&self) -> f32 {
        self.get_tensor()[0][0]
    }
}
