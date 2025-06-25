use std::ops::Deref;
use burn::data::dataset::Dataset;
use burn::optim::SimpleOptimizer;
use burn::tensor::TensorData;
use burn::{
    backend::{Autodiff, NdArray},
    module::Module,
    nn::{Dropout, DropoutConfig, Linear, LinearConfig, Lstm, LstmConfig},
    optim::{Adam, AdamConfig},
    record::{BinFileRecorder, FullPrecisionSettings, Recorder},
    tensor::{
        backend::{AutodiffBackend, Backend},
        Device, Float, Int, Tensor,
    },
    LearningRate,
};
use burn::tensor::cast::ToElement;
use ic_cdk::update;

// 1. 模型定义
#[derive(Module, Debug)]
pub struct LstmModel<B: Backend> {
    lstm1: Lstm<B>,
    dropout1: Dropout,
    lstm2: Lstm<B>,
    dense: Linear<B>,
}

impl<B: Backend> Default for LstmModel<B> {
    fn default() -> Self {
        let device = B::Device::default();
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
        seq.slice([0..batch, (seq_len - 1)..seq_len, 0..features])
            .reshape([batch, features])
    }
}

// 2. 训练专用实现（自动微分）
impl<B: AutodiffBackend> LstmModel<B> {
    // 训练步骤（计算损失和梯度）
    pub fn train() {}

    // 验证步骤（仅计算损失）
    pub fn valid_step(&self, input: Tensor<B, 3>, targets: Tensor<B, 2>) -> Tensor<B, 1> {
        // 禁用Dropout
        let input = self.set_dropout(input, false);
        let prediction = self.forward(input);
        mse_loss(prediction, targets)
    }

    /// 控制Dropout状态
    fn set_dropout(&self, input: Tensor<B, 3>, enabled: bool) -> Tensor<B, 3> {
        //todo 完善dropout
        input
    }
    // 8. 推理预测
    pub fn predict(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
        // 禁用Dropout
        // self.dropout1.disable();

        // LSTM层处理
        let (output_seq, _) = self.lstm1.forward(input.clone(), None);
        let (output_seq, _) = self.lstm2.forward(output_seq, None);

        // 提取最后一个时间步
        let [batch, seq_len, features] = output_seq.dims();
        let final_output = output_seq
            .slice([0..batch, (seq_len - 1)..seq_len, 0..features])
            .reshape([batch, features]);

        // 全连接层预测
        let prediction = self.dense.forward(final_output);

        // 重新启用Dropout
        // self.dropout1.enable();

        prediction
    }

}
// 3. 损失函数
fn mse_loss<B: Backend>(prediction: Tensor<B, 2>, targets: Tensor<B, 2>) -> Tensor<B, 1> {
    // 1. 计算预测值与真实值的差值
    let diff = prediction - targets;

    // 2. 计算平方
    // 使用逐元素乘法（推荐）
    let squared = diff.clone().mul(diff);

    // 3. 沿特征维度（维度1）求平均
    let mean_squared = squared.mean_dim(1);

    // 4. 确保结果是一维张量 (Tensor<B, 1>)
    // 方法 A: 使用 flatten（推荐）
    mean_squared.flatten(1,2)


}
// 4. 数据预处理（时间序列到监督学习格式）
pub struct PriceDataset<B: Backend> {
    sequences: Vec<Tensor<B, 3>>,
    targets: Vec<Tensor<B, 2>>,
    device: B::Device,
}

impl<B: Backend> PriceDataset<B> {
    /// 从原始时间序列创建数据集
    pub fn new(
        data: &[f32],
        seq_len: usize,
        device: B::Device
    ) -> Self {
        if data.len() < seq_len + 1 {
            panic!("数据长度必须大于序列长度+1");
        }

        let mut sequences = Vec::new();
        let mut targets = Vec::new();

        // 使用滑动窗口创建序列
        for i in 0..(data.len() - seq_len) {
            // 提取输入序列
            let sequence = &data[i..i + seq_len];
            let tensor_seq = Tensor::<B, 1>::from_floats(
                sequence,
                &device
            ).reshape([1, seq_len, 1]); // [batch=1, seq_len, features=1]

            // 提取目标值（下一个时间点）
            let target = data[i + seq_len];
            let tensor_target = Tensor::<B, 1>::from_floats(
                [target],
                &device
            ).reshape([1, 1]); // [batch=1, features=1]

            sequences.push(tensor_seq);
            targets.push(tensor_target);
        }

        PriceDataset {
            sequences,
            targets,
            device,
        }
    }

    /// 数据集划分：训练集和测试集
    pub fn train_test_split(
        &self,
        test_ratio: f32
    ) -> (PriceDataset<B>, PriceDataset<B>) {
        let total = self.sequences.len();
        let test_size = (total as f32 * test_ratio) as usize;
        let train_size = total - test_size;

        let train_sequences = self.sequences[..train_size].to_vec();
        let train_targets = self.targets[..train_size].to_vec();

        let test_sequences = self.sequences[train_size..].to_vec();
        let test_targets = self.targets[train_size..].to_vec();

        let train_dataset = PriceDataset {
            sequences: train_sequences,
            targets: train_targets,
            device: self.device.clone(),
        };

        let test_dataset = PriceDataset {
            sequences: test_sequences,
            targets: test_targets,
            device: self.device.clone(),
        };

        (train_dataset, test_dataset)
    }
}
// 实现Dataset trait以便使用数据加载器
impl<B: Backend,I:Send + Sync> Dataset<I> for PriceDataset<B> {
    fn get(&self, index: usize) -> Option<I> {
        todo!()
    }

    fn len(&self) -> usize {
        self.sequences.len()
    }
}
// 5. 训练函数
pub fn train<B: AutodiffBackend>(device: B::Device) {
    

    // 保存训练好的模型
    // save_model(&model, "model.bin".to_string());
}

// 6. 模型保存
fn save_model<B: Backend>(model: &LstmModel<B>, path: String) {
    BinFileRecorder::<FullPrecisionSettings>::new()
        .record(model.clone().into_record(), path.into())
        .expect("Failed to save model");
}

// 7. 模型加载
pub fn load_model<B: Backend>(path: &str, device: &B::Device) -> LstmModel<B> {
    LstmModel::<B>::default().load_record(
        BinFileRecorder::<FullPrecisionSettings>::new()
            .load(path.into(), device)
            .expect("Failed to load model"),
    )
}



//（整合流程）
#[update]
fn pred() {}
