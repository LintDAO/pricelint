use burn::prelude::Backend;

pub trait Train<B: Backend> {
    fn mse_loss(&self) -> f32;
    fn grads (&self) -> f32;
}
