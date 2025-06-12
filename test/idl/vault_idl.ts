export const idlFactory = ({ IDL }) => {
  const Conf = IDL.Record({ 'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)) });
  const StrategyDepositArgs = IDL.Record({
    'strategy_id' : IDL.Nat16,
    'ledger' : IDL.Principal,
    'amount' : IDL.Nat,
  });
  const StrategyDepositResponse = IDL.Record({
    'request_id' : IDL.Nat64,
    'tx_id' : IDL.Nat64,
    'shares' : IDL.Nat,
    'amount' : IDL.Nat,
  });
  const ResponseErrorCode = IDL.Variant({
    'AccessDenied' : IDL.Null,
    'NotFound' : IDL.Null,
    'Timeout' : IDL.Null,
    'Validation' : IDL.Null,
    'InternalError' : IDL.Null,
  });
  const ResponseError = IDL.Record({
    'code' : ResponseErrorCode,
    'message' : IDL.Text,
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const Result = IDL.Variant({
    'Ok' : StrategyDepositResponse,
    'Err' : ResponseError,
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
    'kind' : InternalErrorKind,
    'extra' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
    'message' : IDL.Text,
  });
  const EventLogType = IDL.Variant({
    'ExternalCallCompleted' : IDL.Null,
    'StrategyWithdrawCompleted' : IDL.Null,
    'StrategyWithdrawStarted' : IDL.Null,
    'AddLiquidityToPoolFailed' : IDL.Null,
    'AddLiquidityToPoolCompleted' : IDL.Null,
    'ExternalCallFailed' : IDL.Null,
    'SwapTokenFailed' : IDL.Null,
    'RemoveLiquidityFromPoolStarted' : IDL.Null,
    'AddLiquidityToPoolStarted' : IDL.Null,
    'StrategyDepositStarted' : IDL.Null,
    'StrategyDepositCompleted' : IDL.Null,
    'StrategyRebalanceFailed' : IDL.Null,
    'RemoveLiquidityFromPoolFailed' : IDL.Null,
    'SwapTokenCompleted' : IDL.Null,
    'RemoveLiquidityFromPoolCompleted' : IDL.Null,
    'StrategyRebalanceStarted' : IDL.Null,
    'SwapTokenStarted' : IDL.Null,
    'ExternalCallStarted' : IDL.Null,
    'StrategyWithdrawFailed' : IDL.Null,
    'StrategyRebalanceCompleted' : IDL.Null,
    'StrategyDepositFailed' : IDL.Null,
  });
  const EventLogParams = IDL.Variant({
    'ExternalCallCompleted' : IDL.Record({
      'service' : IDL.Text,
      'result' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
      'method' : IDL.Text,
      'params' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    }),
    'StrategyWithdrawCompleted' : IDL.Record({
      'shares' : IDL.Nat,
      'strategy_id' : IDL.Text,
      'amount0' : IDL.Nat,
      'pool_id' : IDL.Opt(IDL.Text),
    }),
    'StrategyWithdrawStarted' : IDL.Record({
      'shares' : IDL.Nat,
      'strategy_id' : IDL.Text,
      'pool_id' : IDL.Opt(IDL.Text),
    }),
    'AddLiquidityToPoolFailed' : IDL.Record({
      'amount0' : IDL.Nat,
      'amount1' : IDL.Nat,
      'pool_id' : IDL.Text,
    }),
    'AddLiquidityToPoolCompleted' : IDL.Record({
      'amount0' : IDL.Nat,
      'amount1' : IDL.Nat,
      'pool_id' : IDL.Text,
    }),
    'ExternalCallFailed' : IDL.Record({
      'service' : IDL.Text,
      'method' : IDL.Text,
      'error' : IDL.Text,
      'params' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    }),
    'SwapTokenFailed' : IDL.Record({
      'token_in' : IDL.Principal,
      'amount_in' : IDL.Nat,
      'token_out' : IDL.Principal,
      'pool_id' : IDL.Text,
    }),
    'RemoveLiquidityFromPoolStarted' : IDL.Record({
      'amount0' : IDL.Nat,
      'amount1' : IDL.Nat,
      'pool_id' : IDL.Text,
    }),
    'AddLiquidityToPoolStarted' : IDL.Record({
      'amount0' : IDL.Nat,
      'amount1' : IDL.Nat,
      'pool_id' : IDL.Text,
    }),
    'StrategyDepositStarted' : IDL.Record({
      'strategy_id' : IDL.Text,
      'amount0' : IDL.Nat,
      'pool_id' : IDL.Opt(IDL.Text),
    }),
    'StrategyDepositCompleted' : IDL.Record({
      'strategy_id' : IDL.Text,
      'amount0' : IDL.Nat,
      'pool_id' : IDL.Opt(IDL.Text),
    }),
    'StrategyRebalanceFailed' : IDL.Record({
      'new_pool_id' : IDL.Opt(IDL.Text),
      'strategy_id' : IDL.Text,
      'previous_pool_id' : IDL.Opt(IDL.Text),
    }),
    'RemoveLiquidityFromPoolFailed' : IDL.Record({
      'amount0' : IDL.Nat,
      'amount1' : IDL.Nat,
      'pool_id' : IDL.Text,
    }),
    'SwapTokenCompleted' : IDL.Record({
      'token_in' : IDL.Principal,
      'amount_out' : IDL.Nat,
      'amount_in' : IDL.Nat,
      'token_out' : IDL.Principal,
    }),
    'RemoveLiquidityFromPoolCompleted' : IDL.Record({
      'amount0' : IDL.Nat,
      'amount1' : IDL.Nat,
      'pool_id' : IDL.Text,
    }),
    'StrategyRebalanceStarted' : IDL.Record({
      'strategy_id' : IDL.Text,
      'previous_pool_id' : IDL.Opt(IDL.Text),
    }),
    'SwapTokenStarted' : IDL.Record({
      'token_in' : IDL.Principal,
      'amount_in' : IDL.Nat,
      'token_out' : IDL.Principal,
      'pool_id' : IDL.Text,
    }),
    'ExternalCallStarted' : IDL.Record({
      'service' : IDL.Text,
      'method' : IDL.Text,
      'params' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    }),
    'StrategyWithdrawFailed' : IDL.Record({
      'shares' : IDL.Nat,
      'strategy_id' : IDL.Text,
      'pool_id' : IDL.Opt(IDL.Text),
    }),
    'StrategyRebalanceCompleted' : IDL.Record({
      'new_pool_id' : IDL.Opt(IDL.Text),
      'strategy_id' : IDL.Text,
      'previous_pool_id' : IDL.Opt(IDL.Text),
    }),
    'StrategyDepositFailed' : IDL.Record({
      'strategy_id' : IDL.Text,
      'amount0' : IDL.Nat,
      'pool_id' : IDL.Opt(IDL.Text),
    }),
  });
  const EventLog = IDL.Record({
    'id' : IDL.Nat64,
    'user' : IDL.Opt(IDL.Principal),
    'error' : IDL.Opt(InternalError),
    'timestamp' : IDL.Nat64,
    'correlation_id' : IDL.Text,
    'event_type' : EventLogType,
    'params' : EventLogParams,
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
    'amount' : IDL.Nat,
  });
  const StrategyWithdrawResponse = IDL.Record({
    'current_shares' : IDL.Nat,
    'amount' : IDL.Nat,
  });
  const Result_1 = IDL.Variant({
    'Ok' : StrategyWithdrawResponse,
    'Err' : ResponseError,
  });
  return IDL.Service({
    'deposit' : IDL.Func([StrategyDepositArgs], [Result], []),
    'get_config' : IDL.Func([], [Conf], ['query']),
    'get_event_logs' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [IDL.Vec(EventLog)],
        [],
      ),
    'get_strategies' : IDL.Func([], [IDL.Vec(StrategyResponse)], ['query']),
    'icpswap_withdraw' : IDL.Func(
        [IDL.Principal, IDL.Nat, IDL.Nat],
        [IDL.Nat],
        [],
      ),
    'icrc10_supported_standards' : IDL.Func(
        [],
        [IDL.Vec(SupportedStandard)],
        ['query'],
      ),
    'icrc28_trusted_origins' : IDL.Func([], [Icrc28TrustedOriginsResponse], []),
    'reset_strategy' : IDL.Func([IDL.Nat16], [], []),
    'user_strategies' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(UserStrategyResponse)],
        [],
      ),
    'withdraw' : IDL.Func([StrategyWithdrawArgs], [Result_1], []),
  });
};
export const init = ({ IDL }) => {
  const Conf = IDL.Record({ 'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)) });
  return [IDL.Opt(Conf)];
};
