use crate::models::interface::predict::Predict;
use crate::models::interface::train::Train;
use crate::models::interface::validate::Validate;
use crate::models::linear::v1::linear_domain::{
    AdamConfigWrap, LinearModel, LinearModelRecord, OptimizerConfigs
};
use burn::backend::ndarray::{NdArray, NdArrayDevice};
use burn::backend::Autodiff;
use burn::data::dataloader::{DataLoader, Dataset};
use burn::grad_clipping::GradientClippingConfig;
use burn::optim::adaptor::OptimizerAdaptor;
use burn::optim::{Adam, AdamConfig, GradientsParams, Optimizer};
use burn::prelude::*;
use burn::prelude::{Backend, Device, Module, Tensor};
use burn::record::{BinBytesRecorder, FullPrecisionSettings, Record, Recorder};
use burn::tensor::backend::AutodiffBackend;
use burn::tensor::TensorKind;
use burn::{nn, LearningRate};
use std::fmt::{Debug, Formatter};



impl<B, const D: usize> Train<B, D, D> for LinearModel<B>
where
    B: AutodiffBackend<Device = NdArrayDevice, FloatElem = f32>
        + From<Autodiff<NdArray>>
        + Into<Autodiff<NdArray>>,
   
{
    //向前传播
    fn forward(&self, x: Tensor<B, D>) -> Tensor<B, D> {
        self.linear.forward(x)
    }

    ///1.前向传播 (Forward Pass)
    // 2.计算损失 (Loss Calculation)
    // 3.反向传播 (Backward Pass)
    // 4.优化器更新 (Optimizer Step)
    fn train_step(
        &mut self,
        input: Tensor<B, D>,
        target: Tensor<B, D>,
        lr: LearningRate,
    ) -> (Self,Vec<(u64,String)>) {
        let output = self.forward(input.clone());
        let loss = self.mse_loss(&output, &target);
        let grads = loss.backward();

        let grads = GradientsParams::from_grads(grads, self);
        ic_cdk::println!("loss:[{}]", loss);
        ic_cdk::println!("input:[{}]]", input);
        ic_cdk::println!("output:[{}]", output);
        ic_cdk::println!("target:[{}]", target.clone());
        ic_cdk::println!("gards:[{:?}]", grads);

        // 定义裁剪配置：使用 L2 Norm，最大范数为 10.0
        let grad_clipping_config = GradientClippingConfig::Value(10.0);
        let adam_config = AdamConfig::new().with_grad_clipping(Some(grad_clipping_config));
        let mut optim = adam_config.init::<B, LinearModel<B>>();
        *self = optim.step(lr, self.clone(), grads);
        (self.clone(),vec![])
    }

    fn record_as_bytes(&self) -> Result<Vec<u8>, String> {
        let recorder = BinBytesRecorder::<FullPrecisionSettings>::new();
        let record = self.clone().into_record();
        let bytes = recorder
            .record(record, ())
            .map_err(|e| format!("{:?}", e))?;
        Ok(bytes)
    }


}
impl<B, const D: usize> Validate<B, D> for LinearModel<B>
where
    B: Backend<FloatElem = f32>,
{
    fn mse_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>) -> Tensor<B, 1> {
        let difference = predictions.clone().sub(targets.clone());
        let squared_difference = difference.powf_scalar(2);
        // 对所有元素的平方差求平均
        let loss_tensor = squared_difference.mean();
        // 转换为FloatElem
        loss_tensor
    }

    fn mre_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>) -> Tensor<B, 1> {
        // 1. 计算绝对误差
        let abs_error = predictions.clone().sub(targets.clone()).abs();

        // 2. 计算 |target|，并防止除 0（极小值替代）
        let denominator = targets.clone().abs().add_scalar(1e-8);

        // 3. 相对误差
        let relative_error = abs_error / denominator;

        // 4. 取平均 → MRE
        relative_error.mean()
    }
}


