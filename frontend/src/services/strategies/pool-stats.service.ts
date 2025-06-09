import {
  _SERVICE,
  Pool,
  PoolMetrics,
} from "../../idl/pool_stats";
import { idlFactory } from "../../idl/pool_stats_idl";
import { getAnonActor } from "../utils";
import { POOL_STATS_CANISTER_ID } from "../../constants";
export class PoolStatsService {
  public async get_pools(): Promise<Array<Pool>> {
    const anonymousActor = await getAnonActor<_SERVICE>(
      POOL_STATS_CANISTER_ID,
      idlFactory
    );
    return await anonymousActor.get_pools();
  }

  public async get_all_pool_metrics(): Promise<Array<[string, PoolMetrics]>> {
    const pools = await this.get_pools();

    const poolMetrics = await this.get_pool_metrics(pools.map((p) => p.id));
    return poolMetrics;
  }


  //TODO: add_type_pool_id
  public async get_pool_metrics(
    metricsRequest: Array<string>
  ): Promise<Array<[string, PoolMetrics]>> {
    const anonymousActor = await getAnonActor<_SERVICE>(
      POOL_STATS_CANISTER_ID,
      idlFactory
    );
    return await anonymousActor.get_pool_metrics(metricsRequest);
  }
}

export const poolStatsService = new PoolStatsService();
