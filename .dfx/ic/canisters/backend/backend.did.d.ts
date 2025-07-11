import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Pred {
  'up' : number,
  'staked' : number,
  'trend' : string,
  'down' : number,
}
export interface Predictor {
  'id' : string,
  'trend' : [] | [string],
  'name' : string,
  'pred' : Pred,
  'stake' : [number, number],
  'create_time' : bigint,
  'price' : number,
}
export interface PredictorView {
  'id' : string,
  'now' : [] | [Predictor],
  'next' : [] | [Predictor],
  'last_1' : [] | [Predictor],
  'last_2' : [] | [Predictor],
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
export type Result_1 = { 'Ok' : string } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Array<WasmFile> } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : PredictorView } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : Array<Predictor> } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : Uint8Array | number[] } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : User } |
  { 'Err' : string };
export interface State {
  'bias' : [] | [Array<number>],
  'max_values' : Array<number>,
  'weights' : [] | [Array<number>],
  'prices' : Array<PriceData>,
  'min_values' : Array<number>,
}
export interface User {
  'id' : string,
  'owner' : Principal,
  'name' : string,
  'create_time' : bigint,
}
export interface WasmFile {
  'wasm_version' : string,
  'wasm_bin' : Uint8Array | number[],
  'wasm_name' : string,
}
export interface _SERVICE {
  'add_price' : ActorMethod<[PriceData], undefined>,
  'delete_wasm' : ActorMethod<[string, string], Result>,
  'find_user_lists' : ActorMethod<[], Array<User>>,
  'get_canister_info' : ActorMethod<[], Result_1>,
  'get_state' : ActorMethod<[], State>,
  'get_wasm_lists' : ActorMethod<[], Result_2>,
  'get_wasm_vec' : ActorMethod<[string, string], Result_3>,
  'pred' : ActorMethod<[], Result_4>,
  'predict' : ActorMethod<[], number>,
  'refill_random_buffer' : ActorMethod<[number], undefined>,
  'show_predictions' : ActorMethod<[], Result_5>,
  'store_wasm' : ActorMethod<[string], Result_6>,
  'train' : ActorMethod<[bigint], undefined>,
  'upload_json_file' : ActorMethod<[Uint8Array | number[]], undefined>,
  'upload_wasm' : ActorMethod<[string, string], Result_6>,
  'user_login' : ActorMethod<[], Result_7>,
  'user_register' : ActorMethod<[], Result_7>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
