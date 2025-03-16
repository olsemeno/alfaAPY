import colors from "tailwindcss/colors";
import {
  useBalances,
  useDeposit,
  usePools,
  useTokens,
  useWithdraw,
} from "../../hooks";
import { StrategyResponse } from "../../services/strategies/idl/vault";
import { Button, Card } from "../ui";
import { TokensLogos } from "./tokens-logos";
import { getStrategyTokenLogos, getTokenLogo } from "./utils";
import SquareLoader from "react-spinners/ClimbingBoxLoader";
import { useEffect, useState } from "react";
import { Deposit } from "./deposit";
import { Withdraw } from "./withdraw";
import { useAgent, useAuth } from "@nfid/identitykit/react";
import { motion } from "framer-motion";
import BigNumber from "bignumber.js";

export function Strategy({
  value,
  onBack,
  balance,
}: {
  value: StrategyResponse;
  onBack: () => unknown;
  usdBalance?: string;
  balance?: {
    user_shares: number;
    total_shares: number;
    price: string;
    usd_balance: number;
    amount_0: number;
    amount_1: number;
  };
}) {
  const { user } = useAuth();
  const agent = useAgent();
  const { tokens } = useTokens();
  const logos = tokens ? getStrategyTokenLogos(value, tokens) : [];
  const { pools, resetPools } = usePools(value.pools);
  const [depositOpen, setDepositOpen] = useState(false);
  const [withdrawOpen, setWithdrawOpen] = useState(false);
  const tokenAddress =
    value.current_pool[0]?.address_0 ?? "mxzaz-hqaaa-aaaar-qaada-cai";
  const token = tokens.length
    ? tokens.find((t) => t.ledger === tokenAddress)!
    : undefined;
  const { balances, refetchBalanceByCanister } = useBalances();

  const tokenBalance = token ? balances[token.ledger] : undefined;

  useEffect(() => {
    if (token && !tokenBalance) refetchBalanceByCanister(token);
  }, [token, refetchBalanceByCanister, tokenBalance]);

  const { deposit, depositDisabled, isDepositing } = useDeposit();
  const { withdraw, isWithdrawing } = useWithdraw();

  const available = balance
    ? BigNumber(balance.user_shares)
        .div(balance.total_shares)
        .multipliedBy(
          BigNumber(balance.amount_1)
            .multipliedBy(value.current_pool[0]!.price)
            .plus(balance.amount_0)
        )
        .toFixed(token!.decimals)
    : "0";
  const shares = balance?.user_shares ?? 0;

  return (
    <motion.div
      key="3"
      className="grid gap-y-[25px]"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.3 }}
    >
      <Card className="flex flex-col relative p-[20px]">
        <span
          onClick={() => {
            onBack();
            resetPools();
          }}
          className="absolute cursor-pointer top-0 left-0 text-[30px]"
        >
          🔙
        </span>
        <h2 className="text-center text-[32px] mb-[15px]">{value.name}</h2>
        <p className="text-center">{value.description}</p>
        <div className="flex mx-auto mt-[35px] items-center">
          <div className="mr-[20px]">
            <TokensLogos logos={logos} />
          </div>
          <h2 className="text-[30px] flex">
            <span className="gradient-text mr-[5px] font-bold">
              {(value.current_pool[0]!.rolling_24h_apy / 100).toFixed(2)}%
            </span>
            APY
          </h2>
        </div>
        {user && tokenBalance && !depositDisabled && (
          <>
            <div className="flex flex-col items-center justify-center mt-[35px]">
              <div className="flex mb-[10px] items-center">
                <span className="text-[22px] mr-[5px] mt-[-6px]">💼</span>
                <h2>
                  Available Balance: {tokenBalance?.balance ?? "0"}{" "}
                  {token?.symbol} = ${tokenBalance?.usdBalance ?? "0"}
                </h2>
              </div>
              <div className="flex items-center">
                <span className="text-[25px] mr-[5px]">💸</span>
                <h2>Deposited: ${balance?.usd_balance.toFixed(2) ?? "0.00"}</h2>
              </div>
            </div>
            <div className="flex flex-col sm:flex-row mx-auto mt-[35px] w-full justify-center">
              <Deposit
                disabled={isDepositing}
                onClick={() => setDepositOpen(true)}
                className="sm:mr-[30px] mb-[20px] sm:mb-0 w-full sm:w-[140px]"
                isOpen={depositOpen}
                onClose={() => setDepositOpen(false)}
                onDeposit={(amount) => {
                  deposit({
                    amount: BigInt(
                      BigNumber(amount)
                        .multipliedBy(Math.pow(10, token!.decimals))
                        .toString()
                    ),
                    strategyId: value.id,
                    ledger: tokenAddress,
                    principal: user.principal,
                    agent: agent!,
                  })
                    .then(() => {
                      setDepositOpen(false);
                    })
                    .catch((e) => {
                      alert(e.message);
                    });
                }}
                balance={tokenBalance?.balance ?? "0"}
                tokenSymbol={token?.symbol ?? ""}
              />
              <Withdraw
                onClick={() => setWithdrawOpen(true)}
                className="sm:mr-[30px] mb-[20px] sm:mb-0 w-full sm:w-[140px]"
                isOpen={withdrawOpen}
                onClose={() => setWithdrawOpen(false)}
                onWithdraw={(percent) => {
                  withdraw({
                    amount: (BigInt(shares) * BigInt(percent)) / BigInt(100),
                    strategyId: value.id,
                    ledger: tokenAddress,
                    principal: user.principal,
                    agent: agent!,
                  })
                    .then(() => {
                      setWithdrawOpen(false);
                    })
                    .catch((e) => {
                      alert(e.message);
                    });
                }}
                available={available}
                tokenSymbol={token?.symbol ?? ""}
                disabled={isWithdrawing}
              />
              <Button disabled className="w-full sm:w-[140px]">
                <span className="text-[20px]">🔄</span> Swap
              </Button>
            </div>
          </>
        )}
      </Card>
      <h2 className="mt-[30px] text-[24px] text-center">🏊 Pools</h2>
      <div className="flex mt-[30px] flex-col">
        {pools.length ? (
          <motion.div
            key="3"
            className="grid gap-y-[25px]"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.3 }}
          >
            <Card
              className="w-full py-[20px] px-[10px] grid gap-y-[15px]"
              bg={colors.blue[100]}
              shadowColor={colors.blue[400]}
            >
              <table>
                <thead>
                  <th></th>
                  <th className="text-start text-gray-700">Pair</th>
                  <th className="text-start text-gray-700">Price</th>
                  <th className="text-start text-gray-700">Tvl</th>
                  <th className="text-start text-gray-700">APY</th>
                </thead>
                <tbody>
                  {pools.map((p, i) => (
                    <tr key={i}>
                      <td className="py-[5px]">
                        <TokensLogos
                          size={30}
                          logos={p.lp_token_symbol
                            .split("_")
                            .map((symbol) => getTokenLogo(symbol, tokens))}
                        />
                      </td>
                      <td>
                        <h3>
                          {p.symbol_0}/{p.symbol_1}
                        </h3>
                      </td>
                      <td>{p.price}</td>
                      <td>${p.tvl / BigInt(10 ** 6)}</td>
                      <td>{(p.rolling_24h_apy / 100).toFixed(2)}%</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </Card>
          </motion.div>
        ) : (
          <div className="mx-auto mt-[30px]">
            <SquareLoader color={colors.amber[500]} loading={true} size={20} />
          </div>
        )}
      </div>
    </motion.div>
  );
}
