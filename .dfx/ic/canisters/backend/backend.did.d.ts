import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface Approve {
  'fee' : [] | [bigint],
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'expected_allowance' : [] | [bigint],
  'expires_at' : [] | [bigint],
  'spender' : Account,
}
export interface ArchivedRange {
  'callback' : [Principal, string],
  'start' : bigint,
  'length' : bigint,
}
export interface ArchivedRange_1 {
  'callback' : [Principal, string],
  'start' : bigint,
  'length' : bigint,
}
export interface Asset { 'class' : AssetClass, 'symbol' : string }
export type AssetClass = { 'Cryptocurrency' : null } |
  { 'FiatCurrency' : null };
export interface BlockRange { 'blocks' : Array<Value> }
export interface Burn {
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'spender' : [] | [Account],
}
export type DurationRange = { 'Microseconds' : null } |
  { 'Minutes' : null } |
  { 'Seconds' : null } |
  { 'Days' : null } |
  { 'Milliseconds' : null } |
  { 'Hours' : null } |
  { 'Nanoseconds' : null };
export interface ExchangeRate {
  'metadata' : ExchangeRateMetadata,
  'rate' : bigint,
  'timestamp' : bigint,
  'quote_asset' : Asset,
  'base_asset' : Asset,
}
export interface ExchangeRateMetadata {
  'decimals' : number,
  'forex_timestamp' : [] | [bigint],
  'quote_asset_num_received_rates' : bigint,
  'base_asset_num_received_rates' : bigint,
  'base_asset_num_queried_sources' : bigint,
  'standard_deviation' : bigint,
  'quote_asset_num_queried_sources' : bigint,
}
export interface ExchangeRateRecord {
  'time' : bigint,
  'xrc_data' : [] | [ExchangeRate],
  'exchange_rate' : bigint,
  'symbol' : string,
}
export interface GetBlocksRequest { 'start' : bigint, 'length' : bigint }
export interface GetBlocksResponse {
  'certificate' : [] | [Uint8Array | number[]],
  'first_index' : bigint,
  'blocks' : Array<Value>,
  'chain_length' : bigint,
  'archived_blocks' : Array<ArchivedRange>,
}
export interface GetTransactionsResponse {
  'first_index' : bigint,
  'log_length' : bigint,
  'transactions' : Array<Transaction>,
  'archived_transactions' : Array<ArchivedRange_1>,
}
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
  'streaming_strategy' : [] | [StreamingStrategy],
  'status_code' : number,
}
export interface ICRC2AllowanceResponse {
  'allowance' : bigint,
  'expires_at' : [] | [bigint],
}
export interface Mint {
  'to' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
}
export interface Pred {
  'up' : bigint,
  'staked' : bigint,
  'trend' : string,
  'down' : bigint,
}
export interface Prediction {
  'id' : string,
  'trend' : [] | [string],
  'pred' : Pred,
  'canister_id' : string,
  'user_id' : string,
  'stake' : [number, number],
  'create_time' : bigint,
  'price' : number,
  'token_name' : string,
}
export interface PredictorResult {
  'trend' : [] | [string],
  'pred' : Pred,
  'price' : [] | [number],
}
export interface PredictorView {
  'id' : string,
  'now' : [] | [PredictorResult],
  'next' : [] | [PredictorResult],
  'last_1' : [] | [PredictorResult],
  'last_2' : [] | [PredictorResult],
  'stake' : [number, number],
  'create_time' : bigint,
  'token_name' : string,
  'accuracy' : number,
}
export interface PriceData {
  'low' : number,
  'high' : number,
  'close' : number,
  'open' : number,
  'volume' : number,
  'price_diff' : number,
}
export interface Recycle { 'time' : bigint }
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : WasmFile } |
  { 'Err' : string };
export type Result_10 = { 'Ok' : Prediction } |
  { 'Err' : string };
export type Result_11 = { 'Ok' : Array<PredictorView> } |
  { 'Err' : string };
export type Result_12 = { 'Ok' : StakeRecord } |
  { 'Err' : string };
export type Result_13 = { 'Ok' : User } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : GetBlocksResponse } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : string } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : Array<[[string, string], Stake]> } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : Array<Prediction> } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : GetTransactionsResponse } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : Array<WasmFile> } |
  { 'Err' : string };
export type Result_9 = { 'Ok' : ICRC2AllowanceResponse } |
  { 'Err' : string };
export interface Reward { 'time' : bigint, 'reward_amount' : bigint }
export interface Stake {
  'id' : string,
  'lock_period_days' : bigint,
  'unlock_time' : bigint,
  'stake_detail' : StakeDetail,
  'last_op_time' : bigint,
  'account' : Account,
  'token_balance' : bigint,
}
export interface StakeDetail {
  'user_principal' : string,
  'canister_principal' : string,
  'staking_percentage' : number,
  'token_name' : string,
}
export interface StakeRecord {
  'reward' : [] | [Reward],
  'is_staking' : boolean,
  'cost' : [] | [Recycle],
  'stake_time' : bigint,
  'account' : Account,
  'amount' : bigint,
  'token_name' : string,
}
export interface State {
  'bias' : [] | [Array<number>],
  'max_values' : Array<number>,
  'weights' : [] | [Array<number>],
  'prices' : Array<PriceData>,
  'min_values' : Array<number>,
}
export type StreamingStrategy = {
    'Callback' : { 'token' : Uint8Array | number[], 'callback' : Principal }
  };
