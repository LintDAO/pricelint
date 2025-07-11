export const idlFactory = ({ IDL }) => {
  const PriceData = IDL.Record({
    'low' : IDL.Float32,
    'high' : IDL.Float32,
    'close' : IDL.Float32,
    'open' : IDL.Float32,
    'volume' : IDL.Float32,
    'price_diff' : IDL.Float32,
  });
  const WasmFile = IDL.Record({
    'wasm_version' : IDL.Text,
    'wasm_bin' : IDL.Vec(IDL.Nat8),
    'wasm_name' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : WasmFile, 'Err' : IDL.Text });
  const User = IDL.Record({
    'id' : IDL.Text,
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'create_time' : IDL.Nat64,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const State = IDL.Record({
    'bias' : IDL.Opt(IDL.Vec(IDL.Float32)),
    'max_values' : IDL.Vec(IDL.Float32),
    'weights' : IDL.Opt(IDL.Vec(IDL.Float32)),
    'prices' : IDL.Vec(PriceData),
    'min_values' : IDL.Vec(IDL.Float32),
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Vec(WasmFile), 'Err' : IDL.Text });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Pred = IDL.Record({
    'up' : IDL.Float64,
    'staked' : IDL.Float64,
    'trend' : IDL.Text,
    'down' : IDL.Float64,
  });
  const Predictor = IDL.Record({
    'id' : IDL.Text,
    'trend' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'pred' : Pred,
    'stake' : IDL.Tuple(IDL.Float64, IDL.Float64),
    'create_time' : IDL.Nat64,
    'price' : IDL.Float64,
  });
  const PredictorView = IDL.Record({
    'id' : IDL.Text,
    'now' : IDL.Opt(Predictor),
    'next' : IDL.Opt(Predictor),
    'last_1' : IDL.Opt(Predictor),
    'last_2' : IDL.Opt(Predictor),
    'stake' : IDL.Tuple(IDL.Float64, IDL.Float64),
    'create_time' : IDL.Nat64,
    'accuracy' : IDL.Float64,
  });
  const Result_4 = IDL.Variant({ 'Ok' : PredictorView, 'Err' : IDL.Text });
  const Result_5 = IDL.Variant({ 'Ok' : IDL.Vec(Predictor), 'Err' : IDL.Text });
  const Result_6 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat8), 'Err' : IDL.Text });
  const Result_7 = IDL.Variant({ 'Ok' : User, 'Err' : IDL.Text });
  return IDL.Service({
    'add_price' : IDL.Func([PriceData], [], []),
    'delete_wasm' : IDL.Func([IDL.Text, IDL.Text], [Result], []),
    'find_user_lists' : IDL.Func([], [IDL.Vec(User)], ['query']),
    'get_canister_info' : IDL.Func([], [Result_1], []),
    'get_state' : IDL.Func([], [State], ['query']),
    'get_wasm_lists' : IDL.Func([], [Result_2], ['query']),
    'get_wasm_vec' : IDL.Func([IDL.Text, IDL.Text], [Result_3], ['query']),
    'pred' : IDL.Func([], [Result_4], ['query']),
    'predict' : IDL.Func([], [IDL.Float32], ['query']),
    'refill_random_buffer' : IDL.Func([IDL.Nat32], [], []),
    'show_predictions' : IDL.Func([], [Result_5], ['query']),
    'store_wasm' : IDL.Func([IDL.Text], [Result_6], ['query']),
    'train' : IDL.Func([IDL.Nat64], [], []),
    'upload_json_file' : IDL.Func([IDL.Vec(IDL.Nat8)], [], []),
    'upload_wasm' : IDL.Func([IDL.Text, IDL.Text], [Result_6], ['query']),
    'user_login' : IDL.Func([], [Result_7], ['query']),
    'user_register' : IDL.Func([], [Result_7], []),
  });
};
export const init = ({ IDL }) => { return []; };
