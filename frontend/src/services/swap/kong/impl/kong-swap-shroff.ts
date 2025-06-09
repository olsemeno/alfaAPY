/* eslint-disable @typescript-eslint/no-explicit-any */
import * as Agent from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import BigNumber from "bignumber.js";
import {
  Account,
} from "../idl/icrc1.d";
import { idlFactory as KongIDL } from "../idl/kong_backend";
import { _SERVICE, PoolsResult, SwapArgs } from "../idl/kong_backend.d";

import { ContactSupportError } from "../../errors/types/contact-support-error";
import { Quote } from "../../quote";
import { SwapName } from "../../types/enums";
import { ShroffAbstract } from "../../shroff/shroff-abstract";
import { LiquidityError, ServiceUnavailableError } from "../../errors/types";
import { KongQuoteImpl } from "./kong-quote-impl";
import { KongCalculator } from "./kong-calculator";
import { SourceInputCalculator } from "../../calculator/calculator";
import { Shroff } from "../../shroff";
import { actorBuilder } from "../../../actors";
import { ICRC1TypeOracle } from "../../../../idl";
import { hasOwnProperty } from "../../../utils";
import { TRIM_ZEROS } from "../../../token/constants";
import { icrc1OracleService } from "../../../token/icrc1/service/icrc1-oracle-service";
import { exchangeRateService } from "../../../exchange/exchange-rate";
import { Actor, Agent as DfinityAgent } from "@dfinity/agent";

export const ROOT_CANISTER = "2ipq2-uqaaa-aaaar-qailq-cai";

export class KongSwapShroffImpl extends ShroffAbstract {
  private authenticatedActor: Agent.ActorSubclass<_SERVICE>;
  private notAuthenticatedActor: Agent.ActorSubclass<_SERVICE>;

  constructor(
    source: ICRC1TypeOracle,
    target: ICRC1TypeOracle,
    agent: DfinityAgent,
    principal: string
  ) {
    super(source, target, principal);
    this.authenticatedActor = actorBuilder<_SERVICE>({
      canisterId: ROOT_CANISTER,
      factory: KongIDL,
      agent,
    });
    this.notAuthenticatedActor = actorBuilder<_SERVICE>({
      canisterId: ROOT_CANISTER,
      factory: KongIDL,
    });
  }

  protected getCalculator(amountInDecimals: BigNumber): SourceInputCalculator {
    return new KongCalculator(
      BigInt(amountInDecimals.toFixed()),
      this.source.fee
    );
  }

  getSwapName(): SwapName {
    return SwapName.Kongswap;
  }

  getTargets(): string[] {
    return [
      this.source.ledger,
      this.target.ledger,
      ROOT_CANISTER,
      ...ShroffAbstract.getStaticTargets(),
    ];
  }

  //TODO improve
  async getQuote(amount: string): Promise<Quote> {
    const amountInDecimals = this.getAmountInDecimals(amount);
    console.debug("Amount in decimals: " + amountInDecimals.toFixed());
    const preCalculation = this.getCalculator(amountInDecimals);
    const targetUSDPricePromise = exchangeRateService.usdPriceForICRC1(
      this.target.ledger
    );
    const sourceUSDPricePromise = exchangeRateService.usdPriceForICRC1(
      this.source.ledger
    );
    const quotePromise = this.getQuotePromise(preCalculation);
    const [targetUSDPrice, sourceUSDPrice, quote] = await Promise.all([
      targetUSDPricePromise,
      sourceUSDPricePromise,
      quotePromise,
    ]);

    if (hasOwnProperty(quote, "Ok")) {
      this.requestedQuote = new KongQuoteImpl(
        amount,
        preCalculation,
        quote.Ok.receive_amount,
        this.source,
        this.target,
        await this.getSlippage(),
        quote.Ok,
        targetUSDPrice?.value,
        sourceUSDPrice?.value
      );
      return this.requestedQuote;
    }

    throw new LiquidityError();
  }

