pub enum ModelCategories {
    LinearModel(),
    RNN(RNN),
}
pub enum RNN {
    LSTM,
}
impl Default for ModelCategories{
    fn default() -> Self {
        ModelCategories::RNN(RNN::LSTM)
    }
}