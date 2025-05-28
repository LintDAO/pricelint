export const idlFactory = ({ IDL }) => {
  const PriceData = IDL.Record({
    'low' : IDL.Float32,
    'high' : IDL.Float32,
    'close' : IDL.Float32,
    'open' : IDL.Float32,
    'volume' : IDL.Float32,
    'price_diff' : IDL.Float32,
  });
  const State = IDL.Record({
    'bias' : IDL.Opt(IDL.Vec(IDL.Float32)),
    'max_values' : IDL.Vec(IDL.Float32),
    'weights' : IDL.Opt(IDL.Vec(IDL.Float32)),
    'prices' : IDL.Vec(PriceData),
    'min_values' : IDL.Vec(IDL.Float32),
  });
  return IDL.Service({
    'add_price' : IDL.Func([PriceData], [], []),
    'get_state' : IDL.Func([], [State], ['query']),
    'predict' : IDL.Func([], [IDL.Float32], ['query']),
    'train' : IDL.Func([IDL.Nat64], [], []),
    'upload_json_file' : IDL.Func([IDL.Vec(IDL.Nat8)], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
