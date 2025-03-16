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
export interface Icrc28TrustedOriginsResponse {
    'trusted_origins' : Array<string>,
}
export interface LPReply {
    'ts' : bigint,
    'usd_balance' : number,
    'balance' : number,
    'name' : string,
    'amount_0' : number,
    'amount_1' : number,
    'address_0' : string,
    'address_1' : string,
    'symbol_0' : string,
    'symbol_1' : string,
    'usd_amount_0' : number,
    'usd_amount_1' : number,
    'chain_0' : string,
    'chain_1' : string,
    'symbol' : string,
}
export interface PoolReply {
    'tvl' : bigint,
    'lp_token_symbol' : string,
    'name' : string,
    'lp_fee_0' : bigint,
    'lp_fee_1' : bigint,
    'balance_0' : bigint,
    'balance_1' : bigint,
    'rolling_24h_volume' : bigint,
    'rolling_24h_apy' : number,
    'address_0' : string,
    'address_1' : string,
    'rolling_24h_num_swaps' : bigint,
    'symbol_0' : string,
    'symbol_1' : string,
    'pool_id' : number,
    'price' : number,
    'chain_0' : string,
    'chain_1' : string,
    'is_removed' : boolean,
    'symbol' : string,
    'rolling_24h_lp_fee' : bigint,
    'lp_fee_bps' : number,
}
export interface PoolsReply {
    'total_24h_lp_fee' : bigint,
    'total_tvl' : bigint,
    'total_24h_volume' : bigint,
    'pools' : Array<PoolReply>,
    'total_24h_num_swaps' : bigint,
}
export interface StrategyResponse {
    'id' : number,
    'name' : string,
    'description' : string,
    'total_shares' : bigint,
    'user_shares' : Array<[Principal, bigint]>,
    'current_pool' : [] | [PoolReply],
    'pools' : Array<string>,
    'initial_deposit' : Array<[Principal, bigint]>,
}
export interface SupportedStandard { 'url' : string, 'name' : string }
export type UserBalancesReply = { 'LP' : LPReply };
export interface UserStrategyResponse {
    'strategy_current_pool' : string,
    'total_shares' : bigint,
    'strategy_id' : number,
    'user_shares' : bigint,
    'strategy_name' : string,
    'initial_deposit' : bigint,
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
    'icrc10_supported_standards' : ActorMethod<[], Array<SupportedStandard>>,
    'icrc28_trusted_origins' : ActorMethod<[], Icrc28TrustedOriginsResponse>,
    'kong_pools' : ActorMethod<[], PoolsReply>,
    'user_balance_all' : ActorMethod<[Principal], Array<UserBalancesReply>>,
    'user_strategies' : ActorMethod<[Principal], Array<UserStrategyResponse>>,
    'withdraw' : ActorMethod<[WithdrawArgs], WithdrawResponse>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];