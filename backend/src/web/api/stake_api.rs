// 按道理来说这个agent调用应该前端实现 直接用户自己的principal操作 待验证 看是否规定强制canister的principal
//此处的所有方法都是canister代替操作 如果需要用户直接操作 则需要在前端agent实现调用
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
    //当前canisters是mint账户的时候 ,转账方法才是铸币 否则是普通转账
    //如果当前canisters不是mint账户 那么则需要dfx或者前端agent直接访问记账罐进行转账也就是铸币
    #[update]
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
    #[update]
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
    #[update]
    pub async fn icrc2_allowance(
        account: Account,
    ) -> Result<ICRC2AllowanceResponse, String> {
        let canister_id = Principal::from_text(ICRC1_LEDGER_CANISTER_ID).unwrap();
        let args = AllowanceArgs {
            account: Account ::from(account),
            spender: Account::from(ic_cdk::id()),
        };
        let (allowance,) = call(canister_id, "icrc2_allowance", (args,))
            .await
            .map_err(|(r, s)| s.to_string())?;
        Ok(allowance)
    }

    //检查代币余额
    //在 query 方法中不能进行 inter-canister call。 后续应该将此方法移植到前端
    #[update]
    async fn icrc1_balance_of() -> Result<Nat, String> {
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

    #[update]
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
    #[update]
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
    use crate::web::models::stake_model::Stake;
    use crate::STAKE;
    use candid::{Nat, Principal};
    use ic_cdk::api::time;
    use ic_cdk::{caller, id, update};
    use icrc_ledger_types::icrc1::account::Account;
    use icrc_ledger_types::icrc2::transfer_from::TransferFromArgs;
    use icrc_ledger_types::icrc3::transactions::{GetTransactionsRequest, Transaction, Transfer};
    use std::ops::Deref;

    ///质押token  累计计算
    #[update]
    pub async fn stake(stake_amount: Nat, lock_days: u64) -> Result<(), String> {
        if stake_amount <= Nat::from(0u32) {
            return Err(StakeError::StakeAmountIsZero.to_string());
        }
        if lock_days <= 0 {
            return Err(StakeError::LockDaysIsZero.to_string());
        }
        //先approve允许转账 ，有额度之后  转移到backend canister
        let resp = icrc2_transfer_from(Account::from(caller()), stake_amount.clone(), None)
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
            let Transaction {
                kind,
                mint,
                burn,
                transfer,
                approve,
                timestamp,
            } = &vec_transactions[0];
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
                        && *to == Account::from(id())
                        && *from == Account::from(caller())
                    {
                        //转账成功
                        STAKE.with(|map| {
                            let stake = map.borrow_mut().get(&(caller().to_string()));
                            let now_time = time();
                            match stake {
                                None => {
                                    let new_stake = Stake {
                                        id: hash_salt(caller(), time().to_string()),
                                        account: Account::from(caller()),
                                        token_balance: stake_amount, //
                                        lock_period_days: 0,         //质押周期 待定
                                        unlock_time: now_time + lock_days * NANOS_PER_DAY,
                                        last_op_time: now_time,
                                    };
                                    map.borrow_mut().insert(caller().to_string(), new_stake);
                                }
                                Some(mut some_stake) => {
                                    some_stake.token_balance += stake_amount;
                                    some_stake.last_op_time += now_time;
                                    some_stake.lock_period_days =
                                        now_time + lock_days * NANOS_PER_DAY; //质押周期 待定 后续修改具体值

                                    map.borrow_mut().insert(caller().to_string(), some_stake);
                                } //质押之后 应该有个存款map记录质押的存款 后续待定
                            }
                        });
                        return Ok(());
                    }
                }
            }
            return Err(StakeError::IsNotTransferTransaction.to_string());
        }
        Err(StakeError::TransactionIsNotExist.to_string())
    }

    //解质押全部token
    #[update]
    pub async fn unstake() -> Result<(), String> {
        let some_stake = STAKE.with(|map| {
            let stake = map.borrow().get(&(caller().to_string()));
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
            return Err(StakeError::StakeAmountIsZero.to_string());
        }

        //用户从backend canister 提取token到用户principal
        let resp = icrc1_transfer(Account::from(caller()), unstake.clone().token_balance, None)
            .await
            .map_err(|e| e.to_string())?;

        STAKE.with(|map| {
            unstake.token_balance=Nat::from(0u32);
            unstake.last_op_time=now_time;
            unstake.lock_period_days=0;
            map.borrow_mut().insert(caller().to_string(),unstake);
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
