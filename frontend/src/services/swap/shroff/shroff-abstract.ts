import { Agent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import BigNumber from "bignumber.js";
import { DepositError, WithdrawError } from "../errors/types";
import { SWAP_FACTORY_CANISTER } from "../icpswap/service/icpswap-service";
import { Quote } from "../quote";
import { Shroff } from "..//shroff";

import { SwapName } from "../types/enums";
import { SourceInputCalculator } from "../calculator/calculator";
import { transferICRC1 } from "../../token/icrc1";
import { hasOwnProperty } from "../../utils";
import { ICRC1TypeOracle, TransferArg } from "../../../idl";
import { exchangeRateService } from "../../exchange/exchange-rate";
import { actorBuilder } from "../../actors";
import { idlFactory as icrc1IDL } from "../../../idl/icrc1_idl";
import {
  _SERVICE as ICRC1ServiceIDL,
  Account,
  ApproveArgs,
} from "../../../idl/icrc1.d";
import { TRIM_ZEROS } from "../../token/constants";
import { ContactSupportError } from "../errors/types/contact-support-error";

export abstract class ShroffAbstract implements Shroff {
  protected readonly source: ICRC1TypeOracle;
  protected readonly target: ICRC1TypeOracle;
  protected requestedQuote: Quote | undefined;
  protected slippage: number = 2;
  protected principal: string;

  protected constructor(
    source: ICRC1TypeOracle,
    target: ICRC1TypeOracle,
    principal: string,
    slippage?: number
  ) {
    this.source = source;
    this.target = target;
    this.principal = principal;
    if (slippage) this.slippage = slippage;
  }

  abstract getSwapName(): SwapName;

  abstract getTargets(): string[];

  setQuote(quote: Quote) {
    this.requestedQuote = quote;
  }

  static getStaticTargets(): string[] {
    return [exchangeRateService.getNodeCanister(), SWAP_FACTORY_CANISTER];
  }

  abstract getQuote(amount: string): Promise<Quote>;

  abstract swap(): Promise<void>;

  protected async icrc2approve(rootCanister: string): Promise<bigint> {
    try {
      const actorICRC2 = this.getICRCActor();

      const spender: Account = {
        owner: Principal.fromText(rootCanister),
        subaccount: [],
      };

      const icrc2_approve_args: ApproveArgs = {
        from_subaccount: [],
        spender,
        fee: [],
        memo: [],
        amount: BigInt(
          this.requestedQuote!.getSourceSwapAmount()
            .plus(Number(this.source.fee))
            .toFixed(this.source.decimals)
            .replace(TRIM_ZEROS, "")
        ),
        created_at_time: [],
        expected_allowance: [],
        expires_at: [
          {
            timestamp_nanos: BigInt(Date.now() * 1_000_000 + 60_000_000_000),
          },
        ],
      };

      const icrc2approve = await actorICRC2.icrc2_approve(icrc2_approve_args);

      if (hasOwnProperty(icrc2approve, "Err")) {
        throw new ContactSupportError(JSON.stringify(icrc2approve.Err));
      }

      return BigInt(icrc2approve.Ok);
    } catch (e) {
      console.error("Deposit error: " + e);
      throw new ContactSupportError("Deposit error: " + e);
    }
  }

  protected async icrc2supported(): Promise<boolean> {
    const actorICRC2 = this.getICRCActor();

    return actorICRC2.icrc1_supported_standards().then((res) => {
      return res
        .map((standard) => standard.name)
        .some((name) => name === "ICRC-2");
    });
  }

  protected getICRCActor() {
    return actorBuilder<ICRC1ServiceIDL>({
      canisterId: this.source.ledger,
      factory: icrc1IDL,
    });
  }

  protected async transferToSwap(agent: Agent) {
    try {
      const amountDecimals = this.requestedQuote!.getTransferToSwapAmount();

      console.debug("Amount decimals: " + BigInt(amountDecimals.toFixed()));

      const transferArgs: TransferArg = {
        amount: BigInt(amountDecimals.toFixed()),
        created_at_time: [],
        fee: [],
        from_subaccount: [],
        memo: [],
        to: await this.getSwapAccount(),
      };

      const result = await transferICRC1(
        agent,
        this.source.ledger,
        transferArgs
      );
      if (hasOwnProperty(result, "Ok")) {
        const id = result.Ok as bigint;
        return id;
      }
      console.error(
        "Transfer to " + this.getSwapName() + ": " + JSON.stringify(result.Err)
      );
      throw new DepositError(JSON.stringify(result.Err));
    } catch (e) {
      console.error("Deposit error: " + e);
      throw new DepositError(e as Error);
    }
  }

  abstract getSwapAccount(): Promise<Account>;

  protected async transferFromSwap(principal: string, agent: Agent) {
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
        agent,
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

  protected getAmountInDecimals(amount: string): BigNumber {
    return new BigNumber(amount).multipliedBy(10 ** this.source.decimals);
  }

  protected abstract getCalculator(
    amountInDecimals: BigNumber
  ): SourceInputCalculator;

  protected getSlippage(): number {
    return this.slippage;
  }
}
