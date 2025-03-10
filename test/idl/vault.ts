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
export type Result = { 'Ok' : bigint } |
    { 'Err' : string };
export interface StrategyResponse {
    'id' : number,
    'name' : string,
    'description' : string,
    'pools' : Array<string>,
}
export interface SuccessResult { 'amount_out' : bigint }
export interface WithdrawArgs {
    'strategy_id' : number,
    'ledger' : Principal,
    'amount' : bigint,
}
export interface _SERVICE {
    'accept_investment' : ActorMethod<[AcceptInvestmentArgs], DepositResponse>,
    'get_config' : ActorMethod<[], Conf>,
    'get_strategies' : ActorMethod<[], Array<StrategyResponse>>,
    'kong_pools' : ActorMethod<[], PoolsReply>,
    'swap' : ActorMethod<[], SuccessResult>,
    'withdraw' : ActorMethod<[WithdrawArgs], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
