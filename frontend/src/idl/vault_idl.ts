/* eslint-disable @typescript-eslint/no-explicit-any */
export const idlFactory = ({ IDL }: { IDL: any }) => {
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
  const Pool = IDL.Record({
    'id' : IDL.Text,
    'provider' : ExchangeId,
    'token0' : IDL.Principal,
    'token1' : IDL.Principal,
  });
  const StrategyResponse = IDL.Record({
    'id' : IDL.Nat16,
    'name' : IDL.Text,
    'description' : IDL.Text,
    'total_shares' : IDL.Nat,
    'initial_deposit' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat)),
    'user_shares' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat)),
    'current_pool' : IDL.Opt(Pool),
    'total_balance' : IDL.Nat,
    'pools' : IDL.Vec(Pool),
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
      'token' : IDL.Principal,
      'amount' : IDL.Nat,
    }),
    'RemoveLiquidity' : IDL.Record({
      'token' : IDL.Principal,
      'amount' : IDL.Nat,
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
    'icpswap_withdraw' : IDL.Func(
        [IDL.Principal, IDL.Nat, IDL.Nat],
        [IDL.Nat],
        [],
      ),
    'icrc10_supported_standards' : IDL.Func(
        [],
        [IDL.Vec(SupportedStandard)],
        ['query'],
      ),
    'icrc28_trusted_origins' : IDL.Func([], [Icrc28TrustedOriginsResponse], []),
    'reset_strategy' : IDL.Func([IDL.Nat16], [], []),
    'user_strategies' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(UserStrategyResponse)],
        [],
      ),
    'withdraw' : IDL.Func([WithdrawArgs], [WithdrawResponse], []),
  });
};
