use crate::web::models::prediction_model::{Prediction, PredictionHistory};
use crate::web::models::record::{Record, RecordKey};
use crate::{EXCHANGE_RATE, PREDICTION, RECORD, STAKE, STAKING_RECORD};
use ic_cdk::api::time;
use std::collections::{BTreeMap, HashMap};
use std::time::Duration;

macro_rules! secs_to_nanos {
    ($sec:ident) => {
        let $sec = Duration::from_secs($sec).as_nanos() as u64;
    };
}

pub trait ExtendPredictorService {
    //get coins from other platform

    fn predictor_config();

    fn get_accuracy(begin_time: u64, end_time: u64) -> BTreeMap<String, u64>;

    fn get_accuracy_record(day: u16) -> BTreeMap<String, u64>;

    fn get_prediction_aggregation(duration: u16) -> BTreeMap<String, PredictionHistory>;

    fn get_last_two_prediction() -> (
        BTreeMap<String, PredictionHistory>,
        BTreeMap<String, PredictionHistory>,
    );

    fn get_total_stake() -> std::collections::BTreeMap<String, u64>;

    fn get_total_stake_util_yesterday() -> BTreeMap<String, u64>;

    fn get_real_stake_24hours() -> BTreeMap<String, u64>;

    fn get_real_stake_util_yesterday() -> BTreeMap<String, u64>;

    fn get_stake_growth_rate() -> BTreeMap<String, f64>;
}
impl ExtendPredictorService for Prediction {
    fn predictor_config() {
        todo!()
    }

    //获取指定时间的准确率
    fn get_accuracy(begin_time: u64, end_time: u64) -> BTreeMap<String, u64> {
        PREDICTION.with_borrow_mut(|rc| {
            rc.iter()
                .filter(|(k, _)| k.1 >= begin_time && k.1 <= end_time)
                .fold(HashMap::new(), |mut acc, (key, value)| {
                    acc.entry(key.2.clone()).or_insert(Vec::new()).push(value);
                    acc
                })
                .iter()
                .map(|(k, v)| {
                    //如果预测和实际一致则认为正确否则不正确 未预测的不纳入统计
                    let total_count = v.len();
                    let true_count = v
                        .iter()
                        .filter(|&a| a.clone().trend.unwrap().eq(&a.clone().pred.trend))
                        .count();
                    (
                        k.clone(),
                        ((true_count as f64 / total_count as f64) * 10f64.powi(8)) as u64,
                    )
                })
                .collect()
        })
    }

    //获取预测准确率记录
    fn get_accuracy_record(duration: u16) -> BTreeMap<String, u64> {
        RECORD.with_borrow_mut(|rc| {
            let mut acc = rc
                .iter()
                .filter(|(k, v)| matches!(k, RecordKey::PredictionAccuracy(_, _,day) if *day == duration))
                .map(|(k, v)| {
                    let (token_name, time) =
                        if let RecordKey::PredictionAccuracy(token_name, time, day) = k {
                            (token_name, time)
                        } else {
                            unreachable!("RecordKey::PredictionAccuracy is not matched")
                        };
                    let accuracy = if let Record::PredictionAccuracy(accuracy) = v {
                        accuracy
                    } else {
                        unreachable!("Record::PredictionAccuracy is not matched")
                    };
                    (token_name, time, accuracy)
                })
                .collect::<Vec<_>>();
            acc.sort_by(|(_, a, _), (_, b, _)| b.cmp(a));
            //按照时间降序排列

            //fold是有序的 所以按次序取最新就行
            acc.into_iter().fold(
                BTreeMap::new(),
                |(mut latest), (token_name, time, acc)| {
                    if !latest.contains_key(&token_name) {
                        latest.insert(token_name.clone(), acc.clone());
                    }
                    latest
                },
            )
        })
    }

