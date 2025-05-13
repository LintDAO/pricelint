use crate::web::common::constants::{API_VERSION, BASE_BIANCE_API, BIANCE_KLINES_API};
use crate::web::models::context::Context;
use crate::web::models::predictor_model::Predictor;
use crate::{map_get, map_insert, Memory, PREDICTOR_CONTEXT, USER_CONTEXT};
use candid::Principal;
use lazy_static::lazy_static;
use proc_macro::{generate_service_impl, generate_service_trait};
use std::cell::RefCell;
use std::collections::HashMap;
use std::thread::LocalKey;
use ic_cdk::query;

generate_service_trait!(Predictor);
generate_service_impl!(Predictor, PREDICTOR_CONTEXT);

pub trait ExtendPredictorService: PredictorService {
    //get coins from other platform
    fn get_coins_prices() -> ();

    fn predictor_config();

    // call predictor results
    fn predictor_results();
}
impl ExtendPredictorService for Predictor {
   fn get_coins_prices() ->(){

    }

    fn predictor_config() {
        todo!()
    }

    fn predictor_results() {
        todo!()
    }
}