  async swap(): Promise<void> {
    if (!this.requestedQuote) {
      throw new Error("Quote not set");
    }

    try {
      const icrc2supported = await this.icrc2supported();

      let icrcTransferId;

      if (icrc2supported) {
        icrcTransferId = await this.icrc2approve(ROOT_CANISTER);
        console.log("ICRC2 approve response", JSON.stringify(icrcTransferId));
      } else {
        try {
          icrcTransferId = await this.transferToSwap(
            Actor.agentOf(this.authenticatedActor)!
          );
          console.log(
            "ICRC21 transfer response",
            JSON.stringify(icrcTransferId)
          );
        } catch (e) {
          throw new ContactSupportError("Deposit error: " + e);
        }
      }

      const slippage = await this.getSlippage();

      const args: SwapArgs = {
        receive_token: this.target.symbol,
        max_slippage: [slippage],
        pay_amount: BigInt(
          this.requestedQuote
            .getSourceSwapAmount()
            .toFixed(this.source.decimals)
            .replace(TRIM_ZEROS, "")
        ),
        referred_by: [],
        receive_amount: [
          BigInt(
            this.requestedQuote
              .getTargetAmount()
              .toFixed(this.target.decimals)
              .replace(TRIM_ZEROS, "")
          ),
        ],
        receive_address: [],
        pay_token: this.source.symbol,
        pay_tx_id: icrc2supported ? [] : [{ BlockIndex: icrcTransferId }],
      };
      console.debug("Swap args", JSON.stringify(args));

      await this.swapInternal(args);
    } catch (e) {
      console.error("Swap error:", e);
      throw e;
    }
  }

  protected async swapInternal(args: SwapArgs): Promise<void> {
    try {
      const resp = await this.authenticatedActor.swap(args);
      console.log("Swap response", JSON.stringify(resp));

      if (hasOwnProperty(resp, "Err")) {
        throw new ContactSupportError(JSON.stringify(resp.Err));
      }
    } catch (e) {
      throw new ContactSupportError("Swap error: " + e);
    }
  }

  protected getQuotePromise(
    sourceCalculator: SourceInputCalculator
  ): Promise<any> {
    return this.notAuthenticatedActor.swap_amounts(
      this.source.symbol,
      sourceCalculator.getSourceSwapAmount(),
      this.target.symbol
    );
  }

  async getPools(source: string, target: string): Promise<PoolsResult[]> {
    const pair1 = await this.notAuthenticatedActor.pools([
      `${source}_${target}`,
    ]);
    const pair2 = await this.notAuthenticatedActor.pools([
      `${target}_${source}`,
    ]);

    return [pair1, pair2];
  }

  async getSwapAccount(): Promise<Account> {
    return {
      subaccount: [],
      owner: Principal.fromText(ROOT_CANISTER),
    };
  }
}

export class KongShroffBuilder {
  private source: string | undefined;
  private target: string | undefined;
  protected sourceOracle: ICRC1TypeOracle | undefined;
  protected targetOracle: ICRC1TypeOracle | undefined;

  public withSource(source: string): KongShroffBuilder {
    this.source = source;
    return this;
  }

  public withTarget(target: string): KongShroffBuilder {
    this.target = target;
    return this;
  }

  //todo generify
  public async build(agent: DfinityAgent, principal: string): Promise<Shroff> {
    if (!this.source) {
      throw new Error("Source is required");
    }

    if (!this.target) {
      throw new Error("Target is required");
    }

    try {
      const [icrc1canisters]: [ICRC1TypeOracle[]] = await Promise.all([
        icrc1OracleService.getICRC1Canisters(),
      ]);

      const st: ICRC1TypeOracle[] = icrc1canisters.filter(
        (icrc1) => icrc1.ledger === this.source || icrc1.ledger === this.target
      );

      this.sourceOracle = st.find((icrc1) => icrc1.ledger === this.source);
      this.targetOracle = st.find((icrc1) => icrc1.ledger === this.target);

      if (!this.sourceOracle || !this.targetOracle) {
        throw new Error("ICRC1 not found");
      }
      const buildShroff = this.buildShroff(agent, principal);

      const pools = await buildShroff.getPools(
        this.sourceOracle.symbol,
        this.targetOracle.symbol
      );

      if (!pools.some((pool) => "Ok" in pool && pool.Ok.pools.length > 0)) {
        throw new LiquidityError();
      }

      return buildShroff;
    } catch (e) {
      if (e instanceof LiquidityError) {
        throw e;
      }
      throw new ServiceUnavailableError();
    }
  }

  protected buildShroff(
    agent: DfinityAgent,
    principal: string
  ): KongSwapShroffImpl {
    return new KongSwapShroffImpl(
      this.sourceOracle!,
      this.targetOracle!,
      agent,
      principal
    );
  }
}
