use super::{Matrix, RawTensor, add, cross, dot, subtract};

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
    fn sub<T: Tensor>(&self, other: &T) -> Matrix {
        Matrix::new(subtract(self.get_tensor(), other.get_tensor()))
    }
    fn value(&self) -> f32 {
        self.get_tensor()[0][0]
    }
}
