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
  'code' : ResponseErrorCode,
  'message' : string,
  'details' : [] | [Array<[string, string]>],
}
export type ResponseErrorCode = { 'AccessDenied' : null } |
  { 'NotFound' : null } |
  { 'Timeout' : null } |
  { 'Validation' : null } |
  { 'InternalError' : null };
export type Result = { 'Ok' : AddLiquidityResponse } |
  { 'Err' : ResponseError };
export type Result_1 = { 'Ok' : WithdrawFromPoolResponse } |
  { 'Err' : ResponseError };
export interface WithdrawFromPoolResponse {
  'token_0_amount' : bigint,
  'token_1_amount' : bigint,
}
export interface _SERVICE {
  'add_liquidity_to_pool' : ActorMethod<[Principal, string, bigint], Result>,
  'add_pool' : ActorMethod<[Principal, Principal, ExchangeId], string>,
  'add_pool_snapshot' : ActorMethod<[PoolSnapshotArgs], undefined>,
  'create_pool_snapshot' : ActorMethod<[string], PoolSnapshot>,
  'delete_all_pools_and_snapshots' : ActorMethod<[], boolean>,
  'delete_pool' : ActorMethod<[string], boolean>,
  'delete_pool_snapshot' : ActorMethod<[string, string], undefined>,
  'delete_pool_snapshots' : ActorMethod<[string], undefined>,
  'get_pool_by_id' : ActorMethod<[string], [] | [Pool]>,
  'get_pool_metrics' : ActorMethod<
    [Array<string>],
    Array<[string, PoolMetrics]>
  >,
  'get_pools' : ActorMethod<[], Array<Pool>>,
  'get_pools_snapshots' : ActorMethod<
    [Array<string>],
    Array<[string, Array<PoolSnapshot>]>
  >,
  'remove_liquidity_from_pool' : ActorMethod<[string], Result_1>,
  'set_operator' : ActorMethod<[Principal], undefined>,
  'update_pool_ids' : ActorMethod<[], boolean>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
