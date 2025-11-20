use crate::models::interface::predict::Predict;
use crate::models::interface::train::Train;
use crate::models::interface::validate::Validate;
use crate::models::linear::v1::domain::{
    AdamConfigWrap, LinearModel, LinearModelRecord, PriceDataset, PriceSample,
};
use burn::backend::ndarray::{NdArray, NdArrayDevice};
use burn::config::Config;
use burn::data::dataloader::{DataLoader, Dataset};
use burn::grad_clipping::GradientClippingConfig;
use burn::nn;
use burn::nn::{Initializer, Linear, LinearConfig};
use burn::optim::decay::WeightDecayConfig;
use burn::optim::{AdamConfig, Optimizer};
use burn::prelude::*;
use burn::prelude::{Backend, Device, Module, Tensor};
use burn::record::{BinBytesRecorder, FullPrecisionSettings, Recorder};
use burn::tensor::backend::AutodiffBackend;
use ic_cdk::update;
use std::fmt::{Debug, Formatter};

impl Dataset<PriceSample> for PriceDataset {
    fn get(&self, index: usize) -> Option<PriceSample> {
        self.samples.get(index).cloned()
    }
    fn len(&self) -> usize {
        self.samples.len()
    }
}

impl<B, const D: usize> Train<B, D> for LinearModel<B>
where
    B: Backend<Device = NdArrayDevice, FloatElem = f32> + From<NdArray> + Into<NdArray>,
{
    //向前传播
    fn forward(&self, x: Tensor<B, D>) -> Tensor<B, D> {
        self.linear.forward(x)
    }

    // 实现 mse_loss

    fn grads(&self) -> f32 {
        todo!()
    }

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
            .load::<LinearModelRecord<B>>(record, &self.config.device)
            .map_err(|e| format!("{:?}", e))?;
        let model = LinearModel::default();
        let module = model.load_record(record);
        Ok(module)
    }
}
impl<B, const D: usize> Validate<B, D> for LinearModel<B>
where
    B: Backend<FloatElem = f32>,
{
    fn mse_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>) -> f32 {
        let difference = predictions.clone().sub(targets.clone());
        let squared_difference = difference.powf_scalar(2);
        // 对所有元素的平方差求平均
        let loss_tensor = squared_difference.mean();
        // 转换为FloatElem
        loss_tensor.into_scalar()
    }

    fn mre_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>) -> f32 {
        // 1. 计算绝对误差
        let abs_error =predictions.clone().sub(targets.clone()).abs();

        // 2. 计算 |target|，并防止除 0（极小值替代）
        let denominator = targets.clone().abs().add_scalar(1e-8);

        // 3. 相对误差
        let relative_error = abs_error / denominator;

        // 4. 取平均 → MRE
        relative_error.mean().into_scalar()
    }
}

impl<B: Backend, I: Send + Sync> Predict<B, I> for LinearModel<B> {
    fn predict(&self, input: I) -> f32 {
        1.0
    }

    fn predicts(&self, inputs: &dyn Dataset<I>) -> Vec<f32> {
        let mut predictions = Vec::new();
        for i in 0..inputs.len() {
            let input = inputs.get(i);
            if let Some(input) = input {
                predictions.push(self.predict(input));
            }
        }
        predictions
    }
}
