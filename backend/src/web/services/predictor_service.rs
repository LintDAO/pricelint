use crate::web::models::context::Context;
use proc_macro::{generate_service_impl, generate_service_trait};
use crate::{map_get, map_insert, PREDICTOR_CONTEXT, Memory, USER_CONTEXT};
use crate::web::models::predictor_model::Predictor;
use std::cell::RefCell;
use std::thread::LocalKey;
use candid::Principal;
use lazy_static::lazy_static;

generate_service_trait!(Predictor);
generate_service_impl!(Predictor,PREDICTOR_CONTEXT);


trait ExtendPredictorService:PredictorService{

    //get coins from other platform
   fn get_coins_prices();

   fn predictor_config();

    // call predictor results
    fn predictor_results();
}
impl ExtendPredictorService for Predictor{

    fn get_coins_prices() {
        todo!()
    }

    fn predictor_config() {
        todo!()
    }

    fn predictor_results() {
        todo!()
    }
}