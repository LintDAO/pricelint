use crate::ml::model::common::ModelCategories;
use crate::ml::model::common::RNN;
use crate::ml::model::default_model::{PricePredictor, PricePredictorRecord};
use crate::{MODEL_MAP};
use burn::module::Module;
use burn::record::{BinBytesRecorder, FullPrecisionSettings, Recorder};
use burn::tensor::backend::AutodiffBackend;
use crate::ml::api::default_api::SEQUENCE_LENGTH;

// 保存模型
pub fn save_model<B>(model: PricePredictor<B>)
where
    B: AutodiffBackend<Device = burn::backend::ndarray::NdArrayDevice>,
{
    //test data
    let kind = ModelCategories::default();
    match kind {
        ModelCategories::LinearModel() => {}
        ModelCategories::RNN(rnn) => match rnn {
            RNN::LSTM => {
                let recorder = BinBytesRecorder::<FullPrecisionSettings>::new();
                let record = model.into_record();
                let bytes = recorder
                    .record(record, ())
                    .expect("Failed to save LSTM model");
                MODEL_MAP.with(|map| {
                    let mut ref_mut = map.borrow_mut();
                    ref_mut.insert("model".to_string(), bytes);
                })
            }
        },
    }
}
pub fn load_model<B>() -> PricePredictor<B>
where
    B: AutodiffBackend<Device = burn::backend::ndarray::NdArrayDevice>,
{
    let kind = ModelCategories::default();
    match kind {
        ModelCategories::LinearModel() => {
            unreachable!()
        }
        ModelCategories::RNN(rnn) => match rnn {
            RNN::LSTM => {
                let device = burn::backend::ndarray::NdArrayDevice::default();

                let recorder = BinBytesRecorder::<FullPrecisionSettings>::default();

                let module = PricePredictor::new(6, 16, 1, SEQUENCE_LENGTH); // input_size=6, hidden_size=16, output_size=1
                let memory_data=MODEL_MAP.with(|map| {
                    map.borrow_mut().get(&"model".to_string()).unwrap()
                });
                let record=recorder.load::<PricePredictorRecord<B>>(memory_data,&device).unwrap();
                module.load_record(record)
            }
        },
    }
}
