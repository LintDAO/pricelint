use burn::data::dataset::Dataset;
use burn::prelude::Backend;

pub trait Predict<B: Backend,I:Send + Sync> {
    fn predict(&self, input: I) -> f32;
    fn predicts(&self, inputs: &dyn Dataset<I>) -> Vec<f32>;
}