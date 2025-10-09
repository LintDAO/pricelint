export const idlFactory = ({ IDL }) => {
  const Value = IDL.Rec();
  const Vec = IDL.Rec();
  const PriceData = IDL.Record({
    'low' : IDL.Float32,
    'high' : IDL.Float32,
    'close' : IDL.Float32,
    'open' : IDL.Float32,
    'volume' : IDL.Float32,
    'price_diff' : IDL.Float32,
  });
  const UpdateType = IDL.Variant({
    'FunctionUpdate' : IDL.Null,
    'ModelUpdate' : IDL.Null,
  });
  const WasmFile = IDL.Record({
    'wasm_version' : IDL.Text,
    'update_type' : UpdateType,
    'wasm_bin' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'upload_time' : IDL.Nat64,
    'wasm_name' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : WasmFile, 'Err' : IDL.Text });
  const User = IDL.Record({
    'id' : IDL.Text,
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'create_time' : IDL.Nat64,
  });
  const GetBlocksRequest = IDL.Record({
    'start' : IDL.Nat,
    'length' : IDL.Nat,
  });
  Vec.fill(
    IDL.Vec(
      IDL.Variant({
        'Int' : IDL.Int,
        'Map' : IDL.Vec(IDL.Tuple(IDL.Text, Value)),
        'Nat' : IDL.Nat,
        'Nat64' : IDL.Nat64,
        'Blob' : IDL.Vec(IDL.Nat8),
        'Text' : IDL.Text,
        'Array' : Vec,
      })
    )
  );
  Value.fill(
    IDL.Variant({
      'Int' : IDL.Int,
      'Map' : IDL.Vec(IDL.Tuple(IDL.Text, Value)),
      'Nat' : IDL.Nat,
      'Nat64' : IDL.Nat64,
      'Blob' : IDL.Vec(IDL.Nat8),
      'Text' : IDL.Text,
      'Array' : Vec,
    })
  );
  const BlockRange = IDL.Record({ 'blocks' : IDL.Vec(Value) });
  const ArchivedRange = IDL.Record({
    'callback' : IDL.Func([GetBlocksRequest], [BlockRange], ['query']),
    'start' : IDL.Nat,
    'length' : IDL.Nat,
  });
  const GetBlocksResponse = IDL.Record({
    'certificate' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'first_index' : IDL.Nat,
    'blocks' : IDL.Vec(Value),
    'chain_length' : IDL.Nat64,
    'archived_blocks' : IDL.Vec(ArchivedRange),
  });
  const Result_1 = IDL.Variant({ 'Ok' : GetBlocksResponse, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const Pred = IDL.Record({
    'up' : IDL.Float64,
    'staked' : IDL.Float64,
    'trend' : IDL.Text,
    'down' : IDL.Float64,
  });
  const Predictor = IDL.Record({
    'id' : IDL.Text,
    'trend' : IDL.Opt(IDL.Text),
    'pred' : Pred,
    'canister_id' : IDL.Text,
    'user_id' : IDL.Text,
    'stake' : IDL.Tuple(IDL.Float64, IDL.Float64),
    'create_time' : IDL.Nat64,
    'price' : IDL.Float64,
  });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Vec(Predictor), 'Err' : IDL.Text });
  const State = IDL.Record({
    'bias' : IDL.Opt(IDL.Vec(IDL.Float32)),
    'max_values' : IDL.Vec(IDL.Float32),
    'weights' : IDL.Opt(IDL.Vec(IDL.Float32)),
    'prices' : IDL.Vec(PriceData),
    'min_values' : IDL.Vec(IDL.Float32),
  });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const Burn = IDL.Record({
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
    'spender' : IDL.Opt(Account),
  });
  const Mint = IDL.Record({
    'to' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
  });
  const Approve = IDL.Record({
    'fee' : IDL.Opt(IDL.Nat),
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
    'expected_allowance' : IDL.Opt(IDL.Nat),
    'expires_at' : IDL.Opt(IDL.Nat64),
    'spender' : Account,
  });
  const Transfer = IDL.Record({
    'to' : Account,
    'fee' : IDL.Opt(IDL.Nat),
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
    'spender' : IDL.Opt(Account),
  });
  const Transaction = IDL.Record({
    'burn' : IDL.Opt(Burn),
    'kind' : IDL.Text,
    'mint' : IDL.Opt(Mint),
    'approve' : IDL.Opt(Approve),
    'timestamp' : IDL.Nat64,
    'transfer' : IDL.Opt(Transfer),
  });
  const TransactionRange = IDL.Record({
    'transactions' : IDL.Vec(Transaction),
  });
  const ArchivedRange_1 = IDL.Record({
    'callback' : IDL.Func([GetBlocksRequest], [TransactionRange], ['query']),
    'start' : IDL.Nat,
    'length' : IDL.Nat,
  });
  const GetTransactionsResponse = IDL.Record({
    'first_index' : IDL.Nat,
    'log_length' : IDL.Nat,
    'transactions' : IDL.Vec(Transaction),
    'archived_transactions' : IDL.Vec(ArchivedRange_1),
  });
  const Result_4 = IDL.Variant({
    'Ok' : GetTransactionsResponse,
    'Err' : IDL.Text,
  });
  const Result_5 = IDL.Variant({ 'Ok' : IDL.Vec(WasmFile), 'Err' : IDL.Text });
  const Result_6 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text });
  const ICRC2AllowanceResponse = IDL.Record({
    'allowance' : IDL.Nat,
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const Result_7 = IDL.Variant({
    'Ok' : ICRC2AllowanceResponse,
    'Err' : IDL.Text,
  });
  const Result_8 = IDL.Variant({ 'Ok' : Predictor, 'Err' : IDL.Text });
  const PredictorResult = IDL.Record({
    'trend' : IDL.Opt(IDL.Text),
    'pred' : Pred,
    'price' : IDL.Opt(IDL.Float64),
  });
  const PredictorView = IDL.Record({
    'id' : IDL.Text,
    'now' : IDL.Opt(PredictorResult),
    'next' : IDL.Opt(PredictorResult),
    'last_1' : IDL.Opt(PredictorResult),
    'last_2' : IDL.Opt(PredictorResult),
    'stake' : IDL.Tuple(IDL.Float64, IDL.Float64),
    'create_time' : IDL.Nat64,
    'accuracy' : IDL.Float64,
  });
  const Result_9 = IDL.Variant({ 'Ok' : PredictorView, 'Err' : IDL.Text });
  const Result_10 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const DurationRange = IDL.Variant({
    'Microseconds' : IDL.Null,
    'Minutes' : IDL.Null,
    'Seconds' : IDL.Null,
    'Days' : IDL.Null,
    'Milliseconds' : IDL.Null,
    'Hours' : IDL.Null,
    'Nanoseconds' : IDL.Null,
  });
  const Result_11 = IDL.Variant({ 'Ok' : User, 'Err' : IDL.Text });
  return IDL.Service({
    'add_price' : IDL.Func([PriceData], [], []),
    'delete_wasm' : IDL.Func([IDL.Text, IDL.Text], [Result], []),
    'find_user_lists' : IDL.Func([], [IDL.Vec(User)], ['query']),
    'get_blocks' : IDL.Func([GetBlocksRequest], [Result_1], []),
    'get_canister_info' : IDL.Func([], [Result_2], []),
    'get_latest_version' : IDL.Func([UpdateType], [Result], ['query']),
    'get_predictor_vec' : IDL.Func([], [Result_3], ['query']),
    'get_principal' : IDL.Func([], [IDL.Principal], ['query']),
    'get_state' : IDL.Func([], [State], ['query']),
    'get_transactions' : IDL.Func([GetBlocksRequest], [Result_4], []),
    'get_wasm_bin' : IDL.Func([IDL.Text, IDL.Text], [Result], ['query']),
    'get_wasm_lists' : IDL.Func([], [Result_5], ['query']),
    'icrc1_balance_of' : IDL.Func([], [Result_6], []),
    'icrc1_transfer' : IDL.Func(
        [Account, IDL.Nat, IDL.Opt(IDL.Vec(IDL.Nat8))],
        [Result_6],
        [],
      ),
    'icrc2_allowance' : IDL.Func([Account], [Result_7], []),
    'icrc2_approve' : IDL.Func([IDL.Nat], [Result_2], []),
    'icrc2_transfer_from' : IDL.Func(
        [Account, IDL.Nat, IDL.Opt(IDL.Vec(IDL.Nat8))],
        [Result_6],
        [],
      ),
    'minting_or_burn' : IDL.Func([Account, IDL.Nat], [Result_6], []),
    'predict' : IDL.Func([], [IDL.Float32], ['query']),
    'push_user_pred' : IDL.Func([Predictor], [Result_8], []),
    'refill_random_buffer' : IDL.Func([IDL.Nat32], [], []),
    'show_predictions' : IDL.Func([], [Result_9], ['query']),
    'stake' : IDL.Func([IDL.Nat, IDL.Nat64], [Result_10], []),
    'test_1' : IDL.Func([DurationRange], [IDL.Nat64, IDL.Nat64], ['query']),
    'train' : IDL.Func([IDL.Nat64], [], []),
    'unstake' : IDL.Func([], [Result_10], []),
    'upload_json_file' : IDL.Func([IDL.Vec(IDL.Nat8)], [], []),
    'upload_wasm' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Vec(IDL.Nat8), UpdateType],
        [Result_2],
        [],
      ),
    'user_login' : IDL.Func([], [Result_11], ['query']),
    'user_register' : IDL.Func([], [Result_11], []),
  });
};
export const init = ({ IDL }) => { return []; };
