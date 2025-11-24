//该模块的所有操作都是本canister在操作并不是用户本身，需要用户本身大操作直接前端调用principal
pub mod icrc_api {
    use crate::web::common::constants::ICRC1_LEDGER_CANISTER_ID;
    use crate::web::common::guard::is_admin;
    pub(crate) use crate::web::models::stake_model::{ICRC1BalanceOfArgs, ICRC2AllowanceResponse};
    use crate::web::models::stake_model::{ICRC2TransferFromArgs, SwapArgs, WithdrawArgs};
    use candid::{CandidType, Deserialize, Encode, Nat, Principal};
    use ic_cdk::api::time;
    use ic_cdk::{call, caller, query, update};
    use ic_stable_structures::Storable;
    use icrc_ledger_types::icrc1::account::Account;
    use icrc_ledger_types::icrc1::transfer::{
        BlockIndex, Memo, NumTokens, TransferArg, TransferError,
    };
    use icrc_ledger_types::icrc2::allowance::AllowanceArgs;
    use icrc_ledger_types::icrc2::approve::{ApproveArgs, ApproveError};
    use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
    use std::process::id;

    //铸币
    //使用 铸币账户直接转账也就是canisters的principal进行转账 ，就是铸币
    //
    // #[update(guard = "is_admin")]
    // 后续只能管理员操作 测试时不需要
    //1.当前canisters是mint账户的时候 ,转账方法才是铸币 否则是普通转账
    //2.如果当前canisters不是mint账户 那么则需要dfx或者前端agent直接访问记账罐进行转账也就是铸币
    async fn minting_or_burn(account: Account, amount: Nat) -> Result<Nat, String> {
        let canister_id = Principal::from_text(ICRC1_LEDGER_CANISTER_ID)
            .expect("Could not decode the principal.");

        let args = TransferArg {
            from_subaccount: None, //铸币账户
            to: account,
            amount: Nat::from(amount),
            memo: None,
            fee: None,
            created_at_time: Some(time()),
        };
        call::<(TransferArg,), (Result<Nat, TransferError>,)>(
            canister_id,
            "icrc1_transfer",
            (args,),
        )
        .await
        .map_err(|e| format!("failed to call icrc1_transfer: {:?}", e))?
        .0
        .map_err(|e| format!("minting error {:?}", e))
    }

