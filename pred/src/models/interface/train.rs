use crate::models::linear::v1::linear_domain::{LinearModel, LinearModelRecord};
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::module::Module;
use burn::prelude::{Backend, Tensor};
use burn::record::{BinBytesRecorder, FullPrecisionSettings, Recorder};
use burn::tensor::backend::AutodiffBackend;
use burn::LearningRate;

pub trait Train <B, const D1: usize,const D2: usize> : Module<B>
where
    B: AutodiffBackend<Device = NdArrayDevice, FloatElem = f32>
        + From<Autodiff<NdArray>>
        + Into<Autodiff<NdArray>>,
{
    fn forward(&self, x: Tensor<B, D1>) -> Tensor<B, D2>;
    fn train_step(&mut self, x: Tensor<B, D1>, y: Tensor<B, D2>, lr: LearningRate) -> Self;
    fn record_as_bytes(&self) -> Result<Vec<u8>, String> {
        let recorder = BinBytesRecorder::<FullPrecisionSettings>::new();
        let record = self.clone().into_record();
        let bytes = recorder
            .record(record, ())
            .map_err(|e| format!("{:?}", e))?;
        Ok(bytes)
    }
    fn restore_from_bytes(&self, record: Vec<u8>) -> Result<LinearModel<B>, String> {
        let recorder = BinBytesRecorder::<FullPrecisionSettings>::new();
        let record = recorder
            .load::<LinearModelRecord<B>>(record, &NdArrayDevice::Cpu)
            .map_err(|e| format!("{:?}", e))?;
        let model = LinearModel::default();
        let module = model.load_record(record);
        Ok(module)
    }
}
