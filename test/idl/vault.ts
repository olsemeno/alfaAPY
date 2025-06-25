import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AddLiquidityToPoolCompleted {
  'amount0' : [] | [bigint],
  'amount1' : [] | [bigint],
  'pool_id' : string,
}
export interface AddLiquidityToPoolFailed {
  'error' : InternalError,
  'amount0' : [] | [bigint],
  'pool_id' : string,
}
export interface AddLiquidityToPoolStarted {
  'amount0' : [] | [bigint],
  'amount1' : [] | [bigint],
  'pool_id' : string,
}
export interface Conf { 'controllers' : [] | [Array<Principal>] }
export type Event = {
    'StrategyWithdrawCompleted' : StrategyWithdrawCompleted
  } |
  { 'StrategyWithdrawStarted' : StrategyWithdrawStarted } |
  { 'AddLiquidityToPoolFailed' : AddLiquidityToPoolFailed } |
  { 'AddLiquidityToPoolCompleted' : AddLiquidityToPoolCompleted } |
  { 'WithdrawLiquidityFromPoolStarted' : WithdrawLiquidityFromPoolStarted } |
  { 'SwapTokenFailed' : SwapTokenFailed } |
  { 'AddLiquidityToPoolStarted' : AddLiquidityToPoolStarted } |
  { 'StrategyDepositStarted' : StrategyDepositStarted } |
  { 'StrategyDepositCompleted' : StrategyDepositCompleted } |
  { 'StrategyRebalanceFailed' : StrategyRebalanceFailed } |
  { 'SwapTokenCompleted' : SwapTokenCompleted } |
  {
    'WithdrawLiquidityFromPoolCompleted' : WithdrawLiquidityFromPoolCompleted
  } |
  { 'StrategyRebalanceStarted' : StrategyRebalanceStarted } |
  { 'SwapTokenStarted' : SwapTokenStarted } |
  { 'StrategyWithdrawFailed' : StrategyWithdrawFailed } |
  { 'WithdrawLiquidityFromPoolFailed' : WithdrawLiquidityFromPoolFailed } |
  { 'StrategyRebalanceCompleted' : StrategyRebalanceCompleted } |
  { 'StrategyDepositFailed' : StrategyDepositFailed };
export interface EventRecord {
  'id' : bigint,
  'user' : [] | [Principal],
  'event' : Event,
  'timestamp' : bigint,
  'correlation_id' : string,
}
export type ExchangeId = { 'Sonic' : null } |
  { 'KongSwap' : null } |
  { 'ICPSwap' : null };
export type GetEventRecordsResult = { 'Ok' : Array<EventRecord> } |
  { 'Err' : ResponseError };
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
  'kind' : InternalErrorKind,
  'message' : string,
  'details' : [] | [Array<[string, string]>],
}
export interface StrategyDepositArgs {
  'strategy_id' : number,
  'ledger' : Principal,
  'amount' : bigint,
}
export interface StrategyDepositCompleted {
  'strategy_id' : string,
  'amount0' : [] | [bigint],
  'pool_id' : [] | [string],
}
export interface StrategyDepositFailed {
  'error' : InternalError,
  'strategy_id' : string,
  'amount0' : [] | [bigint],
  'pool_id' : [] | [string],
}
export interface StrategyDepositResponse {
  'tx_id' : bigint,
  'shares' : bigint,
  'amount' : bigint,
  'position_id' : bigint,
}
export type StrategyDepositResult = { 'Ok' : StrategyDepositResponse } |
  { 'Err' : ResponseError };
export interface StrategyDepositStarted {
  'strategy_id' : string,
  'amount0' : [] | [bigint],
  'pool_id' : [] | [string],
}
export interface StrategyRebalanceCompleted {
  'new_pool_id' : [] | [string],
  'strategy_id' : string,
  'previous_pool_id' : [] | [string],
}
export interface StrategyRebalanceFailed {
  'new_pool_id' : [] | [string],
  'error' : InternalError,
  'strategy_id' : string,
  'previous_pool_id' : [] | [string],
}
export interface StrategyRebalanceStarted {
  'strategy_id' : string,
  'previous_pool_id' : [] | [string],
}
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
export interface StrategyWithdrawArgs {
  'strategy_id' : number,
  'ledger' : Principal,
  'percentage' : bigint,
}
export interface StrategyWithdrawCompleted {
  'shares' : [] | [bigint],
  'strategy_id' : string,
  'amount0' : [] | [bigint],
  'pool_id' : [] | [string],
}
export interface StrategyWithdrawFailed {
  'shares' : [] | [bigint],
  'error' : InternalError,
  'strategy_id' : string,
  'pool_id' : [] | [string],
}
export interface StrategyWithdrawResponse {
  'current_shares' : bigint,
  'amount' : bigint,
}
export type StrategyWithdrawResult = { 'Ok' : StrategyWithdrawResponse } |
  { 'Err' : ResponseError };
export interface StrategyWithdrawStarted {
  'shares' : [] | [bigint],
  'strategy_id' : string,
  'pool_id' : [] | [string],
}
export interface SupportedStandard { 'url' : string, 'name' : string }
export interface SwapTokenCompleted {
  'token_in' : Principal,
  'amount_out' : [] | [bigint],
  'amount_in' : [] | [bigint],
  'token_out' : Principal,
  'pool_id' : string,
}
export interface SwapTokenFailed {
  'token_in' : Principal,
  'error' : InternalError,
  'amount_in' : [] | [bigint],
  'token_out' : Principal,
  'pool_id' : string,
}
export interface SwapTokenStarted {
  'token_in' : Principal,
  'amount_in' : [] | [bigint],
  'token_out' : Principal,
  'pool_id' : string,
}
export interface UserStrategyResponse {
  'strategy_current_pool' : Pool,
  'total_shares' : bigint,
  'strategy_id' : number,
  'initial_deposit' : bigint,
  'user_shares' : bigint,
  'strategy_name' : string,
  'users_count' : number,
}
export interface WithdrawLiquidityFromPoolCompleted {
  'shares' : bigint,
  'total_shares' : bigint,
  'amount_token0' : bigint,
  'amount_token1' : bigint,
  'pool_id' : string,
}
export interface WithdrawLiquidityFromPoolFailed {
  'shares' : bigint,
  'total_shares' : bigint,
  'error' : InternalError,
  'pool_id' : string,
}
export interface WithdrawLiquidityFromPoolStarted {
  'shares' : bigint,
  'total_shares' : bigint,
  'pool_id' : string,
}
export interface _SERVICE {
  'deposit' : ActorMethod<[StrategyDepositArgs], StrategyDepositResult>,
  'get_config' : ActorMethod<[], Conf>,
  'get_event_records' : ActorMethod<[bigint, bigint], GetEventRecordsResult>,
  'get_strategies' : ActorMethod<[], Array<StrategyResponse>>,
  'icrc10_supported_standards' : ActorMethod<[], Array<SupportedStandard>>,
  'icrc28_trusted_origins' : ActorMethod<[], Icrc28TrustedOriginsResponse>,
  'test_icpswap_withdraw' : ActorMethod<[Principal, bigint, bigint], bigint>,
  'test_reset_strategy' : ActorMethod<[number], undefined>,
  'test_update_strategy_stats' : ActorMethod<[], undefined>,
  'user_strategies' : ActorMethod<[Principal], Array<UserStrategyResponse>>,
  'withdraw' : ActorMethod<[StrategyWithdrawArgs], StrategyWithdrawResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