    //转账 从from账户到to账户  通过canister代转账的 需要先要from账户授权给to账户 icrc2_approve  足够的amount
    pub async fn icrc2_transfer_from(
        to_account: Account,
        amount: Nat,
        transfer_memo: Option<Memo>,
    ) -> Result<BlockIndex, String> {
        let transfer_from_args = TransferFromArgs {
            from: Account::from(ic_cdk::caller()),
            memo: transfer_memo,
            amount: amount,
            spender_subaccount: None,
            fee: None,
            to: to_account,
            created_at_time: Some(time()),
        };

        let canister_id = Principal::from_text(ICRC1_LEDGER_CANISTER_ID)
            .expect("Could not decode the principal.");
        // let base_url = Url::parse(LOCAL_URL).map_err(|err| err.to_string())?;

        call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
            canister_id,
            "icrc2_transfer_from",
            (transfer_from_args,),
        )
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        .0
        .map_err(|e| format!("ledger transfer error {:?}", e))
    }

    // 查询 授权用户account 和被授权用户spender   授权的amount金额
    // //在 query 方法中不能进行 inter-canister call。 后续应该将此方法移植到前端
    pub async fn icrc2_allowance(account: Account) -> Result<ICRC2AllowanceResponse, String> {
        let canister_id = Principal::from_text(ICRC1_LEDGER_CANISTER_ID).unwrap();
        let args = AllowanceArgs {
            account: Account::from(account),
            spender: Account::from(ic_cdk::id()),
        };
        let (allowance,) = call(canister_id, "icrc2_allowance", (args,))
            .await
            .map_err(|(r, s)| s.to_string())?;
        Ok(allowance)
    }

    //检查代币余额
    //在 query 方法中不能进行 inter-canister call。 后续应该将此方法移植到前端
    async fn get_pcl_balance() -> Result<Nat, String> {
        let canister_id = Principal::from_text(ICRC1_LEDGER_CANISTER_ID).unwrap();
        let args = ICRC1BalanceOfArgs {
            owner: caller(),
            subaccount: None,
        };
        let (balance,) = call(canister_id, "icrc1_balance_of", (args,))
            .await
            .map_err(|(r, s)| s.to_string())?;
        Ok(balance)
    }
    //当前罐子直接转账给指定用户

    pub async fn icrc1_transfer(
        to_account: Account,
        amount: Nat,
        transfer_memo: Option<Memo>,
    ) -> Result<Nat, String> {
        let canister_id = Principal::from_text(ICRC1_LEDGER_CANISTER_ID)
            .expect("Could not decode the principal.");

        let args = TransferArg {
            from_subaccount: None,
            to: to_account,
            amount: Nat::from(amount),
            memo: transfer_memo,
            fee: None,
            created_at_time: Some(time()),
        };
        call::<(TransferArg,), (Result<Nat, TransferError>,)>(
            canister_id,
            "icrc1_transfer",
            (args,),
        )
        .await
        .map_err(|e| format!("failed to call icrc1_transfer: {:?}", e))?
        .0
        .map_err(|e| format!("minting error {:?}", e))
    }

    //
    // 用户授权给canisters的可以转账的 amount金额  ,每进行一次转账都要消耗授权额度 amount
    // 铸币账户不能被授权转账
    //需要前端实现agent调用授权 canisters不能代替授权
    pub async fn icrc2_approve(amount: Nat) -> Result<String, String> {
        let args: ApproveArgs = ApproveArgs {
            from_subaccount: None,
            spender: Account::from(ic_cdk::id()),
            amount,
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: Some(time()),
        };
        let canister_id = Principal::from_text(ICRC1_LEDGER_CANISTER_ID).unwrap();
        let approve_response = call::<(ApproveArgs,), (Result<Nat, ApproveError>,)>(
            canister_id,
            "icrc2_approve",
            (args,),
        )
        .await
        .map_err(|e| format!("failed to call icrc2_approve: {:?}", e))?
        .0
        .map_err(|e| format!("icrc2_approve error {:?}", e))?;
        Ok(ic_cdk::id().to_text())
    }
}

pub mod transfer_log {
    use crate::web::common::constants::ICRC1_LEDGER_CANISTER_ID;
    pub(crate) use crate::web::models::stake_model::{GetBlocksArgs, QueryBlocksResponse};
    use candid::MotokoResult::ok;
    use candid::{CandidType, Principal};
    use ic_cdk::api::call::CallResult;
    use ic_cdk::{call, caller, query, update};
    use icrc_ledger_types::icrc1::transfer::BlockIndex;
    use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
    use icrc_ledger_types::icrc3::blocks::{GetBlocksRequest, GetBlocksResponse};
    use icrc_ledger_types::icrc3::transactions::{GetTransactionsRequest, GetTransactionsResponse};
    use serde::{Deserialize, Serialize};

    //从 ledger canister 获取某个区块（block）
    #[update]
    pub async fn get_blocks(args: GetBlocksRequest) -> Result<GetBlocksResponse, String> {
        let canister_id = Principal::from_text(ICRC1_LEDGER_CANISTER_ID).unwrap();
        let (ret,) = call(canister_id, "get_blocks", (args,))
            .await
            .map_err(|(r, s)| s.to_string())?;
        Ok(ret)
    }

    //查询交易记录
    #[update]
    pub async fn get_transactions(
        args: GetTransactionsRequest,
    ) -> Result<GetTransactionsResponse, String> {
        let canister_id = Principal::from_text(ICRC1_LEDGER_CANISTER_ID).unwrap();
        let (resp,) = call::<(GetTransactionsRequest,), (GetTransactionsResponse,)>(
            canister_id,
            "get_transactions",
            (args,),
        )
        .await
        .map_err(|(_, msg)| msg)?;
        Ok(resp)
    }
}

