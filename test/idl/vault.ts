import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Conf { 'controllers' : [] | [Array<Principal>] }
export interface EventLog {
  'id' : bigint,
  'user' : [] | [Principal],
  'error' : [] | [InternalError],
  'timestamp' : bigint,
  'correlation_id' : string,
  'event_type' : EventLogType,
  'params' : EventLogParams,
}
export type EventLogParams = {
    'ExternalCallCompleted' : {
      'service' : string,
      'result' : Array<[string, string]>,
      'method' : string,
      'params' : Array<[string, string]>,
    }
  } |
  {
    'StrategyWithdrawCompleted' : {
      'shares' : [] | [bigint],
      'strategy_id' : string,
      'amount0' : [] | [bigint],
      'pool_id' : [] | [string],
    }
  } |
  {
    'StrategyWithdrawStarted' : {
      'shares' : [] | [bigint],
      'strategy_id' : string,
      'pool_id' : [] | [string],
    }
  } |
  {
    'AddLiquidityToPoolFailed' : {
      'amount0' : [] | [bigint],
      'amount1' : [] | [bigint],
      'pool_id' : string,
    }
  } |
  {
    'AddLiquidityToPoolCompleted' : {
      'amount0' : [] | [bigint],
      'amount1' : [] | [bigint],
      'pool_id' : string,
    }
  } |
  {
    'WithdrawLiquidityFromPoolStarted' : {
      'amount0' : [] | [bigint],
      'amount1' : [] | [bigint],
      'pool_id' : string,
    }
  } |
  {
    'ExternalCallFailed' : {
      'service' : string,
      'method' : string,
      'error' : string,
      'params' : Array<[string, string]>,
    }
  } |
  {
    'SwapTokenFailed' : {
      'token_in' : Principal,
      'amount_in' : [] | [bigint],
      'token_out' : Principal,
      'pool_id' : string,
    }
  } |
  {
    'AddLiquidityToPoolStarted' : {
      'amount0' : [] | [bigint],
      'amount1' : [] | [bigint],
      'pool_id' : string,
    }
  } |
  {
    'StrategyDepositStarted' : {
      'strategy_id' : string,
      'amount0' : [] | [bigint],
      'pool_id' : [] | [string],
    }
  } |
  {
    'StrategyDepositCompleted' : {
      'strategy_id' : string,
      'amount0' : [] | [bigint],
      'pool_id' : [] | [string],
    }
  } |
  {
    'StrategyRebalanceFailed' : {
      'new_pool_id' : [] | [string],
      'strategy_id' : string,
      'previous_pool_id' : [] | [string],
    }
  } |
  {
    'SwapTokenCompleted' : {
      'token_in' : Principal,
      'amount_out' : [] | [bigint],
      'amount_in' : [] | [bigint],
      'token_out' : Principal,
    }
  } |
  {
    'WithdrawLiquidityFromPoolCompleted' : {
      'amount0' : [] | [bigint],
      'amount1' : [] | [bigint],
      'pool_id' : string,
    }
  } |
  {
    'StrategyRebalanceStarted' : {
      'strategy_id' : string,
      'previous_pool_id' : [] | [string],
    }
  } |
  {
    'SwapTokenStarted' : {
      'token_in' : Principal,
      'amount_in' : [] | [bigint],
      'token_out' : Principal,
      'pool_id' : string,
    }
  } |
  {
    'ExternalCallStarted' : {
      'service' : string,
      'method' : string,
      'params' : Array<[string, string]>,
    }
  } |
  {
    'StrategyWithdrawFailed' : {
      'shares' : [] | [bigint],
      'strategy_id' : string,
      'pool_id' : [] | [string],
    }
  } |
  {
    'WithdrawLiquidityFromPoolFailed' : {
      'amount0' : [] | [bigint],
      'amount1' : [] | [bigint],
      'pool_id' : string,
    }
  } |
  {
    'StrategyRebalanceCompleted' : {
      'new_pool_id' : [] | [string],
      'strategy_id' : string,
      'previous_pool_id' : [] | [string],
    }
  } |
  {
    'StrategyDepositFailed' : {
      'strategy_id' : string,
      'amount0' : [] | [bigint],
      'pool_id' : [] | [string],
    }
  };
