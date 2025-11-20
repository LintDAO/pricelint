use crate::common::errors::ConfigError;
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::NdArray;
use burn::module::{Devices, Ignored, ModuleMapper, ModuleVisitor, Param};
use burn::nn;
use burn::nn::{Initializer, Linear, LinearConfig};
use burn::optim::{Adam, AdamConfig, SgdConfig};
use burn::prelude::{Backend, Config, Device, Module, Tensor};
use burn::tensor::backend::AutodiffBackend;
use serde::Serialize;
use std::fmt::{Debug, DebugList, Display, Formatter, Write};
use std::marker::PhantomData;

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
    pub linear: nn::Linear<B>,
    pub config: Ignored<GlobalConfig>,
}

//配置汇总
#[derive(Debug, Clone)]
pub struct GlobalConfig {
    pub backend: NdArray,
    pub device: NdArrayDevice,
    pub optim: OptimizerConfigs,
    pub linear_config: LinearModelConfig,
}
#[derive(Debug, Clone)]
pub enum OptimizerConfigs {
    Adam(AdamConfigWrap),
    // Sgd(SgdConfigWrap),
}
#[derive(Clone)]
pub struct AdamConfigWrap(pub AdamConfig);

impl Debug for AdamConfigWrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("AdamConfigWrap({:?})", self.0.to_string()))
    }
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

impl<B> LinearModel<B>
where
    B: Backend<Device = NdArrayDevice> + Into<NdArray> + From<NdArray>,
{
    pub fn default() -> Self {
        let config = LinearModelConfig::new();
        let optim = AdamConfig::new();
        //// 3个输入特征 token price time，1个输出 price
        let linear = nn::LinearConfig::new(config.input_size, config.output_size)
            .with_bias(config.bias)
            .with_initializer(config.initializer)
            .init(&Default::default());
        Self {
            linear,
            config: Ignored(GlobalConfig {
                optim: OptimizerConfigs::Adam(AdamConfigWrap(optim)),
                linear_config: LinearModelConfig::new(),
                backend: B::default().into(),
                device: B::Device::default().into(),
            }),
        }
    }
    pub fn new(
        config: LinearModelConfig,
        optim: OptimizerConfigs,
        device: B::Device,
    ) -> Self {
        let linear = nn::LinearConfig::new(config.input_size, config.output_size)
            .with_bias(config.bias)
            .with_initializer(config.initializer)
            .init(&device);
        Self {
            linear,
            config: Ignored(GlobalConfig {
                optim,
                linear_config: LinearModelConfig::new(),
                backend: B::default().into(),
                device,
            }),
        }
    }

    pub fn get_device(&self) -> B::Device {
        self.config.device.into()
    }
    pub fn convert_to_linear_config(&self) -> LinearConfig {
        LinearConfig::new(
            self.config.linear_config.input_size,
            self.config.linear_config.output_size,
        )
    }

    pub fn linear_from_config(&self) -> Linear<B> {
        let config = self.config.linear_config.clone();
        //// 3个输入特征 token price time，1个输出 price
        let linear = nn::LinearConfig::new(config.input_size, config.output_size)
            .with_bias(config.bias)
            .with_initializer(config.initializer)
            .init(&self.config.device.into());
        linear
    }
    pub fn get_weights(&self) -> &Param<Tensor<B, 2>> {
        &self.linear.weight
    }

    pub fn get_bias(&self) -> Option<&Param<Tensor<B, 1>>> {
        self.linear.bias.as_ref()
    }
    pub fn default_backend() -> B {
        B::default()
    }
    pub fn default_device() -> B::Device {
        B::Device::default()
    }
}
