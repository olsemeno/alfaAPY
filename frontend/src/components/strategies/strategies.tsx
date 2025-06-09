import colors from "tailwindcss/colors";
import SquareLoader from "react-spinners/ClimbingBoxLoader";
import { Button, Card, Input } from "../ui";
import { useTokens } from "../../hooks";
import { useState } from "react";
import { Strategy } from "./strategy";
import { TokensLogos } from "./tokens-logos";
import { getStrategyTokenLogos, getProfitLevel, getProfitColor } from "./utils";
import { motion } from "framer-motion";
import { useAuth } from "@nfid/identitykit/react";
import { UserStats } from "../profile";
import { useStrategies } from "../../hooks/strategies";

interface PlatformStats {
  totalTvl: bigint;
  avgApy: bigint;
  totalStrategies: number;
  deposited: number;
  totalUsers: number;
}

export function Strategies() {
  const { user } = useAuth();
  const { strategies, balances } = useStrategies(user?.principal.toString());
  const { tokens } = useTokens();
  const [selectedStrategy, setSelectedStrategy] = useState<number | undefined>();
  const [searchTerm, setSearchTerm] = useState("");
  const [showUserStrategies, setShowUserStrategies] = useState(false);

  // Calculate platform stats
  const platformStats: PlatformStats | undefined = strategies?.reduce(
    //TODO hui poimi chto
    (acc, strategy) => {
      const currentPool = strategy.currentPool;
      if (currentPool) {
        return {
          ...acc,
          totalTvl: acc.totalTvl + strategy.tvl,
          avgApy: (acc.avgApy + strategy.apy) / BigInt(strategies?.length || 1),
          deposited: strategy.initialDeposit.reduce(
            (acc, [, value]) =>
              acc + Number(value) / 10 ** strategy.pools[0].token0.decimals * (strategy.pools[0].price0 ?? 0),
            0
          ), // These would come from actual user data
          totalUsers: acc.totalUsers + strategy.userShares.length,
        };
      }
      return acc;
    },
    {
      totalTvl: 0n,
      avgApy: 0n,
      totalStrategies: strategies?.length || 0,
      deposited: 0,
      totalUsers: 0,
    }
  );

  if (platformStats) {
    platformStats.avgApy =
      platformStats.avgApy / BigInt(strategies?.length || 1) / 100n;
  }

  if (!strategies || !tokens.length) {
    return (
      <SquareLoader
        className="mx-auto"
        color={colors.amber[500]}
        loading={true}
        size={20}
      />
    );
  }

  if (selectedStrategy) {
    const strategy = strategies.find((s) => s.id === selectedStrategy)!;

    console.log("b", balances?.[strategy.id]);

    return (
      <Strategy
        value={strategy}
        onBack={() => setSelectedStrategy(undefined)}
        balance={balances?.[strategy.id]}
      />
    );
  }

  const filteredStrategies = strategies?.filter(strategy => 
    strategy.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    strategy.pools[0].token0.symbol.toLowerCase().includes(searchTerm.toLowerCase()) ||
    strategy.pools[0].token1.symbol.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const filteredUserStrategies = showUserStrategies 
    ? strategies?.filter(strategy => 
        strategy.userShares.some(([principal]) => principal.toString() === user?.principal.toString())
      )
    : filteredStrategies;

  return (
    <motion.div
      key="3"
      className="grid gap-y-[35px]"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.3 }}
    >
      {user && (
        <>
          <h3 className="text-lg font-bold">Your Stats</h3>
          <UserStats />
        </>
      )}
      <>
        <h3 className="text-lg font-bold">Platform Stats</h3>
        <Card className="p-[20px]" light={!!user}>
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-8">
            <div>
              <h3 className="text-gray-600 text-sm">DEPOSITED</h3>
              <p className="text-2xl font-bold">
                ${(platformStats?.deposited ?? 0 / 10 ** 8).toFixed(2) ?? "0"}
              </p>
            </div>
            <div>
              <h3 className="text-gray-600 text-sm">TOTAL USERS</h3>
              <p className="text-2xl font-bold">
                {platformStats?.totalUsers.toLocaleString() ?? "0"}
              </p>
            </div>
            <div>
              <h3 className="text-gray-600 text-sm">HIGHEST APY</h3>
              <p className="text-2xl font-bold">
                {Number(platformStats?.avgApy ?? 0)}%
              </p>
            </div>
            <div>
              <h3 className="text-gray-600 text-sm">TVL</h3>
              <p className="text-2xl font-bold">
                ${platformStats?.totalTvl.toLocaleString() ?? "0"}
              </p>
            </div>
            <div>
              <h3 className="text-gray-600 text-sm">STRATEGIES</h3>
              <p className="text-2xl font-bold">
                {platformStats?.totalStrategies ?? 0}
              </p>
            </div>
          </div>
        </Card>
      </>
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-8">
        {/* Platform Stats */}

        {/* TVL Chart */}
        {/* <Card className="overflow-hidden mb-[50px]">
          <div className="flex flex-row items-center justify-between px-4 pt-2 pb-0 mb-0">
            <div className="text-2xl font-mono">APR Change</div>
            <div className="flex gap-2">
              {(["24h", "1m", "1y", "all"] as const).map((p) => (
                <Button
                  key={p}
                  onClick={() => setPeriod(p)}
                  className="!h-[24px] !min-w-[40px] !px-2 text-xs"
                  bg={period === p ? "#fbbf24" : "#fef3c7"}
                >
                  {p}
                </Button>
              ))}
            </div>
          </div>
          <LineChart
            period={period}
            series={[
              { name: "IcpSwap", data: icpSwap, color: "#a855f7" },
              { name: "KongSwap", data: kongSwap, color: "#22c55e" },
            ]}
          />
        </Card> */}
      </div>

      {/* Search and Filters */}
      <div className="flex flex-col items-center md:items-end md:flex-row justify-between justify-end md:justify-between mb-[10px]">
        <div className="flex items-center mb-[20px] md:mb-0 gap-4">
          {(user ? ["All", "My strategies"] : ([] as const)).map((p) => (
            <Button
              key={p}
              onClick={() => {
                setShowUserStrategies(p === "My strategies");
              }}
              className="!h-[24px] !min-w-[40px] !px-2 text-xs"
              bg={p === "My strategies" ? (showUserStrategies ? "#fbbf24" : "#fef3c7") : (showUserStrategies ? "#fef3c7" : "#fbbf24")}
            >
              {p}
            </Button>
          ))}
        </div>
        <Input
          type="text"
          placeholder="Search by name or token..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="w-full max-w-md h-[35px] text-[14px]"
        />
      </div>

      {/* Strategies List */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-y-[35px] gap-8">
        {(showUserStrategies
          ? filteredUserStrategies
          : filteredStrategies
        )?.map((s) => {
          const logos = getStrategyTokenLogos(s, tokens);
          const currentPool = s.currentPool;
          const isDisabled = !currentPool;

          return (
            <Card
              key={s.id}
              className="p-[20px] relative hover:shadow-lg transition-shadow duration-200"
              bg={isDisabled ? colors.gray[200] : colors.amber[200]}
              shadowColor={isDisabled ? colors.gray[600] : colors.amber[600]}
            >
              {/* First row - Logo, Name, Description, TVL */}
              <div className="flex items-start gap-6">
                <div className="w-[100px]">
                  <TokensLogos logos={logos} />
                </div>
                <div className="flex flex-col flex-grow">
                  <div className="flex items-center gap-2">
                    <h3 className="text-[20px] font-semibold">{s.name}</h3>
                    <span
                      className={
                        "px-2 py-0.5 rounded text-sm " +
                        (getProfitLevel(s) === "Hot"
                          ? "bg-red-500 text-white"
                          : getProfitColor(getProfitLevel(s)))
                      }
                    >
                      {getProfitLevel(s).toUpperCase()}
                    </span>
                  </div>
                  <p className="text-sm text-gray-600 mt-2">
                    {s.description || "Earn rewards by providing liquidity"}
                  </p>
                </div>

                <div className="text-right">
                  <p className="text-sm text-gray-600">TVL</p>
                  <p className="text-lg font-medium">${s.tvl.toString()}</p>
                </div>
              </div>

              {/* Second row - APY and Button */}
              <div className="flex justify-center items-center gap-20 mt-6 pt-6 border-t border-amber-600/20">
                <div className="flex items-baseline gap-2">
                  <span className="gradient-text text-[30px] font-bold">
                    {Number(s.apy) / 100}%
                  </span>
                  <span className="text-gray-600">APY</span>
                </div>

                <Button
                  className="w-[120px] h-[36px] text-sm"
                  onClick={() => {
                    if (!isDisabled) setSelectedStrategy(s.id);
                  }}
                  disabled={isDisabled}
                >
                  {isDisabled ? "Soon..." : "Jump into!"}
                </Button>
              </div>

              {/* Third row - Additional info */}
              <div
                className={
                  (user
                    ? "grid grid-cols-2 md:grid-cols-4"
                    : "grid grid-cols-2") +
                  " gap-x-8 mt-6 pt-6 border-t border-amber-600/20 text-sm justify-items-stretch text-center"
                }
              >
                <div>
                  <span className="text-gray-600 block">Platform:</span>
                  <p className="font-medium">
                    {/* TODO: Fix this */}
                    {Array.from(["KongSwap", "IcpSwap"]).join(", ")}
                  </p>
                </div>
                <div>
                  {/* TODO: Fix this */}
                  <span className="text-gray-600 block">Deposit Token:</span>
                  <p className="font-medium">{s.pools[0]?.token0.symbol}</p>
                </div>
                {user && (
                  <>
                    <div>
                      <span className="text-gray-600 block">Deposited:</span>
                      <p className="font-medium">
                        {balances?.[s.id]?.initial_deposit?.toLocaleString() ??
                          "0"}
                        "N/A"
                      </p>
                    </div>
                    <div>
                      <span className="text-gray-600 block">
                        Yield (Daily):
                      </span>
                      <p className="font-medium">
                        $
                        {balances?.[s.id]?.user_shares
                          ? (balances[s.id].user_shares * 0.001).toFixed(2)
                          : "0.00"}
                        "N/A"
                      </p>
                    </div>
                  </>
                )}
              </div>
            </Card>
          );
        })}
      </div>
    </motion.div>
  );
}
