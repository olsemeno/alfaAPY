import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AddLiquidityResponse {
  'token_0_amount' : bigint,
  'token_1_amount' : bigint,
  'position_id' : bigint,
}
export type AddLiquidityResult = { 'Ok' : AddLiquidityResponse } |
  { 'Err' : ResponseError };
export type AddPoolResult = { 'Ok' : string } |
  { 'Err' : ResponseError };
export interface ApyValue { 'tokens_apy' : number, 'usd_apy' : number }
export type DeletePoolResult = { 'Ok' : null } |
  { 'Err' : ResponseError };
export type ExchangeId = { 'Sonic' : null } |
  { 'KongSwap' : null } |
  { 'ICPSwap' : null };
export type GetPoolByIdResult = { 'Ok' : Pool } |
  { 'Err' : ResponseError };
export type GetPoolsResult = { 'Ok' : Array<Pool> } |
  { 'Err' : ResponseError };
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
  'position_id' : [] | [bigint],
}
export interface PoolData { 'tvl' : bigint }
export interface PoolMetrics { 'apy' : ApyValue, 'tvl' : bigint }
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
export type TestCreatePoolSnapshotResult = { 'Ok' : PoolSnapshot } |
  { 'Err' : ResponseError };
export interface WithdrawLiquidityResponse {
  'token_0_amount' : bigint,
  'token_1_amount' : bigint,
}
export type WithdrawLiquidityResult = { 'Ok' : WithdrawLiquidityResponse } |
  { 'Err' : ResponseError };
export interface _SERVICE {
  'add_liquidity_to_pool' : ActorMethod<
    [Principal, string, bigint],
    AddLiquidityResult
  >,
  'add_pool' : ActorMethod<[Principal, Principal, ExchangeId], AddPoolResult>,
  'delete_pool' : ActorMethod<[string], DeletePoolResult>,
  'get_pool_by_id' : ActorMethod<[string], GetPoolByIdResult>,
  'get_pool_metrics' : ActorMethod<
    [Array<string>],
    Array<[string, PoolMetrics]>
  >,
  'get_pools' : ActorMethod<[], GetPoolsResult>,
  'get_pools_snapshots' : ActorMethod<
    [Array<string>],
    Array<[string, Array<PoolSnapshot>]>
  >,
  'set_operator' : ActorMethod<[Principal], undefined>,
  'test_add_pool_snapshot' : ActorMethod<[PoolSnapshotArgs], undefined>,
  'test_create_pool_snapshot' : ActorMethod<
    [string],
    TestCreatePoolSnapshotResult
  >,
  'test_delete_all_pools_and_snapshots' : ActorMethod<[], undefined>,
  'test_delete_pool_snapshot' : ActorMethod<[string, string], undefined>,
  'test_delete_pool_snapshots' : ActorMethod<[string], undefined>,
  'test_update_pool_ids' : ActorMethod<[], undefined>,
  'withdraw_liquidity_from_pool' : ActorMethod<
    [string],
    WithdrawLiquidityResult
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
