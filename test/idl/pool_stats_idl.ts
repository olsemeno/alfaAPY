export const idlFactory = ({ IDL }) => {
  const AddLiquidityResponse = IDL.Record({
    'token_0_amount' : IDL.Nat,
    'token_1_amount' : IDL.Nat,
    'position_id' : IDL.Nat64,
  });
  const InternalErrorKind = IDL.Variant({
    'AccessDenied' : IDL.Null,
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
  const AddLiquidityResult = IDL.Variant({
    'Ok' : AddLiquidityResponse,
    'Err' : ResponseError,
  });
  const ExchangeId = IDL.Variant({
    'Sonic' : IDL.Null,
    'KongSwap' : IDL.Null,
    'ICPSwap' : IDL.Null,
  });
  const AddPoolResult = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : ResponseError });
  const DeletePoolResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : ResponseError,
  });
  const Pool = IDL.Record({
    'id' : IDL.Text,
    'provider' : ExchangeId,
    'token0' : IDL.Principal,
    'token1' : IDL.Principal,
    'position_id' : IDL.Opt(IDL.Nat64),
  });
  const GetPoolByIdResult = IDL.Variant({ 'Ok' : Pool, 'Err' : ResponseError });
  const ApyValue = IDL.Record({
    'tokens_apy' : IDL.Float64,
    'usd_apy' : IDL.Float64,
  });
  const PoolMetrics = IDL.Record({ 'apy' : ApyValue, 'tvl' : IDL.Nat });
  const GetPoolsResult = IDL.Variant({
    'Ok' : IDL.Vec(Pool),
    'Err' : ResponseError,
  });
  const PoolData = IDL.Record({ 'tvl' : IDL.Nat });
  const PositionData = IDL.Record({
    'id' : IDL.Nat64,
    'usd_amount0' : IDL.Nat,
    'usd_amount1' : IDL.Nat,
    'amount0' : IDL.Nat,
    'amount1' : IDL.Nat,
  });
  const PoolSnapshot = IDL.Record({
    'id' : IDL.Text,
    'pool_data' : IDL.Opt(PoolData),
    'timestamp' : IDL.Nat64,
    'pool_id' : IDL.Text,
    'position_data' : IDL.Opt(PositionData),
  });
  const PoolSnapshotArgs = IDL.Record({
    'pool_data' : IDL.Opt(PoolData),
    'timestamp' : IDL.Nat64,
    'pool_id' : IDL.Text,
    'position_data' : IDL.Opt(PositionData),
  });
  const TestCreatePoolSnapshotResult = IDL.Variant({
    'Ok' : PoolSnapshot,
    'Err' : ResponseError,
  });
  const WithdrawLiquidityResponse = IDL.Record({
    'token_0_amount' : IDL.Nat,
    'token_1_amount' : IDL.Nat,
  });
  const WithdrawLiquidityResult = IDL.Variant({
    'Ok' : WithdrawLiquidityResponse,
    'Err' : ResponseError,
  });
  return IDL.Service({
    'add_liquidity_to_pool' : IDL.Func(
        [IDL.Principal, IDL.Text, IDL.Nat],
        [AddLiquidityResult],
        [],
      ),
    'add_pool' : IDL.Func(
        [IDL.Principal, IDL.Principal, ExchangeId],
        [AddPoolResult],
        [],
      ),
    'delete_pool' : IDL.Func([IDL.Text], [DeletePoolResult], []),
    'get_pool_by_id' : IDL.Func([IDL.Text], [GetPoolByIdResult], []),
    'get_pool_metrics' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [IDL.Vec(IDL.Tuple(IDL.Text, PoolMetrics))],
        [],
      ),
    'get_pools' : IDL.Func([], [GetPoolsResult], []),
    'get_pools_snapshots' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(PoolSnapshot)))],
        [],
      ),
    'set_operator' : IDL.Func([IDL.Principal], [], []),
    'test_add_pool_snapshot' : IDL.Func([PoolSnapshotArgs], [], []),
    'test_create_pool_snapshot' : IDL.Func(
        [IDL.Text],
        [TestCreatePoolSnapshotResult],
        [],
      ),
    'test_delete_all_pools_and_snapshots' : IDL.Func([], [], []),
    'test_delete_pool_snapshot' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'test_delete_pool_snapshots' : IDL.Func([IDL.Text], [], []),
    'test_update_pool_ids' : IDL.Func([], [], []),
    'withdraw_liquidity_from_pool' : IDL.Func(
        [IDL.Text],
        [WithdrawLiquidityResult],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const Conf = IDL.Record({ 'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)) });
  return [IDL.Opt(Conf)];
};