    //获取实时的的预测聚合
    fn get_prediction_aggregation(duration: u16) -> BTreeMap<String, PredictionHistory> {
        PREDICTION.with_borrow_mut(|rc| {
            let now = Duration::from_nanos(time()).as_secs();
            let begein_time = now - (duration * 60u16) as u64;
            secs_to_nanos!(begein_time);
            secs_to_nanos!(now);
            let filter_data = rc
                .iter()
                .filter(|(k, v)| k.1 <= now && k.1 >= begein_time)
                .collect::<Vec<_>>();
            filter_data
                .iter()
                .fold(HashMap::new(), |mut acc, (key, value)| {
                    acc.entry(key.2.clone()).or_insert(Vec::new()).push(value);
                    acc
                })
                .iter()
                .map(|(k, v)| {
                    let pred_up = v.iter().map(|(&v)| v.pred.up).sum::<u64>();
                    let pred_down = v.iter().map(|(&v)| v.pred.down).sum::<u64>();
                    let stake = v.iter().map(|(&v)| v.pred.staked).sum::<u64>();
                    //todo ge't from xrc
                    let realtime_price = 1;
                    let last_realtime_price = EXCHANGE_RATE.with_borrow(|r| {
                        //缩小范围 只取最近2小时的记录 避免性能爆炸
                        let two_hour_ago = now - Duration::from_secs(60 * 60 * 2).as_nanos() as u64;
                        r.iter()
                            .filter(|(_, a)| a.time >= two_hour_ago && a.symbol.eq(k))
                            .max_by(|(_, a), (_, b)| a.time.cmp(&b.time))
                            .map(|(_, a)| a.exchange_rate)
                    });
                    let real_trend = if realtime_price >= last_realtime_price.unwrap() {
                        String::from("up")
                    } else {
                        String::from("down")
                    };
                    //这里强制规定 等于也为涨 涨幅0%
                    let pred_trend = if pred_up >= pred_down {
                        String::from("up")
                    } else {
                        String::from("down")
                    };

                    let history = PredictionHistory {
                        token_name: k.clone(),
                        price: realtime_price,
                        trend: real_trend, //实际比较
                        pred: (stake, pred_up, pred_down, pred_trend),
                        time: time(),
                    };
                    (k.clone(), history)
                })
                .collect::<BTreeMap<String, PredictionHistory>>()
        })
    }

    // 获取最近的两次预测
    // 无论是5m还是1h
    fn get_last_two_prediction() -> (
        BTreeMap<String, PredictionHistory>,
        BTreeMap<String, PredictionHistory>,
    ) {
        RECORD.with_borrow_mut(|r| {
            let mut histories = r
                .iter()
                .filter(|(k, v)| matches!(k, RecordKey::PredictionHistory(_, _)))
                .map(|(k, v)| {
                    let (token_name, time) =
                        if let RecordKey::PredictionHistory(token_name, time) = k {
                            (token_name, time)
                        } else {
                            unreachable!("RecordKey::PredictionHistory is not matched")
                        };
                    let history = if let Record::PredictionHistory(history) = v {
                        history
                    } else {
                        unreachable!("Record::PredictionHistory is not matched")
                    };
                    (token_name, time, history)
                })
                .collect::<Vec<_>>();
            histories.sort_by(|(_, a, _), (_, b, _)| b.cmp(a));
            //按照时间降序排列

            //fold是有序的 所以按次序取最新就行
            histories.into_iter().fold(
                (BTreeMap::new(), BTreeMap::new()),
                |(mut latest, mut previous), (token_name, time, history)| {
                    if !latest.contains_key(&token_name) {
                        latest.insert(token_name.clone(), history.clone());
                    } else if !previous.contains_key(&token_name) {
                        previous.insert(token_name.clone(), history.clone());
                    } else {
                    }
                    (latest, previous)
                },
            )
        })
    }

    //获取当前所有用户质押的总金额(OK)
    fn get_total_stake() -> BTreeMap<String, u64> {
        STAKE.with_borrow_mut(|r| {
            r.iter()
                .fold(BTreeMap::new(), |mut acc, (_k, v)| {
                    if let Ok(stake) = u64::try_from(&v.token_balance.0) {
                        *acc.entry(v.stake_detail.token_name.clone()).or_insert(0) += stake;
                    }
                    acc
                })
                .into_iter()
                .collect()
        })
    }
    ////24小时内质押的总共的金额  ,从昨天24：00开始到现在的时间 (OK)
    fn get_total_stake_util_yesterday() -> BTreeMap<String, u64> {
        RECORD.with_borrow_mut(|rc| {
        let now = Duration::from_nanos(time()).as_secs();
        let one_day_secs = 24 * 60 * 60;
        let yesterday_end = now - (now % one_day_secs);
        //因为定时任务的延迟  避免出现49:59和00:01这种情况 所以至少预留两小时
        let yesterday_filter_start = now - (now % one_day_secs) - 60 * 60 * 2;
        secs_to_nanos!(yesterday_end);
        secs_to_nanos!(yesterday_filter_start);
        rc.iter()
            .filter(|(k, v)| {
                matches!(k, RecordKey::StakeAmount(_, time) if *time <= yesterday_end && *time >= yesterday_filter_start)
            })
            .fold(BTreeMap::new(), |mut acc, (k, v)| {
                if let (RecordKey::StakeAmount(token_name, time), Record::StakeAmount(amount)) =
                    (k, v)
                {
                    // 检查是否已经存在该token的记录
                    match acc.get(&token_name) {
                        Some((existing_time, _)) => {
                            // 如果当前记录的时间更大，则更新
                            if time > *existing_time {
                                acc.insert(token_name.clone(), (time, amount));
                            }
                        }
                        None => {
                            // 如果还没有该token的记录，直接插入
                            acc.insert(token_name.clone(), (time, amount));
                        }
                    }
                }
                acc
            })
            .into_iter()
            .map(|(token_name, (time, amount))| (token_name, amount))
            .collect()
    })
    }

