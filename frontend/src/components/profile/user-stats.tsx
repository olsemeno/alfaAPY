import { Card } from "../ui";
import { useAuth } from "@nfid/identitykit/react";
import { useEffect, useState } from "react";
import { fetchUserStrategies } from "../../store";
import {
  useDispatch,
  useSelector,
  Status,
} from "../../store";
import { Strategy } from "../../services/strategies/strategy-service";

interface UserStatsData {
  totalTvl: bigint;
  avgApy: bigint;
  avgApyMonth: bigint;
  totalStrategies: number;
  deposited: number;
}

export function UserStats() {
  const { user } = useAuth();
  const dispatch = useDispatch();
  const strategies = useSelector((state) => state.strategies.strategies.data);
  const balances = useSelector((state) => state.strategies.balances);
  const status = useSelector((state) => state.strategies.strategies.status);

  const [userStats, setUserStats] = useState<UserStatsData>({
    totalTvl: 0n,
    avgApy: 0n,
    avgApyMonth: 0n,
    totalStrategies: 0,
    deposited: 0,
  });

  useEffect(() => {
    if (user) {
      dispatch(fetchUserStrategies(user.principal));
    }
  }, [user, dispatch]);

  useEffect(() => {
    if (strategies && balances && status === Status.SUCCEEDED) {
      const stats = strategies.reduce(
        (acc: UserStatsData, strategy: Strategy) => {
          const currentPool = strategy.currentPool;
          if (currentPool) {
            return {
              ...acc,
              totalTvl: acc.totalTvl + strategy.tvl,
              avgApy: (acc.avgApy + strategy.apy),
              avgApyMonth: (acc.avgApyMonth + strategy.apy_month),
              totalStrategies: acc.totalStrategies + 1,
              deposited: strategy.initialDeposit.reduce(
                (acc, [, value]) =>
                  acc + Number(value) / (10 ** strategy.pools[0].token0.decimals) * (strategy.pools[0].price0 ?? 0),
                0
              )
            };
          }
          return acc;
        },
        {
          totalTvl: 0n,
          avgApy: 0n,
          avgApyMonth: 0n,
          totalStrategies: 0,
          deposited: 0,
        }
      );

      // Calculate average APY
      if (stats.totalStrategies > 0) {
        stats.avgApy = stats.avgApy / BigInt(stats.totalStrategies) / 100n;
        stats.avgApyMonth = stats.avgApyMonth / BigInt(stats.totalStrategies) / 100n;
      }

      console.log("stats", stats);

      setUserStats(stats);
    }
  }, [strategies, balances, status]);

  return (
    <Card className="p-[20px]">
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-8">
        <div>
          <h3 className="text-gray-600 text-sm">DEPOSITED</h3>
          <p className="text-2xl font-bold">
            ${(userStats.deposited).toFixed(2)}
          </p>
        </div>
        <div>
          {/* TODO: add monthly yield */}
          <h3 className="text-gray-600 text-sm">MONTHLY YIELD</h3>
          <p className="text-2xl font-bold">
            $0.01 
          </p>
        </div>
        <div>
          <h3 className="text-gray-600 text-sm">DAILY YIELD</h3>
          <p className="text-2xl font-bold">
            $0
          </p>
        </div>
        <div>
          <h3 className="text-gray-600 text-sm">AVG. APY</h3>
          <p className="text-2xl font-bold">
            {Number(userStats.avgApy)}%
          </p>
        </div>
        <div>
          <h3 className="text-gray-600 text-sm">TVL</h3>
          <p className="text-2xl font-bold">
            ${(Number(userStats.totalTvl) / 10**8).toFixed(2)}
          </p>
        </div>
        <div>
          <h3 className="text-gray-600 text-sm">STRATEGIES</h3>
          <p className="text-2xl font-bold">
            {userStats.totalStrategies}
          </p>
        </div>
      </div>
    </Card>
  );
}
