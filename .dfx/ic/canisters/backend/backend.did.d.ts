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
  'up' : number,
  'staked' : number,
  'trend' : string,
  'down' : number,
}
export interface Predictor {
  'id' : string,
  'trend' : [] | [string],
  'pred' : Pred,
  'canister_id' : string,
  'user_id' : string,
  'stake' : [number, number],
  'create_time' : bigint,
  'price' : number,
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
export type Result = { 'Ok' : WasmFile } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : GetBlocksResponse } |
  { 'Err' : string };
export type Result_10 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_11 = { 'Ok' : User } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : string } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : Array<Predictor> } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : GetTransactionsResponse } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : Array<WasmFile> } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : ICRC2AllowanceResponse } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : Predictor } |
  { 'Err' : string };
export type Result_9 = { 'Ok' : PredictorView } |
  { 'Err' : string };
export interface State {
  'bias' : [] | [Array<number>],
  'max_values' : Array<number>,
  'weights' : [] | [Array<number>],
  'prices' : Array<PriceData>,
  'min_values' : Array<number>,
}
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
  'delete_wasm' : ActorMethod<[string, string], Result>,
  'find_user_lists' : ActorMethod<[], Array<User>>,
  'get_blocks' : ActorMethod<[GetBlocksRequest], Result_1>,
  'get_canister_info' : ActorMethod<[], Result_2>,
  'get_latest_version' : ActorMethod<[UpdateType], Result>,
  'get_predictor_vec' : ActorMethod<[], Result_3>,
  'get_principal' : ActorMethod<[], Principal>,
  'get_state' : ActorMethod<[], State>,
  'get_transactions' : ActorMethod<[GetBlocksRequest], Result_4>,
  'get_wasm_bin' : ActorMethod<[string, string], Result>,
  'get_wasm_lists' : ActorMethod<[], Result_5>,
  'icrc1_balance_of' : ActorMethod<[], Result_6>,
  'icrc1_transfer' : ActorMethod<
    [Account, bigint, [] | [Uint8Array | number[]]],
    Result_6
  >,
  'icrc2_allowance' : ActorMethod<[Account], Result_7>,
  'icrc2_approve' : ActorMethod<[bigint], Result_2>,
  'icrc2_transfer_from' : ActorMethod<
    [Account, bigint, [] | [Uint8Array | number[]]],
    Result_6
  >,
  'minting_or_burn' : ActorMethod<[Account, bigint], Result_6>,
  'predict' : ActorMethod<[], number>,
  'push_user_pred' : ActorMethod<[Predictor], Result_8>,
  'refill_random_buffer' : ActorMethod<[number], undefined>,
  'show_predictions' : ActorMethod<[], Result_9>,
  'stake' : ActorMethod<[bigint, bigint], Result_10>,
  'test_1' : ActorMethod<[DurationRange], [bigint, bigint]>,
  'train' : ActorMethod<[bigint], undefined>,
  'unstake' : ActorMethod<[], Result_10>,
  'upload_json_file' : ActorMethod<[Uint8Array | number[]], undefined>,
  'upload_wasm' : ActorMethod<
    [string, string, Uint8Array | number[], UpdateType],
    Result_2
  >,
  'user_login' : ActorMethod<[], Result_11>,
  'user_register' : ActorMethod<[], Result_11>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
