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
export type Result = { 'Ok' : PredictorView } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : Array<Predictor> } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : boolean } |
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
export interface _SERVICE {
  'add_price' : ActorMethod<[PriceData], undefined>,
  'create_user' : ActorMethod<[], undefined>,
  'find_user_lists' : ActorMethod<[], Array<User>>,
  'get_state' : ActorMethod<[], State>,
  'pred' : ActorMethod<[], Result>,
  'predict' : ActorMethod<[], number>,
  'refill_random_buffer' : ActorMethod<[number], undefined>,
  'show_predictions' : ActorMethod<[], Result_1>,
  'test' : ActorMethod<[], number>,
  'train' : ActorMethod<[bigint], undefined>,
  'upload_json_file' : ActorMethod<[Uint8Array | number[]], undefined>,
  'user_login' : ActorMethod<[], Result_2>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
