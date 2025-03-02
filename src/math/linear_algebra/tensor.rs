use super::RawTensor;

pub trait Tensor {
    fn get_tensor(&self) -> &RawTensor;
}
