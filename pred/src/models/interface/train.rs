use crate::models::linear::v1::domain::LinearModel;
use burn::prelude::{Backend, Tensor};
use burn::record::{BinBytesRecorder, FullPrecisionSettings, Recorder};

pub trait Train<B: Backend, const D: usize> {
    fn forward(&self, x: Tensor<B, D>) -> Tensor<B, D>;
    fn grads(&self) -> f32;
    fn record_as_bytes(&self) -> Result<Vec<u8>, String>;
    fn restore_from_bytes(&self, record: Vec<u8>) -> Result<LinearModel<B>, String>;
}
