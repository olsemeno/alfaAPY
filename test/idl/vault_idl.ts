export const idlFactory = ({ IDL }) => {
  const Conf = IDL.Record({ 'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)) });
  const StrategyDepositArgs = IDL.Record({
    'strategy_id' : IDL.Nat16,
    'ledger' : IDL.Principal,
    'amount' : IDL.Nat,
  });
  const StrategyDepositResponse = IDL.Record({
    'tx_id' : IDL.Nat64,
    'shares' : IDL.Nat,
    'amount' : IDL.Nat,
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
  const ResponseError = IDL.Record({
    'code' : IDL.Nat32,
    'kind' : InternalErrorKind,
    'message' : IDL.Text,
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const StrategyDepositResult = IDL.Variant({
    'Ok' : StrategyDepositResponse,
    'Err' : ResponseError,
  });
  const StrategyWithdrawCompleted = IDL.Record({
    'shares' : IDL.Opt(IDL.Nat),
    'strategy_id' : IDL.Text,
    'amount0' : IDL.Opt(IDL.Nat),
    'pool_id' : IDL.Opt(IDL.Text),
  });
  const StrategyWithdrawStarted = IDL.Record({
    'shares' : IDL.Opt(IDL.Nat),
    'strategy_id' : IDL.Text,
    'pool_id' : IDL.Opt(IDL.Text),
  });
  const InternalError = IDL.Record({
    'context' : IDL.Text,
    'code' : IDL.Nat32,
    'kind' : InternalErrorKind,
    'extra' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
    'message' : IDL.Text,
  });
  const AddLiquidityToPoolFailed = IDL.Record({
    'error' : InternalError,
    'amount0' : IDL.Opt(IDL.Nat),
    'pool_id' : IDL.Text,
  });
  const AddLiquidityToPoolCompleted = IDL.Record({
    'amount0' : IDL.Opt(IDL.Nat),
    'amount1' : IDL.Opt(IDL.Nat),
    'pool_id' : IDL.Text,
  });
  const WithdrawLiquidityFromPoolStarted = IDL.Record({
    'shares' : IDL.Nat,
    'total_shares' : IDL.Nat,
    'pool_id' : IDL.Text,
  });
  const SwapTokenFailed = IDL.Record({
    'token_in' : IDL.Principal,
    'error' : InternalError,
    'amount_in' : IDL.Opt(IDL.Nat),
    'token_out' : IDL.Principal,
    'pool_id' : IDL.Text,
  });
  const AddLiquidityToPoolStarted = IDL.Record({
    'amount0' : IDL.Opt(IDL.Nat),
    'amount1' : IDL.Opt(IDL.Nat),
    'pool_id' : IDL.Text,
  });
  const StrategyDepositStarted = IDL.Record({
    'strategy_id' : IDL.Text,
    'amount0' : IDL.Opt(IDL.Nat),
    'pool_id' : IDL.Opt(IDL.Text),
  });
  const StrategyDepositCompleted = IDL.Record({
    'strategy_id' : IDL.Text,
    'amount0' : IDL.Opt(IDL.Nat),
    'pool_id' : IDL.Opt(IDL.Text),
  });
  const StrategyRebalanceFailed = IDL.Record({
    'new_pool_id' : IDL.Opt(IDL.Text),
    'error' : InternalError,
    'strategy_id' : IDL.Text,
    'previous_pool_id' : IDL.Opt(IDL.Text),
  });
  const SwapTokenCompleted = IDL.Record({
    'token_in' : IDL.Principal,
    'amount_out' : IDL.Opt(IDL.Nat),
    'amount_in' : IDL.Opt(IDL.Nat),
    'token_out' : IDL.Principal,
    'pool_id' : IDL.Text,
  });
  const WithdrawLiquidityFromPoolCompleted = IDL.Record({
    'shares' : IDL.Nat,
    'total_shares' : IDL.Nat,
    'amount_token0' : IDL.Nat,
    'amount_token1' : IDL.Nat,
    'pool_id' : IDL.Text,
  });
  const StrategyRebalanceStarted = IDL.Record({
    'strategy_id' : IDL.Text,
    'previous_pool_id' : IDL.Opt(IDL.Text),
  });
  const SwapTokenStarted = IDL.Record({
    'token_in' : IDL.Principal,
    'amount_in' : IDL.Opt(IDL.Nat),
    'token_out' : IDL.Principal,
    'pool_id' : IDL.Text,
  });
  const StrategyWithdrawFailed = IDL.Record({
    'shares' : IDL.Opt(IDL.Nat),
    'error' : InternalError,
    'strategy_id' : IDL.Text,
    'pool_id' : IDL.Opt(IDL.Text),
  });
  const WithdrawLiquidityFromPoolFailed = IDL.Record({
    'shares' : IDL.Nat,
    'total_shares' : IDL.Nat,
    'error' : InternalError,
    'pool_id' : IDL.Text,
  });
  const StrategyRebalanceCompleted = IDL.Record({
    'new_pool_id' : IDL.Opt(IDL.Text),
    'strategy_id' : IDL.Text,
    'previous_pool_id' : IDL.Opt(IDL.Text),
  });
  const StrategyDepositFailed = IDL.Record({
    'error' : InternalError,
    'strategy_id' : IDL.Text,
    'amount0' : IDL.Opt(IDL.Nat),
    'pool_id' : IDL.Opt(IDL.Text),
  });
  const Event = IDL.Variant({
    'StrategyWithdrawCompleted' : StrategyWithdrawCompleted,
    'StrategyWithdrawStarted' : StrategyWithdrawStarted,
    'AddLiquidityToPoolFailed' : AddLiquidityToPoolFailed,
    'AddLiquidityToPoolCompleted' : AddLiquidityToPoolCompleted,
    'WithdrawLiquidityFromPoolStarted' : WithdrawLiquidityFromPoolStarted,
    'SwapTokenFailed' : SwapTokenFailed,
    'AddLiquidityToPoolStarted' : AddLiquidityToPoolStarted,
    'StrategyDepositStarted' : StrategyDepositStarted,
    'StrategyDepositCompleted' : StrategyDepositCompleted,
    'StrategyRebalanceFailed' : StrategyRebalanceFailed,
    'SwapTokenCompleted' : SwapTokenCompleted,
    'WithdrawLiquidityFromPoolCompleted' : WithdrawLiquidityFromPoolCompleted,
    'StrategyRebalanceStarted' : StrategyRebalanceStarted,
    'SwapTokenStarted' : SwapTokenStarted,
    'StrategyWithdrawFailed' : StrategyWithdrawFailed,
    'WithdrawLiquidityFromPoolFailed' : WithdrawLiquidityFromPoolFailed,
    'StrategyRebalanceCompleted' : StrategyRebalanceCompleted,
    'StrategyDepositFailed' : StrategyDepositFailed,
  });
  const EventRecord = IDL.Record({
    'id' : IDL.Nat64,
    'user' : IDL.Opt(IDL.Principal),
    'event' : Event,
    'timestamp' : IDL.Nat64,
    'correlation_id' : IDL.Text,
  });
  const GetEventRecordsResult = IDL.Variant({
    'Ok' : IDL.Vec(EventRecord),
    'Err' : ResponseError,
  });
  const ExchangeId = IDL.Variant({
    'Sonic' : IDL.Null,
    'KongSwap' : IDL.Null,
    'ICPSwap' : IDL.Null,
  });
  const Pool = IDL.Record({
    'id' : IDL.Text,
    'provider' : ExchangeId,
    'token0' : IDL.Principal,
    'token1' : IDL.Principal,
  });
  const StrategyResponse = IDL.Record({
    'id' : IDL.Nat16,
    'name' : IDL.Text,
    'description' : IDL.Text,
    'total_shares' : IDL.Nat,
    'initial_deposit' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat)),
    'user_shares' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat)),
    'current_pool' : IDL.Opt(Pool),
    'total_balance' : IDL.Nat,
    'pools' : IDL.Vec(Pool),
  });
  const SupportedStandard = IDL.Record({ 'url' : IDL.Text, 'name' : IDL.Text });
  const Icrc28TrustedOriginsResponse = IDL.Record({
    'trusted_origins' : IDL.Vec(IDL.Text),
  });
  const UserStrategyResponse = IDL.Record({
    'strategy_current_pool' : Pool,
    'total_shares' : IDL.Nat,
    'strategy_id' : IDL.Nat16,
    'initial_deposit' : IDL.Nat,
    'user_shares' : IDL.Nat,
    'strategy_name' : IDL.Text,
    'users_count' : IDL.Nat32,
  });
  const StrategyWithdrawArgs = IDL.Record({
    'strategy_id' : IDL.Nat16,
    'ledger' : IDL.Principal,
    'percentage' : IDL.Nat,
  });
  const StrategyWithdrawResponse = IDL.Record({
    'current_shares' : IDL.Nat,
    'amount' : IDL.Nat,
  });
  const StrategyWithdrawResult = IDL.Variant({
    'Ok' : StrategyWithdrawResponse,
    'Err' : ResponseError,
  });
  return IDL.Service({
    'deposit' : IDL.Func([StrategyDepositArgs], [StrategyDepositResult], []),
    'get_config' : IDL.Func([], [Conf], ['query']),
    'get_event_records' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [GetEventRecordsResult],
        [],
      ),
    'get_strategies' : IDL.Func([], [IDL.Vec(StrategyResponse)], ['query']),
    'icrc10_supported_standards' : IDL.Func(
        [],
        [IDL.Vec(SupportedStandard)],
        ['query'],
      ),
    'icrc28_trusted_origins' : IDL.Func([], [Icrc28TrustedOriginsResponse], []),
    'test_icpswap_withdraw' : IDL.Func(
        [IDL.Principal, IDL.Nat, IDL.Nat],
        [IDL.Nat],
        [],
      ),
    'test_reset_strategy' : IDL.Func([IDL.Nat16], [], []),
    'test_update_strategy_stats' : IDL.Func([], [], []),
    'user_strategies' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(UserStrategyResponse)],
        [],
      ),
    'withdraw' : IDL.Func([StrategyWithdrawArgs], [StrategyWithdrawResult], []),
  });
};
export const init = ({ IDL }) => {
  const Conf = IDL.Record({ 'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)) });
  return [IDL.Opt(Conf)];
};
