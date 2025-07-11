use burn::optim::{GradientsParams, Optimizer, SimpleOptimizer};
use burn::tensor::cast::ToElement;
use burn::{
    module::Module,
    nn::{Dropout, DropoutConfig, Linear, LinearConfig, Lstm, LstmConfig},
    optim::{ AdamConfig},
    record::{ FullPrecisionSettings, Recorder},
    tensor::{
        backend::{AutodiffBackend, Backend},
        Device, Float, Int, Tensor,
    },
    LearningRate,
};
use crate::common::lifecycle::MODEL_MAP;

use ic_cdk::update;
use serde::de::StdError;
use std::error::Error;
use std::ops::Deref;
use burn::record::{BinBytesRecorder, RecorderError};
use candid::{CandidType, Deserialize};
use serde::Serialize;

// 1. 模型定义
#[derive(Module, Debug)]
pub struct LstmModel<B: Backend> {
    lstm1: Lstm<B>,
    dropout1: Dropout,
    lstm2: Lstm<B>,
    dense: Linear<B>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct RawDataset {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: f32,  //成交量
    price_diff: f32, //价格差
}
pub struct PriceDataset<B: Backend> {
    sequences: Vec<Tensor<B, 3>>,
    targets: Vec<Tensor<B, 2>>,
    device: B::Device,
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
    pub fn train(
        &self,
        device: B::Device,
        data: Vec<f32>,
        seq_len: usize,
        test_ratio: f32,
        num_epochs: usize,
        batch_size: usize,
        learning_rate: LearningRate,
        model_save_path: &str,
    )->Result<(),String> where
        B::FloatElem: ToElement,

    {
        // 1. 准备数据集
        let dataset = PriceDataset::<B>::new(&data, seq_len, device.clone());
        let (train_dataset, test_dataset) = dataset.train_test_split(test_ratio);

        // 转换为张量格式以便批量处理
        let (train_sequences, train_targets) = train_dataset.to_tensors();
        let (test_sequences, test_targets) = test_dataset.to_tensors();

        // 2. 初始化模型和优化器
        let mut model = LstmModel::<B>::default();
        let mut optim = AdamConfig::new().init();

        // 3. 训练循环
        for epoch in 0..num_epochs {
            let mut total_loss = 0.0;
            let mut num_batches = 0;

            // 训练模式
            // model = model.train();

            // 分批训练
            for i in (0..train_sequences.dims()[0]).step_by(batch_size) {
                let end = std::cmp::min(i + batch_size, train_sequences.dims()[0]);

                // 获取当前批次数据
                let batch_sequences = train_sequences.clone().slice([i..end, 0..seq_len, 0..1]);
                let batch_targets = train_targets.clone().slice([i..end, 0..1]);

                // 前向传播
                let prediction = model.forward(batch_sequences.clone());
                let loss = mse_loss(prediction, batch_targets.clone());
                // 反向传播和优化
                let grads = loss.backward();
                let grads_params = GradientsParams::from_grads(grads, &self.clone());
                model = optim.step(learning_rate, model, grads_params);
                // 累加损失
                let loss_data = loss.clone().into_data();
                let loss_slice = loss_data.as_slice::<f32>();
                let loss_value = loss_slice.ok().unwrap().clone();
                total_loss += loss_value[0];
                num_batches += 1;
            }

            // 计算平均损失
            let avg_train_loss = total_loss / num_batches as f32;

            // 验证模式
            // model = model.eval();
            let test_prediction = model.forward(test_sequences.clone());
            let test_loss = mse_loss(test_prediction, test_targets.clone());

            let test_loss_data = test_loss.clone().into_data();
            let test_loss_slice = test_loss_data.as_slice::<f32>();
            let test_loss = test_loss_slice.ok().unwrap();
            let avg_test_loss = test_loss[0];

            // 打印训练进度
            println!(
                "Epoch {}/{} - Train Loss: {:.6}, Test Loss: {:.6}",
                epoch + 1,
                num_epochs,
                avg_train_loss,
                avg_test_loss
            );
        }

        // 4. 保存训练好的模型
        save_model(&model);
        println!("Model saved to {}", model_save_path);
        Ok(())
    }

