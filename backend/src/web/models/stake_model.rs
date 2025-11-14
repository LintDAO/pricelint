use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens};
use serde::{Deserialize, Serialize};
use crate::common::utils::xrc::ExchangeRate;
use crate::impl_storable;

#[derive(CandidType, Deserialize, Serialize)]
pub struct ICRC2TransferFromArgs {
    pub amount: NumTokens,
    pub to_account: Account,
}
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ICRC1TransferArgs {
    pub to: Account,
    pub fee: Option<u128>,
    pub memo: Option<Vec<u8>>,
    pub from_subaccount: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: u128,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum TransferError {
    GenericError { message: String, error_code: u64 },
    TemporarilyUnavailable,
    BadBurn { min_burn_amount: u128 },
    Duplicate { duplicate_of: u128 },
    BadFee { expected_fee: u128 },
    CreatedInFuture { ledger_time: u64 },
    TooOld,
    InsufficientFunds { balance: u128 },
}


#[derive(Serialize, Deserialize, CandidType)]
pub struct GetBlocksArgs {
    pub start: BlockIndex,
    pub length: u32,
}
#[derive(Serialize, Deserialize, CandidType)]
pub struct QueryBlocksResponse<Block, ArchivedBlockRange> {
    pub chain_length: u64,
    pub certificate: Option<Vec<u8>>,
    pub blocks: Vec<Block>, // Block 结构体也要定义
    pub first_block_index: u64,
    pub archived_blocks: Vec<ArchivedBlockRange>, // 也要定义
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct ICRC1BalanceOfArgs {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}



#[derive(CandidType, Deserialize, Serialize)]
pub struct AllowanceArgs {
    pub account: Account,
    pub spender: Account,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct ICRC2AllowanceResponse {
    pub allowance: Nat,
    pub expires_at: Option<u64>,
}



#[derive(CandidType, Deserialize, Serialize)]
pub struct ApproveArgs {
    pub fee: Option<Nat>,
    pub memo: Option<Vec<u8>>,
    pub from_subaccount: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: Nat,
    pub expected_allowance: Option<Nat>,
    pub expires_at: Option<u64>,
    pub spender: Account,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct Duplicate {
    pub duplicate_of: Nat,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct BadFee {
    pub expected_fee: Nat,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AllowanceChanged {
    pub current_allowance: Nat,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CreatedInFuture {
    pub ledger_time: u64,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct Expired {
    pub ledger_time: u64,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct InsufficientFunds {
    pub balance: Nat,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct GenericError {
    pub message: String,
    pub error_code: Nat,
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum ApproveError {
    GenericError(GenericError),
    TemporarilyUnavailable,
    Duplicate(Duplicate),
    BadFee(BadFee),
    AllowanceChanged(AllowanceChanged),
    CreatedInFuture(CreatedInFuture),
    TooOld,
    Expired(Expired),
    InsufficientFunds(InsufficientFunds),
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum ApproveResult {
    Ok(Nat),
    Err(ApproveError),
}



#[derive(CandidType, Deserialize, Serialize)]
pub struct SwapArgs {
    pub user:Principal,
    pub amount:Nat,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct SwapResponse {

}


#[derive(CandidType, Deserialize, Serialize)]
pub struct WithdrawArgs {
    pub token: Principal,
    pub to: Account,      // 账户结构体
    pub amount: Nat,
    pub fee: Option<Nat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}




//稳定存储的数据
//本结构体用于stake的一个简单记录和存储 ，实际结果以记账罐数据为准
#[derive(CandidType, Deserialize, Serialize,Clone)]
pub struct Stake{
    pub id:String,
    pub account:Account,
    pub token_balance:Nat, //PCL质押金额
    pub lock_period_days :u64, //质押周期 天
    pub unlock_time :u64,    //解除锁定的时间戳
    pub last_op_time:u64,   //最后操作时间
    pub stake_detail:StakeDetail,
}

#[derive(CandidType, Deserialize, Serialize,Clone)]
pub struct StakeDetail {
    //质押比例
    pub staking_percentage:f64,
    //质押的token的名字
    pub token_name:String,
}


pub  enum  TransactionsKind{
    Transfer,
    Mint,
    Burn,
    Approve
}
//记录stake质押历史的相关信息 不一定需要
pub struct StakeInfo{
    pub id:Nat,
    //操作者
    pub operator: Account,
    //转账类型
    pub kind:TransactionsKind,

    //操作时间
    pub time:u64,
}

//具体质押操作记录
//解质押只能解除没有参与质押的,记录这部分是参与了质押的只能被消耗回收
#[derive(CandidType, Deserialize, Serialize,Clone)]
pub struct StakeRecord{
    //操作账户
    pub account: Account,
    //质押时间
    pub stake_time:u64,
    //质押代币被回收或者消耗
    pub cost:Option<Recycle>,
    //质押的奖励发放
    pub reward:Option<Reward>,
    //操作金额
    pub amount:u64,
    //是否质押状态 
    //不在质押状态就是被回收或者奖励了
    pub is_staking:bool,
    //质押目标token的名字也，就是你使用了我们的代币对什么token进行质押
    pub token_name:String,
}
//回收
#[derive(CandidType, Deserialize, Serialize,Clone)]
pub  struct Recycle{
    //回收代币的时间
    pub time:u64,
}
//奖励
#[derive(CandidType, Deserialize, Serialize,Clone)]
pub struct Reward{
    //奖励发放的时间
    pub time:u64,
    //额外奖励金额=其他未质押中的金额的加权平均，根据质押的金额加权
    //总奖励金额=质押金额+额外奖励金额
    pub reward_amount:u64
}


//通过token代币 用户  质押数据首次产生时间 唯一确认
#[derive(Serialize, Deserialize, Debug, Clone, CandidType, Ord, PartialOrd, Eq, PartialEq)]
pub struct StakeRecordKey(pub String, pub Account, pub u64);


