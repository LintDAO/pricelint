import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface PriceData {
  'low' : number,
  'high' : number,
  'close' : number,
  'open' : number,
  'volume' : number,
  'price_diff' : number,
}
export interface State {
  'bias' : [] | [Array<number>],
  'max_values' : Array<number>,
  'weights' : [] | [Array<number>],
  'prices' : Array<PriceData>,
  'min_values' : Array<number>,
}
export interface _SERVICE {
  'add_price' : ActorMethod<[PriceData], undefined>,
  'get_state' : ActorMethod<[], State>,
  'predict' : ActorMethod<[], number>,
  'train' : ActorMethod<[bigint], undefined>,
  'upload_json_file' : ActorMethod<[Uint8Array | number[]], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