    //24小时内质押的实际金额  ,从昨天24：00开始到现在的时间 (OK)
    fn get_real_stake_24hours() -> BTreeMap<String, u64> {
        STAKING_RECORD.with_borrow_mut(|rc| {
            let now = Duration::from_nanos(time()).as_secs();
            let seconds_per_day = 24 * 60 * 60;
            let yesterday_start = (now / seconds_per_day - 1) * seconds_per_day;
            let yesterday_end = yesterday_start + seconds_per_day - 1;
            secs_to_nanos!(now);
            secs_to_nanos!(yesterday_end);
            let total_stake_24hours = rc
                .iter()
                .filter(|(_, v)| {
                    v.cost.is_none()
                        && v.reward.is_none()
                        && yesterday_end <= v.stake_time
                        && v.stake_time <= now
                })
                .fold(BTreeMap::new(), |mut acc, (_k, v)| {
                    *acc.entry(v.token_name).or_insert(0) += v.amount;
                    acc
                });
            total_stake_24hours
        })
    }

    //获取到昨天24：00的质押实际金额(OK)
    fn get_real_stake_util_yesterday() -> BTreeMap<String, u64> {
        let yesterday_staking_token = STAKING_RECORD.with_borrow_mut(|rc| {
            let now = Duration::from_nanos(time()).as_secs();
            let seconds_per_day = 24 * 60 * 60;
            let yesterday_start = (now / seconds_per_day - 1) * seconds_per_day;
            let yesterday_end = yesterday_start + seconds_per_day - 1;
            secs_to_nanos!(yesterday_start);
            secs_to_nanos!(yesterday_end);
            let until_yesterday_end = rc
                .iter()
                .filter(|(_, v)| {
                    v.cost.is_none()
                        && v.reward.is_none()
                        && yesterday_start <= v.stake_time
                        && v.stake_time <= yesterday_end
                })
                .fold(BTreeMap::new(), |mut acc, (_k, v)| {
                    *acc.entry(v.token_name).or_insert(0) += v.amount;
                    acc
                });
            until_yesterday_end
        });
        yesterday_staking_token
    }
    //质押增长率 24小时计（OK)
    fn get_stake_growth_rate() -> BTreeMap<String, f64> {
        let yesterday_staking_token = Self::get_total_stake_util_yesterday();
        let now_staking_token = Self::get_total_stake();
        let mut stake_growth_rate = BTreeMap::new();
        for (token_name, now_stake) in now_staking_token.iter() {
            let yesterday_stake = yesterday_staking_token.get(token_name).unwrap_or(&0);
            let growth_rate = (*now_stake - *yesterday_stake) as f64 / (*yesterday_stake as f64);
            stake_growth_rate.insert(token_name.clone(), growth_rate);
        }
        stake_growth_rate
    }
}

pub mod autosave {
    use crate::common::utils::xrc;
    use crate::common::utils::xrc::{
        Asset, AssetClass, ExchangeRate, GetExchangeRateRequest, GetExchangeRateResult,
    };
    use crate::web::common::constants::XRC_CANISTER_ID;
    use crate::web::models::exchange_rate::{ExchangeRateRecord, ExchangeRateRecordKey};
    use crate::web::models::prediction_model::Prediction;
    use crate::web::models::record::{Record, RecordKey};
    use crate::web::services::prediction_service::ExtendPredictorService;
    use crate::{EXCHANGE_RATE, RECORD};
    use candid::Principal;
    use ic_cdk::api::time;
    use std::collections::BTreeMap;
    use std::time::Duration;