pub mod stake {
    use crate::common::utils::hash::{hash, hash_salt};
    use crate::common::utils::time::DurationRange::{Nanoseconds, Seconds};
    use crate::common::utils::time::NANOS_PER_DAY;
    use crate::web::api::stake_api::icrc_api::{icrc1_transfer, icrc2_transfer_from};
    use crate::web::api::stake_api::transfer_log::get_transactions;
    use crate::web::common::constants::ICRC1_LEDGER_CANISTER_ID;
    use crate::web::common::errors::StakeError;
    use crate::web::common::guard::is_admin;
    use crate::web::common::guard::is_named_user;
    use crate::web::models::stake_model::{Stake, StakeDetail, StakeKey};
    use crate::STAKE;
    use candid::{Nat, Principal};
    use ic_cdk::api::time;
    use ic_cdk::{caller, id, query, update};
    use icrc_ledger_types::icrc1::account::Account;
    use icrc_ledger_types::icrc3::transactions::{GetTransactionsRequest, Transaction, Transfer};
    use std::collections::BTreeMap;

    #[query]
    pub fn get_pcl_stake_balance(canister_id: String) -> Result<Nat, String> {
        STAKE.with_borrow_mut(|rc| {
            let x = rc
                .get(&StakeKey(caller().to_text(), canister_id))
                .ok_or(StakeError::UserOrCanisterIsNotExist.to_string())?;
            Ok(x.token_balance)
        })
    }
    //自用
    #[query(guard = "is_admin")]
    pub fn get_pcl_list() -> Result<BTreeMap<StakeKey, Stake>, String> {
        let map =
            STAKE.with_borrow_mut(|rc| rc.iter().map(|(k, v)| (k, v)).collect::<BTreeMap<_, _>>());
        Ok(map)
    }
    //初始化还有一些默认参数的设置
    #[update]
    pub fn stake_init(
        canister_id: String,
        token_name: String,
        lock_day: u64,
    ) -> Result<(), String> {
        if lock_day < 0 {
            return Err(StakeError::LockDaysIsInvalid.to_string());
        }
        //质押周期设置为0  意思是随时可以解除
        STAKE.with_borrow_mut(|rc| {
            let x = rc.insert(
                StakeKey(caller().to_text(), canister_id.clone()),
                Stake {
                    id: hash_salt(&caller().to_text(), canister_id.to_string()),
                    account: Account::from(caller()),
                    token_balance: Nat::from(0u32),
                    lock_period_days: lock_day,
                    unlock_time: time(),
                    last_op_time: time(),
                    stake_detail: StakeDetail {
                        staking_percentage: 0.0,
                        token_name,
                        user_principal: caller().to_text(),
                        canister_principal: canister_id.clone(),
                    },
                },
            );
            if x.is_some() {
                Err(StakeError::AlreadyInitialized.to_string())
            } else {
                Ok(())
            }
        })
    }
    /// 质押token  累计计算
    /// 目前由backend代操作 实际是以铸币canisters进行操作的
    #[update(guard = "is_named_user")]
    pub async fn pcl_stake(canister_id: String, stake_amount: u64) -> Result<(), String> {
        let stake_amount = Nat::from(stake_amount * 10u64.pow(8));
        let backend_canister_id = ic_cdk::api::id();
        ic_cdk::println!("canister_id：{}", backend_canister_id);
        if stake_amount <= Nat::from(0u32) {
            return Err(StakeError::StakeAmountIsInvalid.to_string());
        }
        //先approve允许转账 ，有额度之后  转移到backend canister
        let resp = icrc2_transfer_from(
            Account::from(backend_canister_id),
            stake_amount.clone(),
            None,
        )
        .await
        .map_err(|e| e.to_string())?;

        let get_transactions_args = GetTransactionsRequest {
            start: resp,
            length: Nat::from(1u32),
        };
        let transactions = get_transactions(get_transactions_args)
            .await
            .map_err(|e| e.to_string())?;
        let vec_transactions = transactions.transactions;

        if !vec_transactions.is_empty() {
            let Transaction { kind, transfer, .. } = &vec_transactions[0];
            if *kind == "transfer" {
                if let Some(transfer_obj) = transfer {
                    let Transfer {
                        amount: transfer_amount,
                        from,
                        to,
                        spender,
                        memo,
                        fee,
                        created_at_time,
                    } = transfer_obj;
                    //转账金额 转账Form  to 三个字段同时对比成功则认为是转账成功 则进行质押流程
                    // 也可以用memo备注信息判断  后续更新再说
                    if *transfer_amount == stake_amount
                        && *to == Account::from(backend_canister_id)
                        && *from == Account::from(caller())
                    {
                        //转账成功
                        STAKE
                            .with(|map| {
                                let stake = map
                                    .borrow_mut()
                                    .get(&StakeKey(caller().to_string(), canister_id.clone()));
                                let now_time = time();
                                //如果匹配是空的说明没进行初始化 先进行初始化
                                match stake {
                                    None => return Err(StakeError::NotInitializedStake.to_string()),
                                    Some(mut some_stake) => {
                                        some_stake.token_balance =  some_stake.token_balance+stake_amount;
                                        some_stake.last_op_time = now_time;
                                        map.borrow_mut().insert(
                                            StakeKey(caller().to_string(), canister_id.clone()),
                                            some_stake,
                                        );
                                        return Ok(());
                                    } //质押之后 应该有个存款map记录质押的存款 后续待定
                                };
                            })
                            .map_err(|e| e.to_string())?;
                        return Ok(());
                    }
                    return Ok(());
                }
            } else {
                return Err(StakeError::IsNotTransferTransaction.to_string());
            };
        }
        Err(StakeError::TransactionIsNotExist.to_string())
    }

