import {
  useBalances,
  useDeposit,
  usePools,
  useTokens,
  useWithdraw,
} from "../../hooks";
import { Card } from "../ui";
import { TokensLogos } from "./tokens-logos";
import { getStrategyTokenLogos, getTokenLogo } from "./utils";
import { useEffect, useState } from "react";
import { Deposit } from "./deposit";
import { Withdraw } from "./withdraw";
import { useAgent, useAuth } from "@nfid/identitykit/react";
import { motion } from "framer-motion";
import BigNumber from "bignumber.js";
import { LineChart } from "../charts/line-chart";
import { Button } from "../ui";
import { Tabs } from "../ui/tabs";
import clsx from "clsx";
import { useNavigate } from "react-router-dom";
import { ConnectWallet } from "../connect-wallet";
import { PaymentsCard } from "../payments";
import { Strategy as StrategyResponse } from "../../services/strategies/strategy-service";

interface APYBreakdown {
  vaultApr: number;
  tradingApr: number;
  boostApr: number;
  totalApy: number;
}

export function Strategy({
  value,
  onBack,
  balance,
}: {
  value: StrategyResponse;
  onBack: () => unknown;
  balance?: {
    user_shares: number;
    total_shares: number;
    initial_deposit: number;
    // usd_balance: number;
    // price: string;
    // amount_0: number;
    // amount_1: number;
  };
}) {
  const navigate = useNavigate();
  const { user } = useAuth();
  const agent = useAgent({ host: "https://ic0.app" });
  const { tokens } = useTokens();
  const logos = tokens ? getStrategyTokenLogos(value, tokens) : [];
  const { resetPools } = usePools(value.pools.map((p) => p.token0.symbol));
  const currentPool = value.pools.find((p) => p.id === value.currentPool);
  const tokenAddress = currentPool?.token0.ledger;
  const token = tokens.length
    ? tokens.find((t) => t.ledger === tokenAddress)!
    : undefined;
  const { balances, refetchBalanceByCanister } = useBalances();
  const tokenBalance = token ? balances[token.ledger] : undefined;
  const [depositOpen, setDepositOpen] = useState(false);
  const [withdrawOpen, setWithdrawOpen] = useState(false);

  useEffect(() => {
    if (token && !tokenBalance) refetchBalanceByCanister(token);
  }, [token, refetchBalanceByCanister, tokenBalance]);

  const { deposit, isDepositing } = useDeposit();
  const { withdraw, isWithdrawing } = useWithdraw();

  // const amountToWithdraw = balance
  //   ? BigNumber(balance.user_shares)
  //       .div(balance.total_shares)
  //       .multipliedBy(
  //         // BigNumber(balance.amount_1)
  //         //   .multipliedBy(currentPool?.price ?? 0)
  //         //   .plus(balance.amount_0)  TODO: fix this
  //         1
  //       )
  //       .toFixed(token!.decimals)
  //   : "0";

  const amountToWithdraw = balance ? BigNumber(balance.initial_deposit).toString() : "0";
  const shares = balance?.user_shares ?? 0;

  // Calculate APY breakdown
  const apyBreakdown: APYBreakdown = {
    vaultApr: 0,
    tradingApr: 0.65, // Example value, replace with actual calculation
    boostApr: 0, // We don't have boost yet
    totalApy: 0,
  };

  // Dropdown for chart label
  const [chartType, setChartType] = useState<"APR Change" | "TVL Change">(
    "APR Change"
  );
  const [dropdownOpen, setDropdownOpen] = useState(false);
  const [period, setPeriod] = useState<"24h" | "1m" | "1y" | "all">("24h");

  // Add back the chartColor variable but with the correct colors
  const chartColor = chartType === "TVL Change" ? "#22c55e" : "#a855f7"; // Green for TVL, Purple for APR

  // Mock data generator for chart
  const generateMockData = (period: "24h" | "1m" | "1y" | "all") => {
    type PeriodKey = "24h" | "1m" | "1y" | "all";
    const now = Date.now();
    const periods: Record<PeriodKey, { length: number; interval: number }> = {
      "24h": { length: 24, interval: 60 * 60 * 1000 },
      "1m": { length: 30, interval: 24 * 60 * 60 * 1000 },
      "1y": { length: 12, interval: 30 * 24 * 60 * 60 * 1000 },
      all: { length: 24, interval: 30 * 24 * 60 * 60 * 1000 },
    };
    const safePeriod: PeriodKey = period in periods ? period : "24h";
    const { length, interval } = periods[safePeriod];
    const icpSwap = Array.from({ length }, (_, i) => ({
      x: now - (length - 1 - i) * interval,
      y: 1000000 + Math.random() * 500000,
    }));
    const kongSwap = Array.from({ length }, (_, i) => ({
      x: now - (length - 1 - i) * interval,
      y: 800000 + Math.random() * 400000,
    }));
    return { icpSwap, kongSwap };
  };

  // Chart data for this strategy
  const provider = value.name.toLowerCase().includes("kong")
    ? "KongSwap"
    : "IcpSwap";
  const { icpSwap, kongSwap } = generateMockData(period);
  const chartData = provider === "KongSwap" ? kongSwap : icpSwap;

  // New Details Card with Tabs
  const [detailsTab, setDetailsTab] = useState<"tokens" | "providers">(
    "tokens"
  );

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      transition={{ duration: 0.3 }}
      className="grid grid-cols-1 gap-y-[25px] max-w-[1400px] mx-auto"
    >
      {/* Back button row */}
      <div>
        <button
          onClick={() => {
            onBack();
            resetPools();
          }}
          className="text-gray-600 hover:text-gray-800 transition-colors"
        >
          ← Back
        </button>
      </div>

      {/* Strategy name and logo row */}
      <div className="flex items-center gap-4">
        <TokensLogos logos={logos} size={48} />
        <div>
          <h1 className="text-2xl font-bold">{value.name}</h1>
          <p className="text-gray-600">{value.description}</p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-3">
        {/* TVL Chart */}
        <Card className="overflow-hidden">
          <div className="flex flex-row items-center justify-between px-4 pt-2 pb-0 mb-0">
            <div className="relative inline-block">
              <select
                value={chartType}
                onChange={(e) =>
                  setChartType(e.target.value as "APR Change" | "TVL Change")
                }
                onFocus={() => setDropdownOpen(true)}
                onBlur={() => setDropdownOpen(false)}
                className="bg-transparent outline-none text-lg font-mono appearance-none pr-6"
                style={{ boxShadow: "none", border: "none" }}
              >
                <option value="APR Change">APR Change</option>
                <option value="TVL Change">TVL Change</option>
              </select>
              <span className="pointer-events-none absolute right-1 top-1/2 transform -translate-y-1/2 text-lg select-none">
                {dropdownOpen ? "▲" : "▼"}
              </span>
            </div>
            <div className="flex gap-4">
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
              {
                name: provider,
                data: chartData,
                color: chartColor,
              },
            ]}
          />
        </Card>
        {/* Right column - Deposit/Withdraw */}
        <div className="grid grid-cols-1 gap-8">
          {/* User Balance Card */}
          <Card className="p-6">
            <h3 className="text-lg font-semibold mb-4">💸 Your Position</h3>
            {user && tokenBalance ? (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="space-y-4">
                  <div>
                    <p className="text-gray-600">Available to Deposit</p>
                    <div className="flex items-center gap-2">
                      <TokensLogos logos={[token?.logo?.[0] ?? ""]} size={24} />
                      <p className="text-lg font-medium">
                        {tokenBalance?.balance ?? "0"} {token?.symbol}
                        <span className="text-gray-500 text-sm ml-2">
                          (${tokenBalance?.usdBalance ?? "0"})
                        </span>
                      </p>
                    </div>
                  </div>
                  <div>
                    <p className="text-gray-600">Your Deposit</p>
                    <div className="flex items-center gap-2">
                      <TokensLogos logos={logos} size={24} />
                      <p className="text-lg font-medium">
                        {/* ${balance?.usd_balance.toFixed(2) ?? "0.00"} */}
                        "0.00"
                      </p>
                    </div>
                  </div>
                  <div>
                    <p className="text-gray-600">Daily Yield</p>
                    <p className="text-lg font-medium text-green-600">
                      $
                      {/* {(
                        (balance?.usd_balance ?? 0) *
                        (apyBreakdown.totalApy / 365 / 100)
                      ).toFixed(2)} */}
                      "N/A"
                    </p>
                  </div>
                </div>
                {/* Deposit/Withdraw Buttons */}
                <div>
                  <div className="flex flex-col gap-6 md:items-end md:content-center">
                    <Deposit
                      className="md:w-[150px]"
                      loading={isDepositing}
                      isOpen={depositOpen}
                      onClose={() => {
                        setDepositOpen(false);
                      }}
                      onClick={() => {
                        setDepositOpen(true);
                      }}
                      onDeposit={async (amount) => {
                        try {
                          await deposit({
                            amount: BigInt(
                              BigNumber(amount)
                                .multipliedBy(Math.pow(10, token!.decimals))
                                .toString()
                            ),
                            strategyId: value.id,
                            ledger: tokenAddress as string,
                            principal: user!.principal,
                            agent: agent!,
                          });
                        } catch (e: unknown) {
                          if (e instanceof Error) {
                            alert(e.message);
                          } else {
                            alert('An unknown error occurred');
                          }
                        }
                      }}
                      balance={tokenBalance?.balance ?? "0"}
                      tokenSymbol={token?.symbol ?? ""}
                    />
                    <Withdraw
                      className="md:w-[150px]"
                      isOpen={withdrawOpen}
                      onClose={() => {
                        setWithdrawOpen(false);
                      }}
                      onClick={() => {
                        setWithdrawOpen(true);
                      }}
                      onWithdraw={async (percent) => {
                        try {
                          await withdraw({
                            amount: (BigInt(shares) * BigInt(percent)) / BigInt(100),
                            strategyId: value.id,
                            ledger: tokenAddress as string,
                            principal: user!.principal,
                            agent: agent!,
                          });
                        } catch (e: unknown) {
                          if (e instanceof Error) {
                            alert(e.message);
                          } else {
                            alert('An unknown error occurred');
                          }
                        }
                      }}
                      available={amountToWithdraw}
                      tokenSymbol={token?.symbol ?? ""}
                      loading={isWithdrawing}
                    />
                    <Button
                      className="md:w-[150px]"
                      onClick={() => navigate("/swap")}
                    >
                      <span className="text-[20px] block mr-[5px]">🔄</span>{" "}
                      Swap
                    </Button>
                  </div>
                </div>
              </div>
            ) : (
              <div className="flex flex-col flex-1 justify-center items-center py-8">
                <p className="text-gray-600 mb-4">
                  Connect your wallet to deposit
                </p>
                <ConnectWallet />
              </div>
            )}
          </Card>
        </div>
      </div>
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-3">
        {/* Combined Stats Card */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4">🔢 Strategy Stats</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
            <div>
              <p className="text-gray-600">Users in Pool</p>
              <p className="text-lg font-medium">{value.userShares.length}</p>
            </div>
            <div>
              <p className="text-gray-600">Last Deposit</p>
              {value.initialDeposit.length > 0 ? (
                <p className="text-lg font-medium">
                  {value.initialDeposit[
                    value.initialDeposit.length - 1
                  ][1].toString()}{" "}
                  by{" "}
                  {value.initialDeposit[value.initialDeposit.length - 1][0]
                    .toString()
                    .slice(0, 8)}
                  ...
                </p>
              ) : (
                <p className="text-lg font-medium">No deposits yet</p>
              )}
            </div>
            <div>
              <p className="text-gray-600">Strategy Age</p>
              <p className="text-lg font-medium">
                {"N/A"}
              </p>
            </div>
          </div>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-6">
            <div>
              <p className="text-gray-600">TVL</p>
              <p className="text-lg font-medium">
                $
                { "0"}
              </p>
            </div>
            <div>
              <p className="text-gray-600">APY</p>
              <p className="text-lg font-medium">
                {(apyBreakdown.totalApy * 100).toFixed(2)}%
              </p>
            </div>
          </div>
        </Card>

        {/* Pools Table */}
        <Card>
          <h3 className="text-lg font-semibold mb-4 px-6 pt-6">🏊 Pools</h3>
          <div className="overflow-x-auto pb-6">
            <table className="w-full">
              <thead>
                <tr>
                  <th className="text-left py-2 text-gray-600 font-medium"></th>
                  <th className="text-left py-2 text-gray-600 font-medium">
                    Pair
                  </th>
                  <th className="text-left py-2 text-gray-600 font-medium">
                    Provider
                  </th>
                  <th className="text-left py-2 text-gray-600 font-medium">
                    TVL
                  </th>
                  <th className="text-left py-2 text-gray-600 font-medium">
                    APY
                  </th>
                </tr>
              </thead>
              <tbody>
                {[
                  {
                    lp_token_symbol: "ICP_CHAT",
                    symbol_0: "ICP",
                    symbol_1: "CHAT",
                    price: 1.02,
                    tvl: 500000,
                    rolling_24h_apy: 15.2,
                    provider: "IcpSwap",
                    active: true,
                  },
                ].map((p, i) => (
                  <tr
                    key={i}
                    className={clsx("border-t border-amber-600/10", {
                      ["bg-amber-300"]: p.active,
                    })}
                  >
                    <td
                      className={clsx("py-4", { ["rounded-l-lg"]: p.active })}
                    >
                      <TokensLogos
                        size={30}
                        logos={p.lp_token_symbol
                          .split("_")
                          .map((symbol) => getTokenLogo(symbol, tokens))}
                      />
                    </td>
                    <td className="py-4">
                      <span className="font-medium">
                        {p.symbol_0}/{p.symbol_1}
                      </span>
                    </td>
                    <td className="py-4">
                      <span className="font-medium">{(p).provider}</span>
                    </td>
                    <td className="py-4">${Number(p.tvl).toLocaleString()}</td>
                    <td
                      className={clsx("py-4 font-medium", {
                        ["rounded-r-lg"]: p.active,
                      })}
                    >
                      {(p.rolling_24h_apy / 100).toFixed(2)}%
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </Card>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-[minmax(300px,_1fr)_2fr] gap-8">
        {/* New Details Card with Tabs */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4">ℹ️ Details</h3>
          <Tabs
            tabs={[
              { id: "tokens", label: "Tokens", icon: "💰" },
              { id: "providers", label: "Providers", icon: "🏦" },
            ]}
            activeTab={detailsTab}
            onTabChange={(tabId) =>
              setDetailsTab(tabId as "tokens" | "providers")
            }
            className="mb-6"
          />
          {detailsTab === "tokens" && (
            <div className="flex flex-col md:flex-row gap-6">
              {logos.slice(0, 2).map((logo, idx) => {
                const tokenSymbol =
                  idx === 0 ? currentPool?.token0.symbol : currentPool?.token1.symbol;
                const tokenObj = tokens.find((t) => t.symbol === tokenSymbol);
                // Hardcode prices for the two tokens
                let hardcodedPrice = "N/A";
                if (tokenSymbol === "ICP") hardcodedPrice = "$12.34";
                if (tokenSymbol === "CHAT") hardcodedPrice = "$0.56";
                if (tokenSymbol === "ckBTC") hardcodedPrice = "$65000.12";
                return (
                  <div
                    key={tokenSymbol}
                    className="flex items-center gap-4 p-4 rounded-lg bg-white/40 shadow"
                  >
                    <TokensLogos logos={[logo]} size={40} />
                    <div>
                      <div className="font-bold text-lg">{tokenSymbol}</div>
                      <div className="text-gray-600 text-sm">
                        {tokenObj?.name ?? ""}
                      </div>
                      <div className="text-black text-base mt-1">
                        {hardcodedPrice}
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
          {detailsTab === "providers" && (
            <div className="flex flex-col gap-6">
              {Array.from(
                new Set(value.pools.map((p) => p.provider).filter(Boolean))
              ).map((provider) => (
                <div
                  key={provider.toString()}
                  className="flex items-center gap-4 p-4 rounded-lg bg-white/40 shadow"
                >
                  <span
                    className={
                      `flex items-center justify-center rounded-full w-10 h-10 text-2xl ` +
                      ( "bg-gray-300")
                    }
                  >
                    {"❓"}
                  </span>
                  <div>
                    <div className="font-bold text-lg">{provider.toString()}</div>
                    <div className="text-gray-600 text-sm mb-2">
                      {provider.toString() === "KongSwap"
                        ? "KongSwap is a decentralized AMM on ICP."
                        : provider.toString() === "IcpSwap"
                        ? "IcpSwap is a leading DEX on ICP."
                        : "No description."}
                    </div>
                    <div className="flex gap-4">
                      <a
                        href={
                          provider.toString() === "KongSwap"
                            ? "https://kongswap.com/"
                            : "https://icpswap.com/"
                        }
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-blue-700 hover:underline"
                      >
                        Website
                      </a>
                      <a href="#" className="text-blue-700 hover:underline">
                        Docs
                      </a>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </Card>
        <PaymentsCard />
      </div>
    </motion.div>
  );
}
