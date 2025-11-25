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
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
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
  const Result_1 = IDL.Variant({ 'Ok' : WasmFile, 'Err' : IDL.Text });
  const StreamingStrategy = IDL.Variant({
    'Callback' : IDL.Record({
      'token' : IDL.Vec(IDL.Nat8),
      'callback' : IDL.Principal,
    }),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'streaming_strategy' : IDL.Opt(StreamingStrategy),
    'status_code' : IDL.Nat16,
  });
  const ExchangeRateMetadata = IDL.Record({
    'decimals' : IDL.Nat32,
    'forex_timestamp' : IDL.Opt(IDL.Nat64),
    'quote_asset_num_received_rates' : IDL.Nat64,
    'base_asset_num_received_rates' : IDL.Nat64,
    'base_asset_num_queried_sources' : IDL.Nat64,
    'standard_deviation' : IDL.Nat64,
    'quote_asset_num_queried_sources' : IDL.Nat64,
  });
  const AssetClass = IDL.Variant({
    'Cryptocurrency' : IDL.Null,
    'FiatCurrency' : IDL.Null,
  });
  const Asset = IDL.Record({ 'class' : AssetClass, 'symbol' : IDL.Text });
  const ExchangeRate = IDL.Record({
    'metadata' : ExchangeRateMetadata,
    'rate' : IDL.Nat64,
    'timestamp' : IDL.Nat64,
    'quote_asset' : Asset,
    'base_asset' : Asset,
  });
  const ExchangeRateRecord = IDL.Record({
    'time' : IDL.Nat64,
    'xrc_data' : IDL.Opt(ExchangeRate),
    'exchange_rate' : IDL.Nat64,
    'symbol' : IDL.Text,
  });
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
  const Result_2 = IDL.Variant({ 'Ok' : GetBlocksResponse, 'Err' : IDL.Text });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const StakeDetail = IDL.Record({
    'user_principal' : IDL.Text,
    'canister_principal' : IDL.Text,
    'staking_percentage' : IDL.Float64,
    'token_name' : IDL.Text,
  });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const Stake = IDL.Record({
    'id' : IDL.Text,
    'lock_period_days' : IDL.Nat64,
    'unlock_time' : IDL.Nat64,
    'stake_detail' : StakeDetail,
    'last_op_time' : IDL.Nat64,
    'account' : Account,
    'token_balance' : IDL.Nat,
  });
  const Result_4 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Text), Stake)),
    'Err' : IDL.Text,
  });
  const Result_5 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text });
  const Pred = IDL.Record({
    'up' : IDL.Nat64,
    'staked' : IDL.Nat64,
    'trend' : IDL.Text,
    'down' : IDL.Nat64,
  });
  const Prediction = IDL.Record({
    'id' : IDL.Text,
    'trend' : IDL.Opt(IDL.Text),
    'pred' : Pred,
    'canister_id' : IDL.Text,
    'user_id' : IDL.Text,
    'stake' : IDL.Tuple(IDL.Float64, IDL.Float64),
    'create_time' : IDL.Nat64,
    'price' : IDL.Float64,
    'token_name' : IDL.Text,
  });
  const Result_6 = IDL.Variant({
    'Ok' : IDL.Vec(Prediction),
    'Err' : IDL.Text,
  });
  const State = IDL.Record({
    'bias' : IDL.Opt(IDL.Vec(IDL.Float32)),
    'max_values' : IDL.Vec(IDL.Float32),
    'weights' : IDL.Opt(IDL.Vec(IDL.Float32)),
    'prices' : IDL.Vec(PriceData),
    'min_values' : IDL.Vec(IDL.Float32),
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
  const Result_7 = IDL.Variant({
    'Ok' : GetTransactionsResponse,
    'Err' : IDL.Text,
  });
  const Result_8 = IDL.Variant({ 'Ok' : IDL.Vec(WasmFile), 'Err' : IDL.Text });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const Result_9 = IDL.Variant({ 'Ok' : Prediction, 'Err' : IDL.Text });
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
    'token_name' : IDL.Text,
    'accuracy' : IDL.Float64,
  });
  const Result_10 = IDL.Variant({
    'Ok' : IDL.Vec(PredictorView),
    'Err' : IDL.Text,
  });
  const Reward = IDL.Record({
    'time' : IDL.Nat64,
    'reward_amount' : IDL.Nat64,
  });
  const Recycle = IDL.Record({ 'time' : IDL.Nat64 });
  const StakeRecord = IDL.Record({
    'reward' : IDL.Opt(Reward),
    'is_staking' : IDL.Bool,
    'cost' : IDL.Opt(Recycle),
    'stake_time' : IDL.Nat64,
    'account' : Account,
    'amount' : IDL.Nat64,
    'token_name' : IDL.Text,
  });
  const Result_11 = IDL.Variant({ 'Ok' : StakeRecord, 'Err' : IDL.Text });
  const DurationRange = IDL.Variant({
    'Microseconds' : IDL.Null,
    'Minutes' : IDL.Null,
    'Seconds' : IDL.Null,
    'Days' : IDL.Null,
    'Milliseconds' : IDL.Null,
    'Hours' : IDL.Null,
    'Nanoseconds' : IDL.Null,
  });
  const Result_12 = IDL.Variant({ 'Ok' : User, 'Err' : IDL.Text });
  return IDL.Service({
    'add_price' : IDL.Func([PriceData], [], []),
    'backup_stable_memory' : IDL.Func([], [Result], []),
    'count_all_symbols' : IDL.Func([], [IDL.Nat64], ['query']),
    'count_by_symbol' : IDL.Func([IDL.Text], [IDL.Nat64], ['query']),
    'delete_backup_data' : IDL.Func([IDL.Nat64], [IDL.Bool], []),
    'delete_wasm' : IDL.Func([IDL.Text, IDL.Text], [Result_1], []),
    'dump_stable_memory' : IDL.Func(
        [IDL.Opt(IDL.Nat64)],
        [HttpResponse],
        ['query'],
      ),
    'find_all_symbols' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(ExchangeRateRecord)))],
        ['query'],
      ),
    'find_backup_data' : IDL.Func([IDL.Nat64], [IDL.Opt(IDL.Text)], []),
    'find_backup_lists' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))],
        ['query'],
      ),
    'find_by_symbol' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(ExchangeRateRecord)],
        ['query'],
      ),
    'find_user_lists' : IDL.Func([], [IDL.Vec(User)], ['query']),
    'get_blocks' : IDL.Func([GetBlocksRequest], [Result_2], []),
    'get_canister_info' : IDL.Func([], [Result_3], []),
    'get_latest_version' : IDL.Func([UpdateType], [Result_1], ['query']),
    'get_pcl_list' : IDL.Func([], [Result_4], ['query']),
    'get_pcl_stake_balance' : IDL.Func([IDL.Text], [Result_5], ['query']),
    'get_predictor_vec' : IDL.Func([], [Result_6], ['query']),
    'get_principal' : IDL.Func([], [IDL.Principal], ['query']),
    'get_state' : IDL.Func([], [State], ['query']),
    'get_transactions' : IDL.Func([GetBlocksRequest], [Result_7], []),
    'get_wasm_bin' : IDL.Func([IDL.Text, IDL.Text], [Result_1], ['query']),
    'get_wasm_lists' : IDL.Func([], [Result_8], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'import_history_records' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Float64))],
        [Result],
        [],
      ),
    'list_symbol_kind' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'pcl_stake' : IDL.Func([IDL.Text, IDL.Float64], [Result], []),
    'pcl_unstake' : IDL.Func([IDL.Text], [Result], []),
    'predict' : IDL.Func([], [IDL.Float32], ['query']),
    'prediction_record' : IDL.Func([Prediction], [Result_9], []),
    'refill_random_buffer' : IDL.Func([IDL.Nat32], [], []),
    'restore_from_file' : IDL.Func([IDL.Text], [Result], []),
    'show_predictions' : IDL.Func([], [Result_10], ['query']),
    'stake_init' : IDL.Func([IDL.Text, IDL.Text, IDL.Nat64], [Result], []),
    'staking_operation_record' : IDL.Func([StakeRecord], [Result_11], []),
    'test_1' : IDL.Func([DurationRange], [IDL.Nat64, IDL.Nat64], ['query']),
    'train' : IDL.Func([IDL.Nat64], [], []),
    'upload_json_file' : IDL.Func([IDL.Vec(IDL.Nat8)], [], []),
    'upload_wasm' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Vec(IDL.Nat8), UpdateType],
        [Result_3],
        [],
      ),
    'user_login' : IDL.Func([], [Result_12], ['query']),
    'user_register' : IDL.Func([], [Result_12], []),
  });
};
export const init = ({ IDL }) => { return []; };
