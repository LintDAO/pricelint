use crate::models::linear::v1::linear_domain::{LinearModelConfig, OptimizerConfigs};
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::config::Config;
use burn::module::{Ignored, Module};
use burn::nn::{Dropout, DropoutConfig, Initializer, Linear, LinearConfig, Lstm, LstmConfig};
use burn::tensor::backend::Backend;
use burn::tensor::Tensor;

/// 定义 LSTM 模型结构
/// 结构: Input -> LSTM -> (取最后一个时间步) -> Linear -> Output
/// LSTM 模型的模块结构体
#[derive(Module, Debug)]
pub struct LstmModel<B: Backend> {
    pub lstm1: Lstm<B>,
    pub dropout: Dropout,
    pub lstm2: Lstm<B>,
    pub dense: Linear<B>,
    pub config: Ignored<GlobalConfig>,
}
#[derive(Debug, Clone)]
pub struct GlobalConfig {
    pub backend: Autodiff<NdArray>,
    pub device: NdArrayDevice,
    pub lstm_config: LstmModelConfig,
}
#[derive(Config, Debug)]
pub struct LstmModelConfig {
    /// 输入特征的大小
    #[config(default = 5)]
    pub input_size: usize,
    /// 隐藏状态的大小
    #[config(default = 64)]
    pub hidden_size: usize,
    // 最终输出
    #[config(default = 1)]
    pub output_size: usize,
    /// Dropout 比率
    #[config(default = 0.2)]
    pub dropout: f64,

    #[config(default = "Initializer::XavierNormal{gain:1.0}")]
    pub lstm_initializer: Initializer,

    #[config(default = "Initializer::KaimingUniform { gain: 1.414, fan_out_only: false }")]
    pub dense_initializer: Initializer,

}

impl<B: Backend> LstmModel<B> {
    pub fn default() -> Self {
        let device = &B::Device::default();
        let config = LstmModelConfig::new();
        let config_clone=config.clone();
        // 对于LSTM的tanh激活，使用适合的gain
        let lstm_gain = 5.0 / 3.0;

        let lstm1 = LstmConfig::new(config.input_size, config.hidden_size, true)
            .with_initializer(config.clone().lstm_initializer)
            .init(device);

        let dropout = DropoutConfig::new(config.dropout).init();

        let lstm2 = LstmConfig::new(config.hidden_size, config.hidden_size, true)
            .with_initializer(config.lstm_initializer)
            .init(device);

        // 对于输出层（通常是线性激活），使用gain=1.0
        let dense = LinearConfig::new(config.hidden_size, config.output_size)
            .with_initializer(config.dense_initializer)
            .init(device);

        Self {
            lstm1,
            dropout,
            lstm2,
            dense,
            config: Ignored(GlobalConfig {
                backend: Default::default(),
                device: Default::default(),
                lstm_config: config_clone,
            }),
        }
    }

}