    //获取当前的汇率 从xrc canister获取
    //OK
    async fn get_exchange_rate(
        base_asset: Asset,
        quote_asset: Asset,
    ) -> Result<ExchangeRate, String> {
        let principal = Principal::from_text(XRC_CANISTER_ID).map_err(|e| e.to_string())?;
        let (ret,) = xrc::get_exchange_rate(
            principal,
            1_000_000_000,
            GetExchangeRateRequest {
                timestamp: None,
                base_asset,
                quote_asset,
            },
        )
        .await
        .map_err(|(r, e)| e.to_string())?;
        match ret {
            GetExchangeRateResult::Ok(v) => Ok(v),
            GetExchangeRateResult::Err(e) => Err(e.to_string()),
        }
    }

    //保存汇率到稳定内存
    //OK
    async fn autosave_exchange_rate() -> Result<(), String> {
        const ICP_SYMBOL: &str = "ICP";
        const BTC_SYMBOL: &str = "BTC";
        const TARGET_SYMBOL: &str = "USDT";
        let icp_to_usdt = get_exchange_rate(
            Asset {
                class: AssetClass::Cryptocurrency,
                symbol: ICP_SYMBOL.to_string(),
            },
            Asset {
                class: AssetClass::FiatCurrency,
                symbol: TARGET_SYMBOL.to_string(),
            },
        )
        .await?;
        let btc_to_usdt = get_exchange_rate(
            Asset {
                class: AssetClass::Cryptocurrency,
                symbol: BTC_SYMBOL.to_string(),
            },
            Asset {
                class: AssetClass::FiatCurrency,
                symbol: TARGET_SYMBOL.to_string(),
            },
        )
        .await?;

        //存储到稳定内存
        EXCHANGE_RATE.with(|rate| {
            let now = time();
            let mut borrowed_rate = rate.borrow_mut();
            borrowed_rate.insert(
                ExchangeRateRecordKey(
                    ICP_SYMBOL.to_string() + TARGET_SYMBOL,
                    icp_to_usdt.clone().timestamp,
                ),
                ExchangeRateRecord {
                    symbol: ICP_SYMBOL.to_string() + TARGET_SYMBOL,
                    xrc_data: Some(icp_to_usdt.clone()),
                    exchange_rate: icp_to_usdt.clone().rate,
                    time: icp_to_usdt.clone().timestamp,
                },
            );
            borrowed_rate.insert(
                ExchangeRateRecordKey(
                    BTC_SYMBOL.to_string() + TARGET_SYMBOL,
                    btc_to_usdt.clone().timestamp,
                ),
                ExchangeRateRecord {
                    symbol: BTC_SYMBOL.to_string() + TARGET_SYMBOL,
                    xrc_data: Some(btc_to_usdt.clone()),
                    exchange_rate: btc_to_usdt.clone().rate,
                    time: btc_to_usdt.clone().timestamp,
                },
            );
        });

        Ok(())
    }
    //定时保存质押总额
    fn autosave_stake_amount() -> Result<(), String> {
        let stake_amount = Prediction::get_total_stake();
        RECORD.with_borrow_mut(|r| {
            stake_amount.iter().for_each(|u| {
                r.insert(
                    RecordKey::StakeAmount(u.0.clone(), time()),
                    Record::StakeAmount(u.1.clone()),
                );
            });
        });
        Ok(())
    }
    fn autosave_predict_accuracy() -> Result<(), String> {
        RECORD.with_borrow_mut(|r| {
            let now = Duration::from_nanos(time()).as_secs();
            let per_second_day = 60 * 60 * 24;
            let today_start = now - now % per_second_day;
            let before_7d = today_start - 7 * per_second_day;
            let before_28d = today_start - 28 * per_second_day;
            secs_to_nanos!(before_7d);
            secs_to_nanos!(before_28d);
            secs_to_nanos!(today_start);
            let accuracy_7d = Prediction::get_accuracy(before_7d, today_start);
            let accuracy_28d = Prediction::get_accuracy(before_28d, today_start);
            accuracy_7d.iter().for_each(|u| {
                r.insert(
                    RecordKey::PredictionAccuracy(u.0.clone(), time(), 7),
                    Record::PredictionAccuracy(u.1.clone()),
                );
            });
            accuracy_28d.iter().for_each(|u| {
                r.insert(
                    RecordKey::PredictionAccuracy(u.0.clone(), time(), 28),
                    Record::PredictionAccuracy(u.1.clone()),
                );
            })
        });
        Ok(())
    }

    fn autosave_prediction_history() -> Result<(), String> {
        RECORD.with_borrow_mut(|r| {
            let aggregation = Prediction::get_prediction_aggregation(5);
            aggregation.iter().for_each(|u| {
                r.insert(
                    RecordKey::PredictionHistory(u.0.clone(), time()),
                    Record::PredictionHistory(u.1.clone()),
                );
            });
        });
        Ok(())
    }
}
