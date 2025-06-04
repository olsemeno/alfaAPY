export const idlFactory = ({ IDL }) => {
    const Conf = IDL.Record({ 'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)) });
    const AcceptInvestmentArgs = IDL.Record({
        'strategy_id' : IDL.Nat16,
        'ledger' : IDL.Principal,
        'amount' : IDL.Nat,
    });
    const DepositResponse = IDL.Record({
        'request_id' : IDL.Nat64,
        'tx_id' : IDL.Nat64,
        'shares' : IDL.Nat,
        'amount' : IDL.Nat,
    });
    const ExchangeId = IDL.Variant({
        'Sonic' : IDL.Null,
        'KongSwap' : IDL.Null,
        'ICPSwap' : IDL.Null,
    });
    const TokenInfo = IDL.Record({
        'ledger' : IDL.Principal,
        'symbol' : IDL.Text,
    });
    const Pool = IDL.Record({
        'provider' : ExchangeId,
        'token0' : TokenInfo,
        'token1' : TokenInfo,
    });
    const PoolResponse = IDL.Record({
        'provider' : ExchangeId,
        'token0' : IDL.Text,
        'token1' : IDL.Text,
    });
    const StrategyResponse = IDL.Record({
        'id' : IDL.Nat16,
        'name' : IDL.Text,
        'description' : IDL.Text,
        'total_shares' : IDL.Nat,
        'initial_deposit' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat)),
        'user_shares' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat)),
        'current_pool' : IDL.Opt(Pool),
        'pools' : IDL.Vec(PoolResponse),
    });
    const SystemEventDetails = IDL.Variant({
        'Swap' : IDL.Null,
        'Rebalance' : IDL.Record({ 'old_pool' : IDL.Text, 'new_pool' : IDL.Text }),
    });
    const SystemEventType = IDL.Variant({
        'Swap' : IDL.Null,
        'Rebalance' : IDL.Null,
    });
    const SystemEvent = IDL.Record({
        'id' : IDL.Nat64,
        'timestamp' : IDL.Nat64,
        'details' : SystemEventDetails,
        'event_type' : SystemEventType,
    });
    const UserEventDetails = IDL.Variant({
        'AddLiquidity' : IDL.Record({
        'token' : IDL.Text,
        'amount' : IDL.Nat,
        'symbol' : IDL.Text,
        }),
        'RemoveLiquidity' : IDL.Record({
        'token' : IDL.Text,
        'amount' : IDL.Nat,
        'symbol' : IDL.Text,
        }),
    });
    const UserEventType = IDL.Variant({
        'AddLiquidity' : IDL.Null,
        'RemoveLiquidity' : IDL.Null,
    });
    const UserEvent = IDL.Record({
        'id' : IDL.Nat64,
        'user' : IDL.Principal,
        'timestamp' : IDL.Nat64,
        'details' : UserEventDetails,
        'event_type' : UserEventType,
    });
    const SupportedStandard = IDL.Record({ 'url' : IDL.Text, 'name' : IDL.Text });
    const Icrc28TrustedOriginsResponse = IDL.Record({
        'trusted_origins' : IDL.Vec(IDL.Text),
    });
    const LPReply = IDL.Record({
        'ts' : IDL.Nat64,
        'usd_balance' : IDL.Float64,
        'balance' : IDL.Float64,
        'name' : IDL.Text,
        'amount_0' : IDL.Float64,
        'amount_1' : IDL.Float64,
        'address_0' : IDL.Text,
        'address_1' : IDL.Text,
        'symbol_0' : IDL.Text,
        'symbol_1' : IDL.Text,
        'usd_amount_0' : IDL.Float64,
        'usd_amount_1' : IDL.Float64,
        'chain_0' : IDL.Text,
        'chain_1' : IDL.Text,
        'symbol' : IDL.Text,
        'lp_token_id' : IDL.Nat64,
    });
    const UserBalancesReply = IDL.Variant({ 'LP' : LPReply });
    const UserStrategyResponse = IDL.Record({
        'strategy_current_pool' : Pool,
        'total_shares' : IDL.Nat,
        'strategy_id' : IDL.Nat16,
        'initial_deposit' : IDL.Nat,
        'user_shares' : IDL.Nat,
        'strategy_name' : IDL.Text,
        'users_count' : IDL.Nat32,
    });
    const WithdrawArgs = IDL.Record({
        'strategy_id' : IDL.Nat16,
        'ledger' : IDL.Principal,
        'amount' : IDL.Nat,
    });
    const WithdrawResponse = IDL.Record({
        'current_shares' : IDL.Nat,
        'amount' : IDL.Nat,
    });
    return IDL.Service({
        'accept_investment' : IDL.Func(
            [AcceptInvestmentArgs],
            [DepositResponse],
            [],
        ),
        'get_config' : IDL.Func([], [Conf], ['query']),
        'get_strategies' : IDL.Func([], [IDL.Vec(StrategyResponse)], ['query']),
        'get_system_events' : IDL.Func(
            [IDL.Nat64, IDL.Nat64],
            [IDL.Vec(SystemEvent)],
            [],
        ),
        'get_user_events' : IDL.Func(
            [IDL.Principal, IDL.Nat64, IDL.Nat64],
            [IDL.Vec(UserEvent)],
            [],
        ),
        'icpswap_withdraw' : IDL.Func([TokenInfo, IDL.Nat, IDL.Nat], [IDL.Nat], []),
        'icrc10_supported_standards' : IDL.Func(
            [],
            [IDL.Vec(SupportedStandard)],
            ['query'],
        ),
        'icrc28_trusted_origins' : IDL.Func([], [Icrc28TrustedOriginsResponse], []),
        'user_balance_all' : IDL.Func([], [IDL.Vec(UserBalancesReply)], []),
        'user_strategies' : IDL.Func(
            [IDL.Principal],
            [IDL.Vec(UserStrategyResponse)],
            [],
        ),
        'withdraw' : IDL.Func([WithdrawArgs], [WithdrawResponse], []),
        'swap_icrc2_icpswap' : IDL.Func([TokenInfo, IDL.Nat, TokenInfo], [IDL.Nat], []),
    });
    };
    export const init = ({ IDL }) => {
    const Conf = IDL.Record({ 'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)) });
    return [IDL.Opt(Conf)];
};
