// use burn::module::{ConstantRecord, Param};
// use burn::{
//     module::Module,
//     nn::lstm::{Lstm, LstmConfig},
//     nn::{Linear, LinearConfig},
//     optim::{GradientsParams, Optimizer, SgdConfig},
//     record::Record,
//     tensor::backend::AutodiffBackend,
//     tensor::{backend::Backend, Tensor},
// };
// use ic_cdk;
// 
// #[derive(Module, Debug)]
// pub struct PricePredictor<B: Backend> {
//     pub lstm: Lstm<B>,
//     pub linear: Linear<B>,
//     pub  sequence_length: usize,
// }
// 
// 
// impl<B> PricePredictor<B>
// where
//     B: AutodiffBackend<Device = burn::backend::ndarray::NdArrayDevice>,
// {
//     pub fn new(
//         input_size: usize,
//         hidden_size: usize,
//         output_size: usize,
//         sequence_length: usize,
//     ) -> Self {
//         let lstm = LstmConfig::new(input_size, hidden_size, false) // 单向 LSTM
//             .init(&burn::backend::ndarray::NdArrayDevice::Cpu);
//         let linear = LinearConfig::new(hidden_size, output_size)
//             .with_bias(true)
//             .init(&burn::backend::ndarray::NdArrayDevice::Cpu);
//         Self {
//             lstm,
//             linear,
//             sequence_length,
//         }
//     }
// 
//     pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
//         let [batch, seq, _] = input.dims();
//         let (output, _) = self.lstm.forward(input, None); // 初始状态为 None
//         let last_output = output.slice([0..batch, seq - 1..seq]); // 取最后一个时间步
//         let last_output_dim = last_output.dims()[2]; // 先获取维度
//         self.linear
//             .forward(last_output.reshape([batch, last_output_dim])) // [batch, output_size]
//     }
// 
//     pub fn train(
//         &mut self,
//         inputs: Tensor<B, 3>,
//         targets: Tensor<B, 2>,
//         learning_rate: f32,
//         epochs: usize,
//     ) {
//         let config = SgdConfig::new();
//         let mut optimizer = config.init();
//         for epoch in 0..epochs {
//             let predictions = self.forward(inputs.clone());
//             let loss = predictions.sub(targets.clone()).powf_scalar(2.0).mean();
//             ic_cdk::println!("Epoch {}: Loss = {}", epoch + 1, loss.clone().into_scalar());
//             let grad = loss.backward();
//             let grads = GradientsParams::from_grads(grad, self);
//             *self = optimizer.step(learning_rate as f64, self.clone(), grads);
//         }
//     }
// 
//     pub fn get_weights(&self) -> &Param<Tensor<B, 2>> {
//         &self.linear.weight
//     }
// 
//     pub fn get_bias(&self) -> Option<&Param<Tensor<B, 1>>> {
//         self.linear.bias.as_ref()
//     }
// }