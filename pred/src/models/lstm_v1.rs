use burn::backend::ndarray::NdArrayDevice;
use burn::backend::Autodiff;
use burn::module::Module;
use burn::nn::{Dropout, DropoutConfig, GateController, Linear, LinearConfig, Lstm, LstmConfig};
use burn::prelude::Backend;
use burn::tensor::backend::AutodiffBackend;
use burn::{TestAutodiffBackend, TestBackend};
use burn::tensor::Tensor;

#[derive(Module, Debug)]
struct LstmModel<B: Backend> {
    lstm1: Lstm<B>,    // 第一层LSTM（return_sequences=True）
    dropout1: Dropout, // Dropout率=0.2
    lstm2: Lstm<B>,    // 第二层LSTM
    dense: Linear<B>,  // 全连接输出层
}

impl Default for LstmModel<Autodiff<NdArrayDevice>> {
    fn default() -> Self {
        let device = Autodiff::<NdArrayDevice>::default();

        Self {
            lstm1: LstmConfig::new(1, 50, true).init(&device),
            dropout1: DropoutConfig::new(0.2).init(),
            lstm2: LstmConfig::new(50, 50, false).init(&device),
            dense: LinearConfig::new(50, 1).init(&device),
        }
    }
}

impl<B: Backend> LstmModel<B> {
    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
        // 第一层LSTM
        let (output_seq, _) = self.lstm1.forward(input, None);

        // Dropout正则化
        let output_seq = self.dropout1.forward(output_seq);

        // 第二层LSTM
        let (output_seq, hidden_state) = self.lstm2.forward(output_seq, None);

        // 方案1：直接使用最后隐藏状态（高效）
        // let final_output = hidden_state; // [batch_size, hidden_size]

        // 方案2：取最后一个时间步的输出（更精确）
        let final_output = self.select_last_timestep(output_seq);

        // 全连接层预测
        self.dense.forward(final_output)
    }

    /// 提取序列最后一个时间步
    fn select_last_timestep(&self, seq: Tensor<B, 3>) -> Tensor<B, 2> {
        let [batch, seq_len, features] = seq.dims();
        seq.slice([0..batch, (seq_len-1)..seq_len, 0..features])
            .reshape([batch, features])
    }
}
// 自动微分特化实现
impl<B: AutodiffBackend> LstmModel<B> {
    /// 训练专用前向传播（保留计算图）
    pub fn forward_train(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
        // 此处可添加训练专用逻辑
        self.forward(input)
    }
}