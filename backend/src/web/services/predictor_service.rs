use crate::web::common::constants::{API_VERSION, BASE_BIANCE_API, BIANCE_KLINES_API};
use crate::web::models::context::Context;
use crate::web::models::predictor_model::Predictor;
use crate::{map_get, map_insert, Memory, PREDICTOR_CONTEXT, USER_CONTEXT};
use candid::Principal;
use ic_cdk::caller;
use lazy_static::lazy_static;
use proc_macro::{generate_service_impl, generate_service_trait};
use std::cell::RefCell;
use std::thread::LocalKey;

generate_service_trait!(Predictor);
generate_service_impl!(Predictor, PREDICTOR_CONTEXT);

pub trait ExtendPredictorService: PredictorService {
    //get coins from other platform
    fn get_coins_prices() -> ();

    fn predictor_config();

    // call predictor results
    fn get_predictor_results() -> f32;
    fn get_last_pred(principal:Principal) -> Vec<Predictor>;
    fn get_accuracy(principal: Principal) -> f64;
    fn get_total_stake() -> f64;
}
impl ExtendPredictorService for Predictor {
    fn get_coins_prices() -> () {}

    fn predictor_config() {
        todo!()
    }

    fn get_predictor_results() -> f32 {
        //todo 暂定如此  后续逻辑需修改 或者重写预测的具体过程和数据存储
        // predict()
        1.0
    }

    //获取最后两次数据 数据不足则获取一次
    fn get_last_pred(principal: Principal) -> Vec<Predictor> {
        let mut predictors: Vec<Predictor> = MAP.with(|map| {
            map.borrow()
                .iter()
                .filter(|(_, p)| p.owner.unwrap() == principal)
                .map(|(_, p)| p.context.unwrap())
                .collect()
        });

        predictors.sort_by(|a, b| b.create_time.cmp(&a.create_time));
        predictors.into_iter().take(2).collect() // 取最新的两个
    }

    fn get_accuracy(principal: Principal) -> f64 {
        MAP.with(|map| {
            let mut borrowed_map = map.borrow_mut();

            //预测所有人的,总人数是所有已经预测了的,也就是 predictor.trend!="none"的
            let mut current_user_predictors = borrowed_map
                .iter()
                .filter(|(_, p)| p.clone().context.unwrap().trend.is_some());

            //预测正确的
            let true_count = current_user_predictors
                .by_ref()
                .filter(|(_, v)| {
                    let predictor = v.clone().context.unwrap();
                    //匹配已有实际结果的predictor历史 不匹配尚未预测的和正在预测的
                    if let Some(_) = predictor.trend {
                        //预测结果与实际结果相等
                        if predictor.trend.unwrap() == predictor.pred.trend {
                            return true;
                        }
                    }
                    return false;
                })
                .count();

            let total_count = current_user_predictors.count().clone();
            (true_count / total_count) as f64  //后续可能均要修改成as f64
        })
    }

    fn get_total_stake() -> f64 {
        todo!()
    }

}
