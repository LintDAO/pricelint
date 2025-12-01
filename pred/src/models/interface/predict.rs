use crate::models::interface::train::Train;
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::data::dataset::Dataset;
use burn::prelude::{Backend, Tensor};
use burn::tensor::backend::AutodiffBackend;

pub trait Predict<B, const D1: usize, const D2: usize>: Train<B, D1, D2>
where
    B: AutodiffBackend<Device = NdArrayDevice, FloatElem = f32> + From<Autodiff<NdArray>>,
    Autodiff<NdArray>: From<B>,
{
   
    fn predict(&self, input: Tensor<B, D1>) -> f32 {
        self.forward(input).into_scalar()
    }
    fn predicts(&self, inputs: Vec<Tensor<B, D1>>) -> Vec<f32> {
        let mut predictions = Vec::new();
        for i in 0..inputs.len() {
            let input = inputs.get(i);
            if let Some(input) = input {
                predictions.push(self.predict(input.clone()));
            }
        }
        predictions
    }
}
