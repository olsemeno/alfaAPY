export const idlFactory = ({ IDL }) => {
  const AddLiquidityResponse = IDL.Record({
    'request_id' : IDL.Nat64,
    'token_0_amount' : IDL.Nat,
    'token_1_amount' : IDL.Nat,
  });
  const InternalErrorKind = IDL.Variant({
    'AccessDenied' : IDL.Null,
    'Infrastructure' : IDL.Null,
    'NotFound' : IDL.Null,
    'Timeout' : IDL.Null,
    'Unknown' : IDL.Null,
    'BusinessLogic' : IDL.Null,
    'ExternalService' : IDL.Null,
    'Validation' : IDL.Null,
  });
  const InternalError = IDL.Record({
    'context' : IDL.Text,
    'code' : IDL.Nat32,
    'kind' : InternalErrorKind,
    'extra' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
    'message' : IDL.Text,
  });
  const ResponseError = IDL.Record({
    'source' : IDL.Opt(InternalError),
    'code' : IDL.Nat32,
    'kind' : InternalErrorKind,
    'message' : IDL.Text,
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const Result = IDL.Variant({
    'Ok' : AddLiquidityResponse,
    'Err' : ResponseError,
  });
  const ExchangeId = IDL.Variant({
    'Sonic' : IDL.Null,
    'KongSwap' : IDL.Null,
    'ICPSwap' : IDL.Null,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : ResponseError });
  const PoolData = IDL.Record({ 'tvl' : IDL.Nat });
  const PositionData = IDL.Record({
    'id' : IDL.Nat,
    'usd_amount0' : IDL.Nat,
    'usd_amount1' : IDL.Nat,
    'amount0' : IDL.Nat,
    'amount1' : IDL.Nat,
  });
  const PoolSnapshotArgs = IDL.Record({
    'pool_data' : IDL.Opt(PoolData),
    'timestamp' : IDL.Nat64,
    'pool_id' : IDL.Text,
    'position_data' : IDL.Opt(PositionData),
  });
  const PoolSnapshot = IDL.Record({
    'id' : IDL.Text,
    'pool_data' : IDL.Opt(PoolData),
    'timestamp' : IDL.Nat64,
    'pool_id' : IDL.Text,
    'position_data' : IDL.Opt(PositionData),
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : ResponseError });
  const Pool = IDL.Record({
    'id' : IDL.Text,
    'provider' : ExchangeId,
    'token0' : IDL.Principal,
    'token1' : IDL.Principal,
    'position_id' : IDL.Opt(IDL.Nat),
  });
  const Result_3 = IDL.Variant({ 'Ok' : Pool, 'Err' : ResponseError });
  const ApyValue = IDL.Record({ 'tokens_apy' : IDL.Nat, 'usd_apy' : IDL.Nat });
  const PoolApy = IDL.Record({
    'month' : ApyValue,
    'week' : ApyValue,
    'year' : ApyValue,
  });
  const PoolMetrics = IDL.Record({ 'apy' : PoolApy, 'tvl' : IDL.Nat });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Vec(Pool), 'Err' : ResponseError });
  const WithdrawFromPoolResponse = IDL.Record({
    'token_0_amount' : IDL.Nat,
    'token_1_amount' : IDL.Nat,
  });
  const Result_5 = IDL.Variant({
    'Ok' : WithdrawFromPoolResponse,
    'Err' : ResponseError,
  });
  return IDL.Service({
    'add_liquidity_to_pool' : IDL.Func(
        [IDL.Principal, IDL.Text, IDL.Nat],
        [Result],
        [],
      ),
    'add_pool' : IDL.Func(
        [IDL.Principal, IDL.Principal, ExchangeId],
        [Result_1],
        [],
      ),
    'add_pool_snapshot' : IDL.Func([PoolSnapshotArgs], [], []),
    'create_pool_snapshot' : IDL.Func([IDL.Text], [PoolSnapshot], []),
    'delete_all_pools_and_snapshots' : IDL.Func([], [IDL.Bool], []),
    'delete_pool' : IDL.Func([IDL.Text], [Result_2], []),
    'delete_pool_snapshot' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'delete_pool_snapshots' : IDL.Func([IDL.Text], [], []),
    'get_pool_by_id' : IDL.Func([IDL.Text], [Result_3], []),
    'get_pool_metrics' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [IDL.Vec(IDL.Tuple(IDL.Text, PoolMetrics))],
        [],
      ),
    'get_pools' : IDL.Func([], [Result_4], []),
    'get_pools_snapshots' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(PoolSnapshot)))],
        [],
      ),
    'remove_liquidity_from_pool' : IDL.Func([IDL.Text], [Result_5], []),
    'set_operator' : IDL.Func([IDL.Principal], [], []),
    'update_pool_ids' : IDL.Func([], [IDL.Bool], []),
  });
};
export const init = ({ IDL }) => {
  const Conf = IDL.Record({ 'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)) });
  return [IDL.Opt(Conf)];
};
