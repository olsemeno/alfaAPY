import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AddLiquidityResponse {
  'request_id' : bigint,
  'token_0_amount' : bigint,
  'token_1_amount' : bigint,
}
export interface ApyValue { 'tokens_apy' : bigint, 'usd_apy' : bigint }
export type ExchangeId = { 'Sonic' : null } |
  { 'KongSwap' : null } |
  { 'ICPSwap' : null };
export interface InternalError {
  'context' : string,
  'code' : number,
  'kind' : InternalErrorKind,
  'extra' : [] | [Array<[string, string]>],
  'message' : string,
}
export type InternalErrorKind = { 'AccessDenied' : null } |
  { 'Infrastructure' : null } |
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
  'position_id' : [] | [bigint],
}
export interface PoolApy {
  'month' : ApyValue,
  'week' : ApyValue,
  'year' : ApyValue,
}
export interface PoolData { 'tvl' : bigint }
export interface PoolMetrics { 'apy' : PoolApy, 'tvl' : bigint }
export interface PoolSnapshot {
  'id' : string,
  'pool_data' : [] | [PoolData],
  'timestamp' : bigint,
  'pool_id' : string,
  'position_data' : [] | [PositionData],
}
export interface PoolSnapshotArgs {
  'pool_data' : [] | [PoolData],
  'timestamp' : bigint,
  'pool_id' : string,
  'position_data' : [] | [PositionData],
}
export interface PositionData {
  'id' : bigint,
  'usd_amount0' : bigint,
  'usd_amount1' : bigint,
  'amount0' : bigint,
  'amount1' : bigint,
}
export interface ResponseError {
  'source' : [] | [InternalError],
  'code' : number,
  'kind' : InternalErrorKind,
  'message' : string,
  'details' : [] | [Array<[string, string]>],
}
export type Result = { 'Ok' : AddLiquidityResponse } |
  { 'Err' : ResponseError };
export type Result_1 = { 'Ok' : string } |
  { 'Err' : ResponseError };
export type Result_2 = { 'Ok' : null } |
  { 'Err' : ResponseError };
export type Result_3 = { 'Ok' : Pool } |
  { 'Err' : ResponseError };
export type Result_4 = { 'Ok' : Array<Pool> } |
  { 'Err' : ResponseError };
export type Result_5 = { 'Ok' : WithdrawLiquidityResponse } |
  { 'Err' : ResponseError };
export interface WithdrawLiquidityResponse {
  'token_0_amount' : bigint,
  'token_1_amount' : bigint,
}
export interface _SERVICE {
  'add_liquidity_to_pool' : ActorMethod<[Principal, string, bigint], Result>,
  'add_pool' : ActorMethod<[Principal, Principal, ExchangeId], Result_1>,
  'add_pool_snapshot' : ActorMethod<[PoolSnapshotArgs], undefined>,
  'create_pool_snapshot' : ActorMethod<[string], PoolSnapshot>,
  'delete_all_pools_and_snapshots' : ActorMethod<[], boolean>,
  'delete_pool' : ActorMethod<[string], Result_2>,
  'delete_pool_snapshot' : ActorMethod<[string, string], undefined>,
  'delete_pool_snapshots' : ActorMethod<[string], undefined>,
  'get_pool_by_id' : ActorMethod<[string], Result_3>,
  'get_pool_metrics' : ActorMethod<
    [Array<string>],
    Array<[string, PoolMetrics]>
  >,
  'get_pools' : ActorMethod<[], Result_4>,
  'get_pools_snapshots' : ActorMethod<
    [Array<string>],
    Array<[string, Array<PoolSnapshot>]>
  >,
  'remove_liquidity_from_pool' : ActorMethod<[string], Result_5>,
  'set_operator' : ActorMethod<[Principal], undefined>,
  'update_pool_ids' : ActorMethod<[], boolean>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
