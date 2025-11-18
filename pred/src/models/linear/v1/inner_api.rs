use burn::backend::NdArray;
use burn::nn;
use ic_cdk::update;
use crate::models::interface::predict::Predict;
use crate::models::linear::v1::domain::{LinearModel, PriceDataset, PriceSample};

#[update]
pub fn t() {
    let model = LinearModel::<NdArray>::new();
    let x = PriceSample::default();
    ic_cdk::println!("model: {:?} {:?}", model.device(), model.backend());
    let dataset = PriceDataset::new(vec![x.clone()]);
    model.predict(x);
    model.predicts(&dataset);
    ic_cdk::println!("config1: {:?}", nn::LinearConfig::new(3, 1));
}
