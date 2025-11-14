use serde::Serialize;
use crate::web::models::prediction_model::PredictionHistory;

#[derive(Serialize, Deserialize, Debug, Clone, CandidType, Ord, PartialOrd, Eq, PartialEq)]
pub enum RecordKey {
    //代币名称 预测时间
    PredictionHistory(String, u64),
    //代币名称 预测时间  统计范围（按天统计）
    PredictionAccuracy(String, u64,u16),
    //代币名称  时间
    StakeAmount(String, u64),
}

#[derive(Serialize, Deserialize, Clone, CandidType, Ord, PartialOrd, Eq, PartialEq)]
pub enum Record {
    //历史预测结果
    PredictionHistory(PredictionHistory),
    //预测准确率的 百分比 *10^8
    PredictionAccuracy(u64),
    //总的质押金额（不是实际参与质押的金额）
    StakeAmount(u64),
}