    // 验证步骤（仅计算损失）
    pub fn valid_step(&self, input: Tensor<B, 3>, targets: Tensor<B, 2>) -> Tensor<B, 1> {
        // 禁用Dropout
        let input = self.set_dropout(input, false);
        let prediction = self.forward(input);
        mse_loss(prediction, targets)
    }

    // 控制Dropout状态
    fn set_dropout(&self, input: Tensor<B, 3>, enabled: bool) -> Tensor<B, 3> {
        // 创建新的Dropout配置
        let dropout_config = DropoutConfig::new(self.dropout1.prob).init();
        if enabled {
            // dropout_config
        }
        // 应用Dropout
        dropout_config.forward(input)
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
    /// 设置模型为评估模式
    pub fn eval(mut self) -> Self {
        // self.dropout1 = self.dropout1.eval;
        self
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
    mean_squared.flatten(0, 1)
}
// 4. 数据预处理（时间序列到监督学习格式）


impl<B: Backend> PriceDataset<B> {
    /// 从原始时间序列创建数据集
    pub fn new(data: &[f32], seq_len: usize, device: B::Device) -> Self {
        if data.len() < seq_len + 1 {
            panic!("数据长度必须大于序列长度+1");
        }

        let mut sequences = Vec::new();
        let mut targets = Vec::new();

        // 使用滑动窗口创建序列
        for i in 0..(data.len() - seq_len) {
            // 提取输入序列
            let sequence = &data[i..i + seq_len];
            let tensor_seq =
                Tensor::<B, 1>::from_floats(sequence, &device).reshape([1, seq_len, 1]); // [batch=1, seq_len, features=1]

            // 提取目标值（下一个时间点）
            let target = data[i + seq_len];
            let tensor_target = Tensor::<B, 1>::from_floats([target], &device).reshape([1, 1]); // [batch=1, features=1]

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
    pub fn train_test_split(&self, test_ratio: f32) -> (PriceDataset<B>, PriceDataset<B>) {
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
    /// 将整个数据集转换为两个张量：序列张量和目标张量
    pub fn to_tensors(&self) -> (Tensor<B, 3>, Tensor<B, 2>) {
        // 将序列列表沿batch维度（0维）拼接
        let sequences = if self.sequences.is_empty() {
            Tensor::zeros([0, 0, 0], &self.device)
        } else {
            Tensor::cat(self.sequences.clone(), 0)
        };

        // 将目标列表沿batch维度（0维）拼接
        let targets = if self.targets.is_empty() {
            Tensor::zeros([0, 0], &self.device)
        } else {
            Tensor::cat(self.targets.clone(), 0)
        };

        (sequences, targets)
    }
}

// 5. 训练函数

// 6. 模型保存
fn save_model<B: Backend>(model: &LstmModel<B>) {
    let recorder = BinBytesRecorder::<FullPrecisionSettings>::new();
    let record = model.clone().into_record();
    let bytes = recorder
        .record(record, ())
        .expect("Failed to save LSTM model");
    MODEL_MAP.with(|map| {
        let mut ref_mut = map.borrow_mut();
        ref_mut.insert("lstm_v1.0.0".to_string(), bytes);
    })
}

// 7. 模型加载
pub fn load_model<B: Backend>(device: &B::Device) -> Result<LstmModel<B>,RecorderError> {

    let recorder = BinBytesRecorder::<FullPrecisionSettings>::default();

    let module = LstmModel::default();
    let memory_data=MODEL_MAP.with(|map| {
        map.borrow_mut().get(&"lstm_v1.0.0".to_string()).unwrap()
    });
    let record=recorder.load::<LstmModelRecord<B>>(memory_data,&device)?;
    Ok(module.load_record(record))
    
}

//（整合流程）
#[update]
fn pred() {}
