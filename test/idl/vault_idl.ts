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
    const StrategyResponse = IDL.Record({
        'id' : IDL.Nat16,
        'name' : IDL.Text,
        'description' : IDL.Text,
        'pools' : IDL.Vec(IDL.Text),
    });
    const PoolReply = IDL.Record({
        'tvl' : IDL.Nat,
        'lp_token_symbol' : IDL.Text,
        'name' : IDL.Text,
        'lp_fee_0' : IDL.Nat,
        'lp_fee_1' : IDL.Nat,
        'balance_0' : IDL.Nat,
        'balance_1' : IDL.Nat,
        'rolling_24h_volume' : IDL.Nat,
        'rolling_24h_apy' : IDL.Float64,
        'address_0' : IDL.Text,
        'address_1' : IDL.Text,
        'rolling_24h_num_swaps' : IDL.Nat,
        'symbol_0' : IDL.Text,
        'symbol_1' : IDL.Text,
        'pool_id' : IDL.Nat32,
        'price' : IDL.Float64,
        'chain_0' : IDL.Text,
        'chain_1' : IDL.Text,
        'is_removed' : IDL.Bool,
        'symbol' : IDL.Text,
        'rolling_24h_lp_fee' : IDL.Nat,
        'lp_fee_bps' : IDL.Nat8,
    });
    const PoolsReply = IDL.Record({
        'total_24h_lp_fee' : IDL.Nat,
        'total_tvl' : IDL.Nat,
        'total_24h_volume' : IDL.Nat,
        'pools' : IDL.Vec(PoolReply),
        'total_24h_num_swaps' : IDL.Nat,
    });
    const SuccessResult = IDL.Record({ 'amount_out' : IDL.Nat });
    const WithdrawArgs = IDL.Record({
        'strategy_id' : IDL.Nat16,
        'ledger' : IDL.Principal,
        'amount' : IDL.Nat,
    });
    const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text });
    return IDL.Service({
        'accept_investment' : IDL.Func(
            [AcceptInvestmentArgs],
            [DepositResponse],
            [],
        ),
        'get_config' : IDL.Func([], [Conf], ['query']),
        'get_strategies' : IDL.Func([], [IDL.Vec(StrategyResponse)], ['query']),
        'kong_pools' : IDL.Func([], [PoolsReply], []),
        'swap' : IDL.Func([], [SuccessResult], []),
        'withdraw' : IDL.Func([WithdrawArgs], [Result], []),
    });
};
export const init = ({ IDL }) => {
    const Conf = IDL.Record({ 'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)) });
    return [IDL.Opt(Conf)];
};
