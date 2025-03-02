use super::{Matrix, RawTensor, add, apply, cross, divide, dot, multiply, subtract, transpose};

pub trait Tensor {
    fn get_tensor(&self) -> &RawTensor;
    fn dot<T: Tensor>(&self, other: &T) -> f32 {
        dot(self.get_tensor(), other.get_tensor())
    }
    fn cross<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::from(cross(self.get_tensor(), other.get_tensor()))
    }
    fn add<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::from(add(self.get_tensor(), other.get_tensor()))
    }
    fn transpose(&self) -> Matrix {
        Matrix::from(transpose(self.get_tensor()))
    }
    fn subtract<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::from(subtract(self.get_tensor(), other.get_tensor()))
    }
    fn multiply<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::from(multiply(self.get_tensor(), other.get_tensor()))
    }
    fn divide<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::from(divide(self.get_tensor(), other.get_tensor()))
    }
    fn apply(&self, operation: impl Fn(f32) -> f32) -> Matrix {
        Matrix::from(apply(self.get_tensor(), operation))
    }
    fn value(&self) -> f32 {
        self.get_tensor()[0][0]
    }
}