export type EventLogType = { 'ExternalCallCompleted' : null } |
  { 'StrategyWithdrawCompleted' : null } |
  { 'StrategyWithdrawStarted' : null } |
  { 'AddLiquidityToPoolFailed' : null } |
  { 'AddLiquidityToPoolCompleted' : null } |
  { 'WithdrawLiquidityFromPoolStarted' : null } |
  { 'ExternalCallFailed' : null } |
  { 'SwapTokenFailed' : null } |
  { 'AddLiquidityToPoolStarted' : null } |
  { 'StrategyDepositStarted' : null } |
  { 'StrategyDepositCompleted' : null } |
  { 'StrategyRebalanceFailed' : null } |
  { 'SwapTokenCompleted' : null } |
  { 'WithdrawLiquidityFromPoolCompleted' : null } |
  { 'StrategyRebalanceStarted' : null } |
  { 'SwapTokenStarted' : null } |
  { 'ExternalCallStarted' : null } |
  { 'StrategyWithdrawFailed' : null } |
  { 'WithdrawLiquidityFromPoolFailed' : null } |
  { 'StrategyRebalanceCompleted' : null } |
  { 'StrategyDepositFailed' : null };
export type ExchangeId = { 'Sonic' : null } |
  { 'KongSwap' : null } |
  { 'ICPSwap' : null };
export interface Icrc28TrustedOriginsResponse {
  'trusted_origins' : Array<string>,
}
export interface InternalError {
  'context' : string,
  'code' : number,
  'kind' : InternalErrorKind,
  'extra' : [] | [Array<[string, string]>],
  'message' : string,
}
export type InternalErrorKind = { 'AccessDenied' : null } |
  { 'NotFound' : null } |
  { 'Timeout' : null } |
  { 'Unknown' : null } |
  { 'BusinessLogic' : null } |
  { 'ExternalService' : null } |
  { 'Validation' : null };
export interface Pool {
  'id' : string,
  'provider' : ExchangeId,
  'token0' : Principal,
  'token1' : Principal,
}
export interface ResponseError {
  'code' : number,
  'kind' : ResponseErrorKind,
  'message' : string,
  'details' : [] | [Array<[string, string]>],
}
export type ResponseErrorKind = { 'AccessDenied' : null } |
  { 'NotFound' : null } |
  { 'Timeout' : null } |
  { 'Unknown' : null } |
  { 'BusinessLogic' : null } |
  { 'ExternalService' : null } |
  { 'Validation' : null };
export interface StrategyDepositArgs {
  'strategy_id' : number,
  'ledger' : Principal,
  'amount' : bigint,
}
export interface StrategyDepositResponse {
  'tx_id' : bigint,
  'shares' : bigint,
  'amount' : bigint,
  'position_id' : bigint,
}
export type StrategyDepositResult = { 'Ok' : StrategyDepositResponse } |
  { 'Err' : ResponseError };
export type StrategyLiquidityResult = { 'Ok' : bigint } |
  { 'Err' : ResponseError };
export interface StrategyResponse {
  'id' : number,
  'name' : string,
  'description' : string,
  'total_shares' : bigint,
  'initial_deposit' : Array<[Principal, bigint]>,
  'user_shares' : Array<[Principal, bigint]>,
  'current_pool' : [] | [Pool],
  'total_balance' : bigint,
  'pools' : Array<Pool>,
}
export interface StrategyWithdrawResponse {
  'current_shares' : bigint,
  'amount' : bigint,
}
export type StrategyWithdrawResult = { 'Ok' : StrategyWithdrawResponse } |
  { 'Err' : ResponseError };
export interface SupportedStandard { 'url' : string, 'name' : string }
export interface UserStrategyResponse {
  'strategy_current_pool' : Pool,
  'total_shares' : bigint,
  'strategy_id' : number,
  'initial_deposit' : bigint,
  'user_shares' : bigint,
  'strategy_name' : string,
  'users_count' : number,
}
export interface _SERVICE {
  'deposit' : ActorMethod<[StrategyDepositArgs], StrategyDepositResult>,
  'get_config' : ActorMethod<[], Conf>,
  'get_event_logs' : ActorMethod<[bigint, bigint], Array<EventLog>>,
  'get_strategies' : ActorMethod<[], Array<StrategyResponse>>,
  'icrc10_supported_standards' : ActorMethod<[], Array<SupportedStandard>>,
  'icrc28_trusted_origins' : ActorMethod<[], Icrc28TrustedOriginsResponse>,
  'strategy_liquidity' : ActorMethod<[number], StrategyLiquidityResult>,
  'test_icpswap_withdraw' : ActorMethod<[Principal, bigint, bigint], bigint>,
  'test_reset_strategy' : ActorMethod<[number], undefined>,
  'user_strategies' : ActorMethod<[Principal], Array<UserStrategyResponse>>,
  'withdraw' : ActorMethod<[StrategyDepositArgs], StrategyWithdrawResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
