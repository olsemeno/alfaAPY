import { useAuth } from "@nfid/identitykit/react";
import { Button } from "../ui";
import { Input } from "./input";
import clsx from "clsx";
import { useBalances, useSwap, useSwapSlippage, useTokens } from "../../hooks";
import { SlippageModal } from "./slippage";
import { useEffect, useRef, useState } from "react";
import SquareLoader from "react-spinners/ClimbingBoxLoader";
import colors from "tailwindcss/colors";
import debounce from "lodash.debounce";
import { SuccessModal } from "./success-modal";

export function Swap({ className }: { className?: string }) {
  const { user } = useAuth();
  const { tokens, loading } = useTokens();
  const { balances, refetchBalanceByCanister } = useBalances();
  const [slippageModalOpened, setSlippageModalOpened] = useState(false);
  const [fromToken, setFromToken] = useState<string>(
    "ryjl3-tyaaa-aaaaa-aaaba-cai"
  );
  const [toToken, setToToken] = useState<string>("etik7-oiaaa-aaaar-qagia-cai");
  const [amount, setAmount] = useState("");
  const [throttledAmount, setThrottledAmount] = useState("");
  const [isSuccessModalOpen, setIsSuccessModalOpen] = useState(false);
  const [isSwapLoading, setIsSwapLoading] = useState(false);
  
  const throttledSetAmount = useRef(
    debounce((value: string) => {
      setThrottledAmount(value);
    }, 1000)
  ).current;

  useEffect(() => {
    return () => throttledSetAmount.cancel();
  }, []);

  const { setSlippage, slippage } = useSwapSlippage();
  const { isQuoteLoading, liquidityError, swap, quote, quoteTimer, isSuccess } = useSwap({
    fromToken,
    toToken,
    amount: throttledAmount,
    slippage,
    onSuccess: () => {
      refetchBalanceByCanister(tokens.find((t) => t.ledger === fromToken)!);
      refetchBalanceByCanister(tokens.find((t) => t.ledger === toToken)!);
    },
  });

  useEffect(() => {
    if (tokens.length) {
      refetchBalanceByCanister(tokens.find((t) => t.ledger === fromToken)!);
    }
  }, [fromToken, tokens, refetchBalanceByCanister]);

  useEffect(() => {
    if (tokens.length) {
      refetchBalanceByCanister(tokens.find((t) => t.ledger === toToken)!);
    }
  }, [toToken, tokens, refetchBalanceByCanister]);

  useEffect(() => {
    if (isSuccess) {
      setIsSuccessModalOpen(true);
      setIsSwapLoading(false);
    }
  }, [isSuccess]);

  const handleSwapTokens = () => {
    const tempToken = fromToken;
    setFromToken(toToken);
    setToToken(tempToken);
  };

  const handleAmountChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    // Allow only numbers and one dot
    if (/^\d*\.?\d*$/.test(value)) {
      setAmount(value);
      throttledSetAmount(value);
    }
  };

  if (loading || !tokens.length)
    return (
      <SquareLoader
        className="mx-auto"
        color={colors.amber[500]}
        loading={true}
        size={20}
      />
    );

  const dropdownOptions = tokens.map((el) => ({
    icon: el.logo[0] || "",
    label: el.symbol,
    value: el.ledger,
  }));

  const fromBalance = balances[fromToken];
  const toBalance = balances[toToken];

  const targetAmount = quote?.getTargetAmountPrettifiedWithSymbol();
  const sourceUsdAmount = quote?.getSourceAmountUSD();
  const targetUsdAmount = quote?.getTargetAmountUSD();

  const handleSwap = async () => {
    try {
      setIsSwapLoading(true);
      await swap();
    } catch (error) {
      console.error("Swap failed:", error);
    } finally {
      setIsSwapLoading(false);
    }
  };

  const handleSuccessModalClose = () => {
    setIsSuccessModalOpen(false);
    setAmount(""); // Reset input amount
    setThrottledAmount(""); // Reset throttled amount
  };

  return (
    <>
      <div className={clsx("flex flex-col", className)}>
        <div className="mb-[5px] flex justify-end text-[25px] mb-[10px]">
          <div
            className="cursor-pointer"
            onClick={() => setSlippageModalOpened(true)}
          >
            ⚙️
          </div>
        </div>
        <Input
          tokens={dropdownOptions}
          token={dropdownOptions.find((o) => o.value === fromToken)!}
          className="flex justify-center"
          balance={fromBalance?.balance ?? "0.00"}
          usdValue={sourceUsdAmount ?? "0.00"}
          onTokenChange={setFromToken}
          onChange={handleAmountChange}
          value={amount}
          disabled={isQuoteLoading}
        />
        {liquidityError && (
          <div className="h-4 mt-[10px] text-xs leading-4 text-red-600">
            {liquidityError?.message}
          </div>
        )}
        <div 
          className="text-[30px] mx-auto my-[10px] cursor-pointer hover:opacity-80 transition-opacity"
          onClick={handleSwapTokens}
        >
          ↕️
        </div>
        <Input
          value={targetAmount ? targetAmount.split(" ")[0] : "0.00"}
          tokens={dropdownOptions}
          token={dropdownOptions.find((o) => o.value === toToken)!}
          disabled
          className="flex justify-center"
          balance={toBalance?.balance ?? "0.00"}
          usdValue={targetUsdAmount ?? "0.00"}
          onTokenChange={setToToken}
        />
        {amount && quote && (
          <div className="flex items-center justify-between mt-6 text-xs text-gray-500">
            {quote?.getQuoteRate()} ({quoteTimer} sec)
          </div>
        )}
        <Button
          onClick={handleSwap}
          className="mt-[30px]"
          disabled={!user || isQuoteLoading || !amount || !quote || isSwapLoading}
        >
          {user
            ? !amount
              ? "Enter an amount"
              : isQuoteLoading
              ? "Fetching quotes 1 of 2"
              : isSwapLoading
              ? "Swapping..."
              : "Swap tokens"
            : "Connect wallet"}
        </Button>
      </div>
      <SlippageModal
        isOpen={slippageModalOpened}
        onClose={() => setSlippageModalOpened(false)}
        onSlippageChange={(slippage) => setSlippage(parseFloat(slippage))}
      />
      <SuccessModal
        isOpen={isSuccessModalOpen}
        onClose={handleSuccessModalClose}
        fromAmount={amount || "0"}
        toAmount={targetAmount?.split(" ")[0] || "0"}
        fromToken={fromToken}
        toToken={toToken}
      />
    </>
  );
}
