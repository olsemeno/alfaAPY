import * as Agent from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { idlFactory as SwapPoolIDL } from "../idl/SwapPool";
import { _SERVICE as SwapPool } from "../idl/SwapPool.d";

import { LiquidityError, ServiceUnavailableError } from "../../errors/types";
import { idlFactory as SwapFactoryIDL } from "../idl/SwapFactory";
import {
  _SERVICE as SwapFactory,
  GetPoolArgs,
  PoolData,
} from "../idl/SwapFactory.d";
import { actorBuilder, agentBaseConfig } from "../../../actors";
import { hasOwnProperty } from "../../../utils";

export const SWAP_FACTORY_CANISTER = "4mmnk-kiaaa-aaaag-qbllq-cai";

class IcpSwapService {
  private poolActor: SwapFactory;

  constructor() {
    this.poolActor = actorBuilder<SwapFactory>({
      canisterId: SWAP_FACTORY_CANISTER, //TODO WIP .env, stage, prod, subnet(?)
      factory: SwapFactoryIDL,
      config: {
        agent: new Agent.HttpAgent({
          ...agentBaseConfig,
        }),
      },
    });
  }

  getPoolFactory(
    sourceCanister: string,
    targetCanister: string
  ): Promise<PoolData> {
    const a: GetPoolArgs = {
      fee: BigInt(3000),
      token0: { address: sourceCanister, standard: "ICRC1" },
      token1: { address: targetCanister, standard: "ICRC1" },
    };
    return this.poolActor.getPool(a).then((pool) => {
      if (hasOwnProperty(pool, "ok")) {
        const data: PoolData = pool.ok as PoolData;
        return data;
      }

      if (hasOwnProperty(pool.err, "InternalError")) {
        throw new ServiceUnavailableError();
      }
      if (hasOwnProperty(pool.err, "UnsupportedToken")) {
        throw new LiquidityError();
      }
      if (hasOwnProperty(pool.err, "InsufficientFunds")) {
        throw new LiquidityError();
      }
      if (hasOwnProperty(pool.err, "CommonError")) {
        throw new LiquidityError();
      }
      console.error("Not able to get pool for pair: " + pool.err);
      throw new ServiceUnavailableError();
    });
  }

  async getBalance(
    swapPoolCanister: string,
    principal: Principal
  ): Promise<{
    balance1: bigint;
    balance2: bigint;
  }> {
    const swapPoolActor = actorBuilder<SwapPool>({
      canisterId: swapPoolCanister, //TODO WIP .env, stage, prod, subnet(?)
      factory: SwapPoolIDL,
      config: {
        agent: new Agent.HttpAgent({
          ...agentBaseConfig,
        }),
      },
    });

    const result = await swapPoolActor.getUserUnusedBalance(principal);

    if (hasOwnProperty(result, "ok")) {
      return result.ok as {
        balance1: bigint;
        balance2: bigint;
      };
    }
    throw new Error("TODO Error handling");
  }
}

export const icpSwapService = new IcpSwapService();
