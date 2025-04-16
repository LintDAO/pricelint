use burn::{
    module::{Module, Param},
    tensor::{backend::Backend, Tensor},
    tensor::backend::AutodiffBackend,
    nn::{Linear, LinearConfig},
    optim::{Optimizer, SgdConfig, GradientsParams},
};

#[derive(Module, Debug)]
pub struct PricePredictor<B: Backend> {
    linear: Linear<B>,
    sequence_length: usize,
}

impl<B: Backend> PricePredictor<B> where B: AutodiffBackend<Device = burn::backend::ndarray::NdArrayDevice> {
    
    //实例
    pub fn new(input_size: usize, output_size: usize, sequence_length: usize) -> Self {
        let linear = LinearConfig::new(input_size * sequence_length, output_size) // [5, 1]
            .with_bias(true)
            .init(&burn::backend::ndarray::NdArrayDevice::Cpu);
        Self { linear, sequence_length }
    }

    //权重和偏置实例
    pub fn from_weights(weights: Vec<f32>, bias: Vec<f32>, sequence_length: usize) -> Self {
        let input_size = 1;
        let output_size = bias.len(); // 应为 1
        let weight_tensor = Tensor::<B, 1>::from_floats(weights.as_slice(), &burn::backend::ndarray::NdArrayDevice::Cpu)
            .reshape([input_size * sequence_length, output_size]); // [5, 1]
        let bias_tensor = Tensor::<B, 1>::from_floats(bias.as_slice(), &burn::backend::ndarray::NdArrayDevice::Cpu);
        let linear = Linear {
            weight: Param::from_tensor(weight_tensor),
            bias: Some(Param::from_tensor(bias_tensor)),
        };
        Self { linear, sequence_length }
    }

    //前向传播
    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
        let [batch, seq, features] = input.dims();
        let input = input.reshape([batch, seq * features]); // [1, 5]
        self.linear.forward(input) // 输出 [1, 1]
    }

    //训练
    pub fn train(&mut self, inputs: Tensor<B, 3>, targets: Tensor<B, 2>, learning_rate: f32, epochs: usize) {
        let config = SgdConfig::new();
        let mut optimizer = config.init();
        for _ in 0..epochs {
            let predictions = self.forward(inputs.clone());
            let loss = predictions.sub(targets.clone()).powf_scalar(2.0).mean();
            let grad = GradientsParams::from_grads(loss.backward(), &self.linear);
            self.linear = optimizer.step(learning_rate as f64, self.linear.clone(), grad);
        }
    }

    pub fn get_weights(&self) -> &Param<Tensor<B, 2>> {
        &self.linear.weight
    }

    pub fn get_bias(&self) -> Option<&Param<Tensor<B, 1>>> {
        self.linear.bias.as_ref()
    }
}