import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AcceptInvestmentArgs {
  'strategy_id' : number,
  'ledger' : Principal,
  'amount' : bigint,
}
export interface Conf { 'controllers' : [] | [Array<Principal>] }
export interface DepositResponse {
  'request_id' : bigint,
  'tx_id' : bigint,
  'shares' : bigint,
  'amount' : bigint,
}
export type ExchangeId = { 'Sonic' : null } |
  { 'KongSwap' : null } |
  { 'ICPSwap' : null };
export interface Icrc28TrustedOriginsResponse {
  'trusted_origins' : Array<string>,
}
export interface Pool {
  'id' : string,
  'provider' : ExchangeId,
  'token0' : Principal,
  'token1' : Principal,
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
export interface SupportedStandard { 'url' : string, 'name' : string }
export interface SystemEvent {
  'id' : bigint,
  'timestamp' : bigint,
  'details' : SystemEventDetails,
  'event_type' : SystemEventType,
}
export type SystemEventDetails = { 'Swap' : null } |
  { 'Rebalance' : { 'old_pool' : string, 'new_pool' : string } };
export type SystemEventType = { 'Swap' : null } |
  { 'Rebalance' : null };
export interface UserEvent {
  'id' : bigint,
  'user' : Principal,
  'timestamp' : bigint,
  'details' : UserEventDetails,
  'event_type' : UserEventType,
}
export type UserEventDetails = {
    'AddLiquidity' : { 'token' : Principal, 'amount' : bigint }
  } |
  { 'RemoveLiquidity' : { 'token' : Principal, 'amount' : bigint } };
export type UserEventType = { 'AddLiquidity' : null } |
  { 'RemoveLiquidity' : null };
export interface UserStrategyResponse {
  'strategy_current_pool' : Pool,
  'total_shares' : bigint,
  'strategy_id' : number,
  'initial_deposit' : bigint,
  'user_shares' : bigint,
  'strategy_name' : string,
  'users_count' : number,
}
export interface WithdrawArgs {
  'strategy_id' : number,
  'ledger' : Principal,
  'amount' : bigint,
}
export interface WithdrawResponse {
  'current_shares' : bigint,
  'amount' : bigint,
}
export interface _SERVICE {
  'accept_investment' : ActorMethod<[AcceptInvestmentArgs], DepositResponse>,
  'get_config' : ActorMethod<[], Conf>,
  'get_strategies' : ActorMethod<[], Array<StrategyResponse>>,
  'get_system_events' : ActorMethod<[bigint, bigint], Array<SystemEvent>>,
  'get_user_events' : ActorMethod<
    [Principal, bigint, bigint],
    Array<UserEvent>
  >,
  'icpswap_withdraw' : ActorMethod<[Principal, bigint, bigint], bigint>,
  'icrc10_supported_standards' : ActorMethod<[], Array<SupportedStandard>>,
  'icrc28_trusted_origins' : ActorMethod<[], Icrc28TrustedOriginsResponse>,
  'reset_strategy' : ActorMethod<[number], undefined>,
  'user_strategies' : ActorMethod<[Principal], Array<UserStrategyResponse>>,
  'withdraw' : ActorMethod<[WithdrawArgs], WithdrawResponse>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
