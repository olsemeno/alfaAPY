import * as Agent from "@dfinity/agent";
import { Actor, Agent as DfinityAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import BigNumber from "bignumber.js";
import { idlFactory as SwapPoolIDL } from "../idl/SwapPool";
import { SourceInputCalculatorIcpSwap } from "./icp-swap-calculator";
import { IcpSwapQuoteImpl } from "./icp-swap-quote-impl";
import {
  icpSwapService,
  SWAP_FACTORY_CANISTER,
} from "../../icpswap/service/icpswap-service";
import { Quote } from "../../quote";
import { Shroff } from "../../shroff";
import { ShroffAbstract } from "../../shroff/shroff-abstract";

import {
  DepositError,
  LiquidityError,
  ServiceUnavailableError,
  SlippageSwapError,
  SwapError,
  WithdrawError,
} from "../../errors/types";
import { SwapName } from "../../types/enums";
import { PoolData } from "../idl/SwapFactory.d";
import {
  _SERVICE as SwapPool,
  DepositArgs,
  Result,
  SwapArgs,
  WithdrawArgs,
} from "../idl/SwapPool.d";
import { SourceInputCalculator } from "../../calculator/calculator";
import { TRIM_ZEROS } from "../../../token/constants";
import { icrc1OracleService, transferICRC1 } from "../../../token/icrc1";
import { ICRC1TypeOracle, TransferArg } from "../../../../idl";
import { actorBuilder } from "../../../actors";
import { hasOwnProperty } from "../../../utils";
import { exchangeRateService } from "../../../exchange/exchange-rate";
import { Account, SubAccount } from "@dfinity/ledger-icp";
import { ContactSupportError } from "../../errors/types/contact-support-error";

export class ShroffIcpSwapImpl extends ShroffAbstract {
  private readonly zeroForOne: boolean;
  private readonly poolData: PoolData;
  protected readonly swapPoolActorAuthenticated: Agent.ActorSubclass<SwapPool>;
  protected readonly swapPoolActorNotAuthenticated: Agent.ActorSubclass<SwapPool>;

  constructor(
    poolData: PoolData,
    zeroForOne: boolean,
    source: ICRC1TypeOracle,
    target: ICRC1TypeOracle,
    agent: DfinityAgent,
    principal: string
  ) {
    super(source, target, principal);
    this.poolData = poolData;
    this.zeroForOne = zeroForOne;
    this.swapPoolActorAuthenticated = actorBuilder<SwapPool>({
      canisterId: poolData.canisterId,
      factory: SwapPoolIDL,
      agent,
    });
    this.swapPoolActorNotAuthenticated = actorBuilder<SwapPool>({
      canisterId: poolData.canisterId,
      factory: SwapPoolIDL,
    });
  }

  private async getAuthenticatedAgent() {
    try {
      const agent = await Actor.agentOf(this.swapPoolActorAuthenticated);
      if (!agent) throw new Error("Not authenticated");
      return agent;
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
    } catch (e) {
      throw new Error("Not authenticated");
    }
  }

  getSwapName(): SwapName {
    return SwapName.ICPSwap;
  }

  setQuote(quote: Quote) {
    this.requestedQuote = quote;
  }

  static getStaticTargets(): string[] {
    return [exchangeRateService.getNodeCanister(), SWAP_FACTORY_CANISTER];
  }

  getTargets(): string[] {
    return [
      this.source.ledger,
      this.target.ledger,
      this.poolData.canisterId.toText(),
      ...ShroffAbstract.getStaticTargets(),
    ];
  }

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

    const slippage = await this.getSlippage();

    const args: SwapArgs = {
      amountIn: preCalculation.getSourceSwapAmount().toString(),
      zeroForOne: this.zeroForOne,
      amountOutMinimum: slippage.toString(),
    };

    const quotePromise = this.swapPoolActorNotAuthenticated.quote(
      args
    ) as Promise<Result>;

    const [targetUSDPrice, sourceUSDPrice, quote] = await Promise.all([
      targetUSDPricePromise,
      sourceUSDPricePromise,
      quotePromise,
    ]);

    if (hasOwnProperty(quote, "ok")) {
      this.requestedQuote = new IcpSwapQuoteImpl(
        amount,
        preCalculation,
        quote.ok as bigint,
        this.source,
        this.target,
        slippage,
        targetUSDPrice?.value,
        sourceUSDPrice?.value
      );
      return this.requestedQuote;
    }

    throw new LiquidityError();
  }

  async swap(): Promise<void> {
    if (!this.requestedQuote) {
      throw new Error("Request quote first");
    }
    // const icrc2supported = await this.icrc2supported();

    let icrcTransferId;

    // if (icrc2supported) {
    //   // TODO ask Oleksii about root canister
    //   const kong = "2ipq2-uqaaa-aaaar-qailq-cai"
    //   await this.icrc2approve(kong);
    //   console.log("ICRC2 approve response", JSON.stringify(icrcTransferId));
    // } else {
      try {
        await this.transferToSwap(await this.getAuthenticatedAgent());
        console.log("ICRC21 transfer response", JSON.stringify(icrcTransferId));
      } catch (e) {
        throw new ContactSupportError("Deposit error: " + e);
      }
    // }

    console.debug("Transfer to swap done");
    await this.deposit();
    console.debug("Deposit done");
    await this.swapOnExchange();
    console.debug("Swap done");
    await this.withdraw();
    console.debug("Withdraw done");
    // await this.transferFromSwap(TREASURY_CANISTER_ID);
    console.debug("Transfer to NFID done");
  }

  async getSwapAccount(): Promise<Account> {
    return {
      subaccount: [
        SubAccount.fromPrincipal(Principal.from(this.principal)).toUint8Array(),
      ],
      owner: this.poolData.canisterId,
    };
  }

  protected async deposit(): Promise<bigint> {
    if (!this.requestedQuote) {
      throw new Error("Quote is required");
    }

    try {
      const amountDecimals = this.requestedQuote
        .getSourceSwapAmount()
        .plus(Number(this.source.fee));
      const args: DepositArgs = {
        fee: this.source.fee,
        token: this.source.ledger,
        amount: BigInt(amountDecimals.toFixed()),
      };
      console.debug("Amount decimals: " + BigInt(amountDecimals.toFixed()));

      const result = await this.swapPoolActorAuthenticated.deposit(args);

      if (hasOwnProperty(result, "ok")) {
        const id = (result as unknown as { ok: bigint }).ok;
        return id;
      }
      const err = (result as unknown as { err: string }).err;
      console.error("Deposit error: " + JSON.stringify(err));
      throw new DepositError(JSON.stringify(err));
    } catch (e) {
      console.error("Deposit error: " + e);
      throw new DepositError("Deposit error: " + e);
    }
  }

  protected async transfer(): Promise<void> {
    await this.getAuthenticatedAgent();
    if (!this.requestedQuote) {
      throw new Error("Quote is required");
    }
  }

  protected async transferFromSwap(principal: string) {
    try {
      const amountDecimals = this.requestedQuote!.getWidgetFeeAmount();

      const transferArgs: TransferArg = {
        amount: amountDecimals,
        created_at_time: [],
        fee: [],
        from_subaccount: [],
        memo: [],
        to: {
          subaccount: [],
          owner: Principal.fromText(principal),
        },
      };

      const result = await transferICRC1(
        await this.getAuthenticatedAgent(),
        this.source.ledger,
        transferArgs
      );
      if (hasOwnProperty(result, "Ok")) {
        const id = result.Ok as bigint;
        return id;
      }
      console.error("NFID transfer error: " + JSON.stringify(result.Err));
      throw new WithdrawError(JSON.stringify(result.Err));
    } catch (e) {
      console.error("NFID transfer error: " + e);
      throw new WithdrawError("NFID transfer error: " + e);
    }
  }

  protected async swapOnExchange(): Promise<bigint> {
    try {
      const args: SwapArgs = {
        amountIn: this.requestedQuote!.getSourceSwapAmount().toFixed(),
        zeroForOne: this.zeroForOne,
        amountOutMinimum: this.requestedQuote!.getTargetAmount()
          .toFixed(this.target.decimals)
          .replace(TRIM_ZEROS, ""),
      };

      console.log("Swap args: " + JSON.stringify(args));

      return this.swapPoolActorAuthenticated.swap(args).then((result) => {
        if (hasOwnProperty(result, "ok")) {
          const response = (result as { ok: bigint }).ok;
          return response;
        }

        const err = (result as unknown as { err: { InternalError?: string } })
          .err;

        console.error("Swap on exchange error: " + JSON.stringify(err));

        if (
          hasOwnProperty(err, "InternalError") &&
          (err.InternalError as string).toLocaleLowerCase().includes("slippage")
        ) {
          throw new SlippageSwapError(JSON.stringify(err));
        }
        throw new SwapError(JSON.stringify(err));
      });
    } catch (e) {
      console.error("Swap error: " + e);
      throw new SwapError("Swap error: " + e);
    }
  }

  protected async withdraw(): Promise<bigint> {
    try {
      const args: WithdrawArgs = {
        amount: BigInt(
          this.requestedQuote!.getTargetAmount()
            .toFixed(this.target.decimals)
            .replace(TRIM_ZEROS, "")
        ),
        token: this.target.ledger,
        fee: this.target.fee,
      };
      // console.debug("Withdraw args: " + JSON.stringify(args));
      return this.swapPoolActorAuthenticated.withdraw(args).then((result) => {
        if (hasOwnProperty(result, "ok")) {
          const id = (result as unknown as { ok: bigint }).ok;
          return id;
        }
        const err = (result as unknown as { err: string }).err;
        console.error("Withdraw error: " + JSON.stringify(err));
        throw new WithdrawError(JSON.stringify(err));
      });
    } catch (e) {
      console.error("Withdraw error: " + e);
      throw new WithdrawError("Withdraw error: " + e);
    }
  }

  protected getCalculator(amountInDecimals: BigNumber): SourceInputCalculator {
    return new SourceInputCalculatorIcpSwap(
      BigInt(amountInDecimals.toFixed()),
      this.source.fee
    );
  }
}

