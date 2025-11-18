use burn::nn;
use burn::nn::Initializer;
use burn::optim::AdamConfig;
use burn::prelude::{Backend, Config, Module, Tensor};

//样本
#[derive(Debug, Clone, Default)]
pub struct PriceSample {
    token_name: String,
    target_token_name: String,
    time: u64,
    exchange_rate: u64,
}

//数据集
pub struct PriceDataset {
    pub(crate) samples: Vec<PriceSample>,
}

//具体模型
#[derive(Module, Debug)]
pub struct LinearModel<B: Backend> {
    linear: nn::Linear<B>, // #[module(ignore)]
                           // check_point: CheckPoint<B>,
}
// 模型配置
#[derive(Config, Debug)]
pub struct LinearModelConfig {
    // 偏置项
    #[config(default = true)]
    pub bias: bool,
    #[config(default = 3)]
    pub input_size: usize,
    #[config(default = 1)]
    pub output_size: usize,
    #[config(default = "Initializer::KaimingUniform { gain: 0.57735, fan_out_only: false }")]
    pub initializer: Initializer,
}

//存储模型信息
#[derive(Debug)]
pub struct CheckPoint<B: Backend> {
    config: LinearModelConfig,
    // optim: AdamConfig,
    backend: B,
}

impl PriceSample {}

impl PriceDataset {
    pub fn new(samples: Vec<PriceSample>) -> Self {
        Self { samples }
    }
    fn add(&mut self, sample: PriceSample) {
        self.samples.push(sample);
    }
    fn len(&self) -> usize {
        self.samples.len()
    }
    fn as_vec(&self) -> Vec<PriceSample> {
        self.samples.clone()
    }
}

impl<B: Backend> LinearModel<B> {
    pub fn new() -> Self {
        let config = LinearModelConfig::new();
        //// 3个输入特征 token price time，1个输出 price
        let linear = nn::LinearConfig::new(config.input_size, config.output_size)
            .with_bias(config.bias)
            .with_initializer(config.initializer)
            .init(&Default::default());
        Self { linear }
    }

    pub fn forward(&self, x: Tensor<B, 2>) -> Tensor<B, 2> {
        self.linear.forward(x)
    }

    pub fn device(&self) -> B::Device {
        B::Device::default()
    }
    pub fn backend(&self) -> B {
        B::default()
    }

    // pub fn config(&self) -> nn::LinearConfig {
    //     self.linear
    // }
    // pub fn backward(&self) -> Tensor<B, 2> {
    //     self.linear.backward()
    // }
}
