/* eslint-disable @typescript-eslint/no-explicit-any */
import {
  _SERVICE as VaultType,
  DepositResponse,
  StrategyResponse,
} from "./idl/vault.ts";
import { _SERVICE as Kong, PoolsReply, PoolsResult } from "./idl/kong_backend.ts";
import {
  Actor,
  ActorSubclass,
  Agent as DfinityAgent,
  HttpAgent,
} from "@dfinity/agent";
import { idlFactory } from "./idl/vault_idl.ts";
import { idlFactory as KongIDL } from "./idl/kong_backend_idl.ts";
import { IDL } from "@dfinity/candid";
import * as Agent from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { _SERVICE as ledgerService, ApproveArgs } from "./idl/ledger.ts";
import { idlFactory as ledger_idl } from "./idl/ledger_idl.ts";

export const alfaACanister = "ownab-uaaaa-aaaap-qp2na-cai";
export const kongCanister = "2ipq2-uqaaa-aaaar-qailq-cai";

export class StrategiesService {
  private actor: ActorSubclass<VaultType>;

  protected constructor(
    actor: ActorSubclass<VaultType>,
  ) {
    this.actor = actor;
  }

  static async build(agent: DfinityAgent): Promise<StrategiesService> {
    const actor = await getTypedActor<VaultType>(
      alfaACanister,
      agent,
      idlFactory
    );
    return new StrategiesService(actor);
  }

  public static async get_strategies(): Promise<Array<StrategyResponse>> {
    const annonymousAgent = await HttpAgent.create({ host: "https://ic0.app" });
    const anonymousActor = await getTypedActor<VaultType>(
      alfaACanister,
      annonymousAgent,
      idlFactory
    );
    return anonymousActor.get_strategies();
  }

  //todo accept identity-kit actor
  public async withdraw(
    strategy_id: number,
    ledger: string,
    amount: bigint,
  ) {
    return this.actor.withdraw({
      strategy_id,
      ledger: Principal.fromText(ledger),
      amount,
    });
  }

  //todo accept identity-kit actor
  public async accept_investment(
    strategy_id: number,
    ledger: string,
    amount: bigint,
    agent: DfinityAgent
  ): Promise<DepositResponse> {
    const ledgerActor = await getTypedActor<ledgerService>(
      ledger,
      agent,
      ledger_idl
    );
    await checkAndApproveTokens(amount, ledgerActor);
    return this.actor.accept_investment({
      strategy_id,
      ledger: Principal.fromText(ledger),
      amount,
    });
  }

  public async get_user_data() {}

  public static async get_pool_data(
    pools_symbols: Array<string>
  ): Promise<any> {
    const annonymousAgent = await HttpAgent.create({ host: "https://ic0.app" });
    const anonymousActor = await getTypedActor<Kong>(
      kongCanister,
      annonymousAgent,
      KongIDL
    );
    return anonymousActor
      .pools([])
      .then((result: PoolsResult) => {
        const pools: PoolsReply = (result as any).Ok;
        return pools.pools.filter((pool) =>
          pools_symbols.includes(pool.symbol)
        );
      })
      .catch((e) => {
        console.error(e);
      });
  }

  public static async get_user_strategies(
    user: Principal
  ): Promise<Array<any>> {
    const actor = await getTypedActor<VaultType>(
      alfaACanister,
      await HttpAgent.create({ host: "https://ic0.app" }),
      idlFactory
    );
    // const balance = await actor.user_balance_all(user);
    // console.log("balance", balance);
    const kongActor = await getTypedActor<Kong>(
      kongCanister,
      await HttpAgent.create({ host: "https://ic0.app" }),
      KongIDL
    );
    // Get user strategies and balances
    return Promise.all([
      actor.user_strategies(user),
      kongActor.user_balances(alfaACanister),
    ])
      .then(([userStrategies, userBalances]) => {
        console.log("User strategies:", userStrategies);
        console.log("User balances:", userBalances);

        const selectedUserBalances: any = [];

        (userBalances as any).Ok.forEach((balance: any) => {
          userStrategies.forEach((strategy) => {
            const lpPosition = balance.LP;
            console.log("lpPosition", lpPosition)
            // Select balances that match user's strategy and have non-zero total shares
            if (
              strategy.strategy_current_pool === lpPosition.symbol &&
              strategy.total_shares > 0n
            ) {
              // Calculate user's share in the strategy
              const userShare =
                Number(strategy.user_shares) / Number(strategy.total_shares);
              const userUsdBalance = lpPosition.usd_balance * userShare;

              selectedUserBalances.push({
                name: strategy.strategy_name,
                strategy_id: strategy.strategy_id,
                symbol: lpPosition.symbol,
                user_shares: strategy.user_shares,
                total_shares: strategy.total_shares,
                share_percentage: userShare * 100,
                symbol_0: lpPosition.symbol_0,
                symbol_1: lpPosition.symbol_1,
                amount_0: lpPosition.amount_0,
                amount_1: lpPosition.amount_1,
                usd_balance: userUsdBalance,
              });
            }
          });
        });
        console.log("Selected user balances:", selectedUserBalances);
        return selectedUserBalances;
      })
      .catch((error) => {
        console.error("Error getting user strategies:", error);
        throw error;
      });
  }

  public static async get_user_balances(principal: string): Promise<
    Array<{
      LP: {
        ts: bigint;
        usd_balance: number;
        balance: number;
        name: string;
        amount_0: number;
        amount_1: number;
        address_0: string;
        address_1: string;
        symbol_0: string;
        symbol_1: string;
        usd_amount_0: number;
        usd_amount_1: number;
        chain_0: string;
        chain_1: string;
        symbol: string;
      };
    }>
  > {
    const annonymousAgent = await HttpAgent.create({ host: "https://ic0.app" });
    const anonymousActor = await getTypedActor<Kong>(
      kongCanister,
      annonymousAgent,
      KongIDL
    );
    return anonymousActor.user_balances(principal).then((result) => {
      const error = (result as { Err: string }).Err;
      if (error) throw new Error(error);
      else return (result as any).Ok;
    });
  }
}

export async function getTypedActor<T>(
  imCanisterId: string,
  agent: DfinityAgent,
  idl: IDL.InterfaceFactory
): Promise<Agent.ActorSubclass<T>> {
  return Actor.createActor(idl, { agent, canisterId: imCanisterId });
}

export const checkAndApproveTokens = async (
  amount: bigint,
  ledgerActor: ActorSubclass<ledgerService>
) => {
  const approveArgs: ApproveArgs = {
    amount: BigInt(10) * amount,
    spender: {
      owner: Principal.fromText(alfaACanister),
      subaccount: [],
    },
    fee: [],
    memo: [],
    from_subaccount: [],
    created_at_time: [],
    expected_allowance: [],
    expires_at: [],
  };

  // Approve tokens
  const approveResponse = await ledgerActor.icrc2_approve(approveArgs);
  console.log("IRC2 approve:", approveResponse);
};
