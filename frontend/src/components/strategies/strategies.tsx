import colors from "tailwindcss/colors";
import { useStrategies } from "../../hooks/strategies";
import SquareLoader from "react-spinners/ClimbingBoxLoader";
import { Button, Card } from "../ui";
import { useTokens } from "../../hooks";
import { useState } from "react";
import { Strategy } from "./strategy";
import { TokensLogos } from "./tokens-logos";
import { getStrategyTokenLogos } from "./utils";
import { motion } from "framer-motion";
import { useAuth } from "@nfid/identitykit/react";

export function Strategies() {
  const { user } = useAuth();
  const { strategies, balances } = useStrategies(user?.principal.toString());
  const { tokens } = useTokens();
  const [selectedStrategy, setSelectedStrategy] = useState<
    number | undefined
  >();

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
    return (
      <Strategy
        value={strategy}
        onBack={() => setSelectedStrategy(undefined)}
        balance={
          strategy.current_pool.length ? balances?.[strategy.id] : undefined
        }
      />
    );
  }

  return (
    <motion.div
      key="3"
      className="grid gap-y-[25px]"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.3 }}
    >
      {strategies?.map((s) => {
        const logos = getStrategyTokenLogos(s, tokens);
        const isDisabled = !s.current_pool.length;
        return (
          <Card
            className="p-[20px] relative"
            bg={isDisabled ? colors.gray[200] : colors.amber[200]}
            shadowColor={isDisabled ? colors.gray[600] : colors.amber[600]}
          >
            <div className="flex">
              <div className="mr-[40px] flex flex-col justify-center w-[100px]">
                <TokensLogos logos={logos} />
              </div>
              <div className="flex flex-col justify-center">
                <h3 className="text-[20px] mb-[10px]">{s.name}</h3>
                <h4 className="text-[16px] text-gray-600">
                  {s.pools.map(
                    (p, i) =>
                      p.replace("_", "/") +
                      (i !== s.pools.length - 1 ? ", " : "")
                  )}
                </h4>
                <p className="mt-[5px] mb-0">
                  {user && s.current_pool.length ? (
                    <>
                      <span className="text-[20px] mr-[4px]">💸</span>
                      {`Deposited: $${
                        balances?.[s.id]?.usd_balance.toFixed(2) ?? "0.00"
                      }`}
                    </>
                  ) : undefined}
                </p>
              </div>
              <div className="flex flex-col ml-auto">
                {s.current_pool.length ? (
                  <h2 className="text-[30px] flex mb-[20px]">
                    <span className="gradient-text mr-[5px] font-bold">
                      {(s.current_pool[0].rolling_24h_apy / 100).toFixed(2)}%
                    </span>
                    APY
                  </h2>
                ) : undefined}
                <Button
                  className="w-[120px] ml-auto"
                  onClick={() => {
                    if (!isDisabled) setSelectedStrategy(s.id);
                  }}
                  disabled={isDisabled}
                >
                  {isDisabled ? "Soon..." : "Jump into!"}
                </Button>
              </div>
            </div>
          </Card>
        );
      })}
    </motion.div>
  );
}