    //解质押全部token
    #[update]
    pub async fn pcl_unstake(canister_id: String) -> Result<(), String> {
        let some_stake = STAKE.with(|map| {
            let stake = map
                .borrow()
                .get(&StakeKey(caller().to_string(), canister_id.clone()));
            return Ok(match stake {
                None => {
                    return Err(StakeError::UserHasNotStake.to_string());
                }
                Some(some_stake) => {
                    return Ok(some_stake);
                }
            });
        });

        let mut unstake = some_stake?;
        let now_time = time();
        if now_time < unstake.lock_period_days {
            return Err(StakeError::LockPeriodNotEnd.to_string());
        }
        if unstake.token_balance <= Nat::from(0u32) {
            return Err(StakeError::StakeAmountIsInvalid.to_string());
        }

        //用户从backend canister 提取token到用户principal
        let resp = icrc1_transfer(Account::from(caller()), unstake.clone().token_balance, None)
            .await
            .map_err(|e| e.to_string())?;

        STAKE.with(|map| {
            unstake.token_balance = Nat::from(0u32);
            unstake.last_op_time = now_time;
            unstake.lock_period_days = 0;
            map.borrow_mut()
                .insert(StakeKey(caller().to_string(), canister_id.clone()), unstake);
        });
        //因为是解质押就不需要太多验证了  直接提取到用户钱包
        Ok(())
    }

    //提取到钱包   从定义上来说，比起解除质押少了个等待解锁的步骤
    pub fn withdraw() {
        todo!()
    }

    // sns  后续实现
    pub fn sns() {
        todo!()
    }

    //icp交换PCL 后续实现
    pub fn swap() {
        todo!()
    }
}

pub mod stake_api {
    use crate::web::models::stake_model::Stake;

    //批量结算token
    fn settled_token() {
        //1.获取预测结果
        // Prediction
        //2.根据预测结果 结算用户质押的token
        //3.如果质押的PCL小于固定值则不结算
    }

    //设置默认的质押配置
    fn set_staking_config(staking_config: Stake) {}
}
