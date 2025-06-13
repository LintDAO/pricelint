export const idlFactory = ({ IDL }) => {
  const PriceData = IDL.Record({
    'low' : IDL.Float32,
    'high' : IDL.Float32,
    'close' : IDL.Float32,
    'open' : IDL.Float32,
    'volume' : IDL.Float32,
    'price_diff' : IDL.Float32,
  });
  const User = IDL.Record({
    'id' : IDL.Text,
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'create_time' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const State = IDL.Record({
    'bias' : IDL.Opt(IDL.Vec(IDL.Float32)),
    'max_values' : IDL.Vec(IDL.Float32),
    'weights' : IDL.Opt(IDL.Vec(IDL.Float32)),
    'prices' : IDL.Vec(PriceData),
    'min_values' : IDL.Vec(IDL.Float32),
  });
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
  const Result_1 = IDL.Variant({ 'Ok' : PredictorView, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Vec(Predictor), 'Err' : IDL.Text });
  const Result_3 = IDL.Variant({ 'Ok' : User, 'Err' : IDL.Text });
  return IDL.Service({
    'add_price' : IDL.Func([PriceData], [], []),
    'find_user_lists' : IDL.Func([], [IDL.Vec(User)], ['query']),
    'get_canister_info' : IDL.Func([], [Result], []),
    'get_canister_info1' : IDL.Func([], [Result], []),
    'get_state' : IDL.Func([], [State], ['query']),
    'pred' : IDL.Func([], [Result_1], ['query']),
    'predict' : IDL.Func([], [IDL.Float32], ['query']),
    'refill_random_buffer' : IDL.Func([IDL.Nat32], [], []),
    'show_predictions' : IDL.Func([], [Result_2], ['query']),
    'train' : IDL.Func([IDL.Nat64], [], []),
    'upload_json_file' : IDL.Func([IDL.Vec(IDL.Nat8)], [], []),
    'user_login' : IDL.Func([], [Result_3], ['query']),
    'user_register' : IDL.Func([], [Result_3], []),
  });
};
export const init = ({ IDL }) => { return []; };
