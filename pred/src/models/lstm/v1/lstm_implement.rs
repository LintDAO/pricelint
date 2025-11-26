use crate::models::interface::predict::Predict;
use crate::models::interface::train::Train;
use crate::models::interface::validate::Validate;
use crate::models::linear::v1::linear_domain::LinearModel;
use crate::models::lstm::v1::lstm_domain::LstmModel;
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::grad_clipping::GradientClippingConfig;
use burn::optim::decay::WeightDecayConfig;
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::prelude::{Backend, Tensor};
use burn::tensor::backend::AutodiffBackend;
use burn::LearningRate;

impl<B> Train<B, 3, 2> for LstmModel<B>
where
    B: AutodiffBackend<Device = NdArrayDevice, FloatElem = f32> + From<Autodiff<NdArray>>,
    Autodiff<NdArray>: From<B>,
{
    /// 定义前向传播
    /// 输入 x: [batch_size, seq_len, input_size] (3D 张量)
    /// 输出 y: [batch_size, output_size] (2D 张量)
    fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
        // input: [batch, seq_len, input_size]
        let (x, _) = self.lstm1.forward(input, None); // 第一层 LSTM
        let x = self.dropout.forward(x); // Dropout
        let (_, hidden) = self.lstm2.forward(x, None); // 第二层 LSTM（只取 hidden）
        let output = self.dense.forward(hidden.hidden); // Linear 输出
        ic_cdk::println!("forward{}", output);
        output // [batch, output_size]
    }
    fn train_step(
        &mut self,
        input: Tensor<B, 3>,  // 输入数据，形状 [Batch, Seq, Feature]
        target: Tensor<B, 2>, // 目标值，形状 [Batch, OutputSize]
        lr: LearningRate,     // 学习率
    ) -> (Self,Vec<(u64,String)>) {
        let output = self.forward(input.clone()); // 预测值 y_hat
        let loss = self.mse_loss(&output, &target);
        let grads = loss.backward(); // 计算梯度
        let grads = GradientsParams::from_grads(grads, self);
        let adam_config = AdamConfig::new()
            .with_grad_clipping(Some(GradientClippingConfig::Norm(1.0)))
            .with_weight_decay(Some(WeightDecayConfig::new(1e-5))); // L2正则化
        let mut optim = adam_config.init::<B, LstmModel<B>>();
        let new_model = optim.step(lr, self.clone(), grads);
        *self = new_model;
        (self.clone(),vec![])
    }
}
impl<B, const D: usize> Validate<B, D> for LstmModel<B>
where
    B: Backend<FloatElem = f32>,
{
    fn mse_loss(&self, pred: &Tensor<B, D>, targets: &Tensor<B, D>) -> Tensor<B, 1> {
        pred.clone().sub(targets.clone()).powf_scalar(2.0).mean()
    }

    fn mre_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>) -> Tensor<B, 1> {
        // 1. 计算绝对误差: |predictions - targets|
        let abs_error = predictions.clone().sub(targets.clone()).abs();
        // 2. 计算分母 |target| + epsilon，防止除以零
        // 使用 1e-8 作为平滑项 epsilon
        let epsilon = 1e-8;
        let denominator = targets.clone().abs().add_scalar(epsilon);
        // 3. 计算相对误差: |error| / denominator
        // Burn 的张量除法自动按元素进行
        let relative_error = abs_error.div(denominator);
        // 4. 取平均值 (Mean) 得到 MRE Loss
        relative_error.mean()
    }
}
impl<B> Predict<B, 3, 2> for LstmModel<B>
where
    B: AutodiffBackend<Device = NdArrayDevice, FloatElem = f32> + From<Autodiff<NdArray>>,
    Autodiff<NdArray>: From<B>,
{
    fn predict(&self, input: Tensor<B, 3>) -> f32 {
        self.forward(input).into_scalar()
    }
}
