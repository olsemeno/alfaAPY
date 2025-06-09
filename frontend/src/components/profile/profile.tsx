import { useState } from "react";
import { LineChart } from "../charts/line-chart";
import { Button, Card } from "../ui";
import { PaymentsCard } from "../payments";
import { UserStats } from "./user-stats";

export function Profile() {
  // Dropdown for chart label
  const [chartType, setChartType] = useState<"APR Change" | "TVL Change">(
    "APR Change"
  );
  const [dropdownOpen, setDropdownOpen] = useState(false);
  const [period, setPeriod] = useState<"24h" | "1m" | "1y" | "all">("24h");

  // Add back the chartColor variable but with the correct colors
  const chartColor = chartType === "TVL Change" ? "#22c55e" : "#a855f7";

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
    const provider = "KongSwap";
    const { icpSwap, kongSwap } = generateMockData(period);
    const chartData = provider === "KongSwap" ? kongSwap : icpSwap;

  return (
    <div className="grid grid-cols-1 gap-8">
      <UserStats />
      <div className="grid grid-cols-1 gap-8">
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
      </div>
      <PaymentsCard />
    </div>
  );
}