export class IcpSwapShroffBuilder {
  private source: string | undefined;
  private target: string | undefined;
  protected poolData: PoolData | undefined;
  protected sourceOracle: ICRC1TypeOracle | undefined;
  protected targetOracle: ICRC1TypeOracle | undefined;
  protected zeroForOne: boolean | undefined;

  public withSource(source: string): IcpSwapShroffBuilder {
    this.source = source;
    return this;
  }

  public withTarget(target: string): IcpSwapShroffBuilder {
    this.target = target;
    return this;
  }

  public async build(agent: DfinityAgent, principal: string): Promise<Shroff> {
    if (!this.source) {
      throw new Error("Source is required");
    }

    if (!this.target) {
      throw new Error("Target is required");
    }

    try {
      const [poolData, icrc1canisters]: [PoolData, ICRC1TypeOracle[]] =
        await Promise.all([
          icpSwapService.getPoolFactory(this.source, this.target),
          icrc1OracleService.requestNetworkForCanisters(),
        ]);

      this.poolData = poolData;

      const st: ICRC1TypeOracle[] = icrc1canisters.filter(
        (icrc1) => icrc1.ledger === this.source || icrc1.ledger === this.target
      );

      this.sourceOracle = st.find((icrc1) => icrc1.ledger === this.source);
      this.targetOracle = st.find((icrc1) => icrc1.ledger === this.target);

      if (!this.sourceOracle || !this.targetOracle) {
        throw new Error("ICRC1 not found");
      }

      this.zeroForOne = this.poolData.token0.address === this.source;

      return this.buildShroff(agent, principal);
    } catch (e) {
      if (e instanceof LiquidityError) {
        throw e;
      }
      throw new ServiceUnavailableError();
    }
  }

  protected buildShroff(agent: DfinityAgent, principal: string): Shroff {
    return new ShroffIcpSwapImpl(
      this.poolData!,
      this.zeroForOne!,
      this.sourceOracle!,
      this.targetOracle!,
      agent,
      principal
    );
  }
}
