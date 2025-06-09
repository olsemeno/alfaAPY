import { LiquidityError, ServiceUnavailableError } from "../errors/types";
import { Quote } from "../quote";
import { IcpSwapShroffBuilder } from "../icpswap/impl/shroff-icp-swap-impl";
import { SwapName } from "../types/enums";
import { Shroff } from "../shroff";
import { Agent } from "@dfinity/agent";
import { KongShroffBuilder } from "../kong/impl/kong-swap-shroff";

const PROVIDERS = [
  { builder: new IcpSwapShroffBuilder(), name: SwapName.ICPSwap },
  { builder: new KongShroffBuilder(), name: SwapName.Kongswap },
];

export class SwapService {
  async getSwapProviders(
    source: string,
    target: string,
    agent: Agent,
    principal: string
  ): Promise<Map<SwapName, Shroff | undefined>> {
    let success = false;
    const map = new Map<SwapName, Shroff | undefined>();

    for (let i = 0; i < PROVIDERS.length; i++) {
      const provider = PROVIDERS[i];
      try {
        const buildedProvider = await provider.builder
          .withTarget(target)
          .withSource(source)
          .build(agent, principal);

        map.set(provider.name, buildedProvider);
        success = true;
      } catch (e) {
        map.set(provider.name, undefined);

        if (e instanceof LiquidityError) success = true;
      }
    }

    if (!success) throw new ServiceUnavailableError();

    if (Array.from(map.values()).every((value) => value === undefined))
      throw new LiquidityError();

    return map;
  }

  async getBestShroff(
    providers: Map<SwapName, Shroff | undefined>,
    amount?: string
  ): Promise<Shroff | undefined> {


    console.log("providers", providers);
    if (!amount || !Number(amount)) return;
    const quotesWithShroffs = await Promise.all(
      [...providers.entries()].map(async ([, shroff]) => {
        if (!shroff) return;
        try {
          console.log("shrofffff", shroff);
          const quote = await shroff.getQuote(amount);

          return { shroff, quote };
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        } catch (e) {
          throw new LiquidityError();
        }
      })
    );

    const validQuotes = quotesWithShroffs.filter(
      (item): item is { shroff: Shroff; quote: Quote } => item !== undefined
    );

    const bestShroff = validQuotes.sort(
      (a, b) =>
        Number(b.quote.getTargetAmountPrettified()) -
        Number(a.quote.getTargetAmountPrettified())
    )[0]?.shroff;

    return bestShroff;
  }
}

export const swapService = new SwapService();
