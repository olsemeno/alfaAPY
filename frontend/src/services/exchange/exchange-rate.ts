import * as Agent from "@dfinity/agent";
import BigNumber from "bignumber.js";

import { actorBuilder } from "../actors";
import { idlFactory as IDL } from "./idl/exchange-rate-idl";
import { _SERVICE as Service, ExchangeRate__1 } from "./idl/exchange-rate";
import { idlFactory as IDL_ICRC1_NODE } from "./idl/node-index-idl";
import { _SERVICE as ServiceNode } from "./idl/node-index";
import { idlFactory as IDL_TOKEN } from "./idl/token-idl";
import { _SERVICE as ServiceToken, PublicTokenOverview } from "./idl/Token";

const EXCHANGE_RATE_CANISTER = "2ixw4-taaaa-aaaag-qcpdq-cai";
type NumberType = string | number | bigint | BigNumber;

export class ExchangeRateService {
  private exchangeRateActor: Agent.ActorSubclass<Service>;
  private exchangeTokenNodeActor: Agent.ActorSubclass<ServiceNode>;
  private ICP2USD: BigNumber = new BigNumber(0);

  static NODE_CANISTER = "ggzvv-5qaaa-aaaag-qck7a-cai";

  constructor() {
    this.exchangeRateActor = actorBuilder<Service>({
      canisterId: EXCHANGE_RATE_CANISTER,
      factory: IDL,
    });
    this.exchangeTokenNodeActor = actorBuilder<ServiceNode>({
      canisterId: ExchangeRateService.NODE_CANISTER,
      factory: IDL_ICRC1_NODE,
    });
  }

  getICP2USD(): BigNumber {
    return this.ICP2USD;
  }

  getNodeCanister(): string {
    return ExchangeRateService.NODE_CANISTER;
  }

  async cacheUsdIcpRate() {
    const result = await this.getExchangeRate("f_USD-c_ICP");
    this.ICP2USD = this.parseTokenAmount(result.rate, result.decimals);
  }

  async getAllIcpTokens() {
    const responseJson = await fetch("https://web2.icptokens.net/api/tokens");
    if (!responseJson.ok) return undefined;
    const tokens: Array<{
      canister_id: string;
      metrics: { price: { usd: string }; change: { "24h": { usd: string } } };
    }> = await responseJson.json();
    return tokens.map((el) => ({
      address: el.canister_id,
      price: Number(el.metrics.price.usd),
      priceDayChange: Number(el.metrics.change["24h"].usd),
    }));
  }

  async usdPriceForICRC1(ledger: string): Promise<
    | {
        value: BigNumber;
        dayChangePercent?: string;
        dayChangePercentPositive?: boolean;
      }
    | undefined
  > {
    try {
      const token = (await this.getAllIcpTokens())?.find(
        (t) => t.address === ledger
      );

      if (!token) {
        const tokenStorageCanister = await this.getTokenStorageCanister(ledger);
        if (!tokenStorageCanister) {
          return undefined;
        }
        const actorStorage = actorBuilder<ServiceToken>({
          canisterId: tokenStorageCanister,
          factory: IDL_TOKEN,
        });

        try {
          const result: PublicTokenOverview = await actorStorage.getToken(
            ledger
          );
          if (result.priceUSD === undefined) return undefined;
          return {
            value: BigNumber(result.priceUSD),
          };
          // eslint-disable-next-line @typescript-eslint/no-unused-vars
        } catch (e) {
          return undefined;
        }
      }

      return {
        value: BigNumber(token.price),
        dayChangePercent: BigNumber(token.priceDayChange).abs().toFixed(2),
        dayChangePercentPositive: BigNumber(token.priceDayChange).gte(0),
      };
    } catch (e) {
      console.error("usdPriceForICRC1 error: ", e);
      return undefined;
    }
  }

  private async getTokenStorageCanister(
    ledger: string
  ): Promise<string | undefined> {
    return this.exchangeTokenNodeActor.tokenStorage(ledger).then((result) => {
      return result.length > 0 ? result[0] : undefined;
    });
  }

  private async getExchangeRate(pair: string): Promise<ExchangeRate__1> {
    return this.exchangeRateActor.get_exchange_rate(pair);
  }

  parseTokenAmount(
    amount: NumberType | null | undefined,
    decimals: number | bigint = 8
  ): BigNumber {
    if (amount !== 0 && !amount) return new BigNumber(0);

    if (typeof amount === "bigint") amount = Number(amount);

    if (typeof decimals === "bigint") decimals = Number(decimals);

    if (Number.isNaN(Number(amount))) return new BigNumber(String(amount));

    return new BigNumber(String(amount)).dividedBy(10 ** Number(decimals));
  }
}

export const exchangeRateService = new ExchangeRateService();
