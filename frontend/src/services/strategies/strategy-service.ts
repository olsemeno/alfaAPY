/* eslint-disable @typescript-eslint/no-explicit-any */
import {
  _SERVICE as VaultType, ExchangeId as ExchangeIdResponse
} from "../../idl/vault";
import { idlFactory } from "../../idl/vault_idl";
import { Principal } from "@dfinity/principal";
import { getAnonActor } from "../utils";
import { poolStatsService } from "./pool-stats.service";
import { VAULT_CANISTER_ID } from "../../constants";
import { PoolMetrics } from "../../idl/pool_stats";
import { ExchangeId } from "./enum";
import { icrc1OracleService } from "../token";
import { ICRC1 } from "../../idl/icrc1_oracle";
import { exchangeRateService } from "../exchange/exchange-rate";

export interface Strategy  {
  id : number,
  name : string,
  description : string,
  totalShares : bigint,
  initialDeposit : Array<[Principal, bigint]>,
  userShares : Array<[Principal, bigint]>,
  currentPool : string | undefined,
  totalBalance : bigint,
  pools : Array<StrategyPool>,
  apy: bigint;
  apy_month: bigint;
  apy_week: bigint;
  tvl: bigint;
}



export interface StrategyPool {
  id : string,
  provider : ExchangeId,
  price0: number | undefined,
  price1: number | undefined,
  token0 : ICRC1,
  token1 : ICRC1,
}

export class StrategiesService {
  public async getStrategies(): Promise<Array<Strategy>> {
    const anonymousActor = await getAnonActor<VaultType>(
      VAULT_CANISTER_ID,
      idlFactory
    );
    const strategies = await anonymousActor
      .get_strategies()
      .then((strategies) =>
        strategies.filter((strategy) => strategy.current_pool.length > 0)
      );

    console.log("strategies", strategies);

    const price: Array<{ledger: string, price: number | undefined}> = (await Promise.all(strategies.map(async (strategy) => {
      const token0 = strategy.pools[0].token0.toText();
      const token1 = strategy.pools[0].token1.toText();
      const [price0, price1] = await Promise.all([
        exchangeRateService.usdPriceForICRC1(token0),
        exchangeRateService.usdPriceForICRC1(token1)
      ]);
      return [{ledger: token0, price: price0?.value.toNumber()},{ledger: token1, price: price1?.value.toNumber()}];
    }))).flat();


     const [icrc1Tokens , poolIds, prices] = await Promise.all([
      icrc1OracleService.getICRC1Canisters(),
      strategies.flatMap((strategy) =>
        strategy.pools.map((pool) => pool.id)
      ),
      price
     ]);

     const icrc1TokensMap = new Map(icrc1Tokens.map((token) => [token.ledger, token]));

    const poolStats: [string, PoolMetrics][] =
      await poolStatsService.get_pool_metrics(poolIds);
    console.log("poolStats", poolStats);

    return strategies.map((strategy) => ({
      id: strategy.id,
      name: strategy.name,
      description: strategy.description,
      currentPool: strategy.current_pool[0]?.id,
      totalShares: strategy.total_shares,
      initialDeposit: strategy.initial_deposit,
      userShares: strategy.user_shares,
      totalBalance: strategy.total_balance,
      pools: strategy.pools.map((pool) => ({
        id: pool.id,
        provider: providerResponseToExchangeId(pool.provider),
        token0: icrc1TokensMap.get(pool.token0.toText())!,
        price0: prices.find((price) => price.ledger === pool.token0.toText())?.price,
        token1: icrc1TokensMap.get(pool.token1.toText())!,
        price1: prices.find((price) => price.ledger === pool.token1.toText())?.price,
      })),
      apy:
        poolStats.find((pool) => {
          const currentPool = strategy.current_pool[0]!;
          return pool[0] === currentPool.id;
        })?.[1].apy.year.tokens_apy ?? 0n, //TODO: Fix this
      apy_month:
        poolStats.find((pool) => {
          const currentPool = strategy.current_pool[0]!;
          return pool[0] === currentPool.id;
        })?.[1].apy.month.tokens_apy ?? 0n,
      apy_week:
        poolStats.find((pool) => {
          const currentPool = strategy.current_pool[0]!;
          return pool[0] === currentPool.id;
        })?.[1].apy.week.tokens_apy ?? 0n,

      tvl:
        poolStats.find((pool) => {
          const currentPool = strategy.current_pool[0]!;
          return pool[0] === currentPool.id;
        })?.[1].tvl ?? 0n,
    }));
  }

  public async getUserStrategies(
    user: Principal
  ): Promise<Array<Strategy>> {
    const userStrategies = await this.getStrategies()
    .then((strategies) => strategies.filter((strategy) => strategy.userShares.some(([principal]) => principal.toString() === user.toString())));
    return userStrategies;
  }
}

export const strategiesService = new StrategiesService();


function providerResponseToExchangeId(provider: ExchangeIdResponse): ExchangeId {
  console.log("provider", provider);
  if (hasOwnProperty(provider, "KongSwap")) {
    return ExchangeId.KongSwap;
  } else if (hasOwnProperty(provider, "ICPSwap")) {
    return ExchangeId.ICPSwap;
  }
  throw new Error("Invalid provider");
}


// A `hasOwnProperty` that produces evidence for the typechecker
export function hasOwnProperty<
  X extends Record<string, unknown>,
  Y extends PropertyKey,
>(obj: X, prop: Y): obj is X & Record<Y, unknown> {
  return Object.prototype.hasOwnProperty.call(obj, prop)
}