export interface Transaction {
  'burn' : [] | [Burn],
  'kind' : string,
  'mint' : [] | [Mint],
  'approve' : [] | [Approve],
  'timestamp' : bigint,
  'transfer' : [] | [Transfer],
}
export interface TransactionRange { 'transactions' : Array<Transaction> }
export interface Transfer {
  'to' : Account,
  'fee' : [] | [bigint],
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'spender' : [] | [Account],
}
export type UpdateType = { 'FunctionUpdate' : null } |
  { 'ModelUpdate' : null };
export interface User {
  'id' : string,
  'owner' : Principal,
  'name' : string,
  'create_time' : bigint,
}
export type Value = { 'Int' : bigint } |
  { 'Map' : Array<[string, Value]> } |
  { 'Nat' : bigint } |
  { 'Nat64' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Vec };
export type Vec = Array<
  { 'Int' : bigint } |
    { 'Map' : Array<[string, Value]> } |
    { 'Nat' : bigint } |
    { 'Nat64' : bigint } |
    { 'Blob' : Uint8Array | number[] } |
    { 'Text' : string } |
    { 'Array' : Vec }
>;
export interface WasmFile {
  'wasm_version' : string,
  'update_type' : UpdateType,
  'wasm_bin' : [] | [Uint8Array | number[]],
  'upload_time' : bigint,
  'wasm_name' : string,
}
export interface _SERVICE {
  'add_price' : ActorMethod<[PriceData], undefined>,
  'backup_stable_memory' : ActorMethod<[], Result>,
  'count_all_symbols' : ActorMethod<[], bigint>,
  'count_by_symbol' : ActorMethod<[string], bigint>,
  'delete_backup_data' : ActorMethod<[bigint], boolean>,
  'delete_wasm' : ActorMethod<[string, string], Result_1>,
  'dump_stable_memory' : ActorMethod<[[] | [bigint]], HttpResponse>,
  'find_all_symbols' : ActorMethod<
    [],
    Array<[string, Array<ExchangeRateRecord>]>
  >,
  'find_backup_data' : ActorMethod<[bigint], [] | [string]>,
  'find_backup_lists' : ActorMethod<[], Array<[bigint, bigint]>>,
  'find_by_symbol' : ActorMethod<[string], Array<ExchangeRateRecord>>,
  'find_user_lists' : ActorMethod<[], Array<User>>,
  'get_blocks' : ActorMethod<[GetBlocksRequest], Result_2>,
  'get_canister_info' : ActorMethod<[], Result_3>,
  'get_latest_version' : ActorMethod<[UpdateType], Result_1>,
  'get_pcl_balance' : ActorMethod<[], Result_4>,
  'get_pcl_list' : ActorMethod<[], Result_5>,
  'get_pcl_stake_balance' : ActorMethod<[string], Result_4>,
  'get_predictor_vec' : ActorMethod<[], Result_6>,
  'get_principal' : ActorMethod<[], Principal>,
  'get_state' : ActorMethod<[], State>,
  'get_transactions' : ActorMethod<[GetBlocksRequest], Result_7>,
  'get_wasm_bin' : ActorMethod<[string, string], Result_1>,
  'get_wasm_lists' : ActorMethod<[], Result_8>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'icrc1_transfer' : ActorMethod<
    [Account, bigint, [] | [Uint8Array | number[]]],
    Result_4
  >,
  'icrc2_allowance' : ActorMethod<[Account], Result_9>,
  'icrc2_approve' : ActorMethod<[bigint], Result_3>,
  'icrc2_transfer_from' : ActorMethod<
    [Account, bigint, [] | [Uint8Array | number[]]],
    Result_4
  >,
  'import_history_records' : ActorMethod<
    [string, Array<[bigint, number]>],
    Result
  >,
  'list_symbol_kind' : ActorMethod<[], Array<string>>,
  'minting_or_burn' : ActorMethod<[Account, bigint], Result_4>,
  'pcl_stake' : ActorMethod<[string, bigint], Result>,
  'pcl_unstake' : ActorMethod<[string], Result>,
  'predict' : ActorMethod<[], number>,
  'prediction_record' : ActorMethod<[Prediction], Result_10>,
  'refill_random_buffer' : ActorMethod<[number], undefined>,
  'restore_from_file' : ActorMethod<[string], Result>,
  'show_predictions' : ActorMethod<[], Result_11>,
  'stake_init' : ActorMethod<[string, string, bigint], Result>,
  'staking_operation_record' : ActorMethod<[StakeRecord], Result_12>,
  'test_1' : ActorMethod<[DurationRange], [bigint, bigint]>,
  'train' : ActorMethod<[bigint], undefined>,
  'upload_json_file' : ActorMethod<[Uint8Array | number[]], undefined>,
  'upload_wasm' : ActorMethod<
    [string, string, Uint8Array | number[], UpdateType],
    Result_3
  >,
  'user_login' : ActorMethod<[], Result_13>,
  'user_register' : ActorMethod<[], Result_13>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
