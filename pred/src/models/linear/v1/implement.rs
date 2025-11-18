use burn::backend::ndarray::{NdArray, NdArrayDevice};
use burn::config::Config;
use burn::data::dataloader::{DataLoader, Dataset};
use burn::grad_clipping::GradientClippingConfig;
use burn::nn;
use burn::nn::{Initializer, Linear, LinearConfig};
use burn::optim::decay::WeightDecayConfig;
use burn::optim::{AdamConfig, Optimizer};
use burn::prelude::*;
use burn::prelude::{Backend, Device, Module, Tensor};
use burn::tensor::backend::AutodiffBackend;
use ic_cdk::update;
use crate::models::interface::predict::Predict;
use crate::models::interface::train::Train;
use crate::models::linear::v1::domain::{LinearModel, PriceDataset, PriceSample};

impl Dataset<PriceSample> for PriceDataset {
    fn get(&self, index: usize) -> Option<PriceSample> {
        self.samples.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.samples.len()
    }
}


impl<B: Backend> Train<B> for LinearModel<B> {
    fn mse_loss(&self) -> f32 {
        todo!()
    }

    fn grads(&self) -> f32 {
        todo!()
    }
}

impl<B: Backend, I: Send + Sync> Predict<B, I> for LinearModel<B> {
    fn predict(&self, input: I) -> f32 {
        1.0
    }

    fn predicts(&self, inputs: &dyn Dataset<I>) -> Vec<f32> {
        // let mut predictions = Vec::new();
        // for i in 0..inputs.len() {
        //     let input = inputs.get(i);
        //     if let Some(input) = input {
        //         predictions.push(self.predict(input));
        //     }
        // }
        // predictions
        vec![1.0]
    }
}

