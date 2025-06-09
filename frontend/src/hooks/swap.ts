import { useCallback, useEffect, useRef, useState } from "react";
import { setSlippage, useDispatch, useSelector } from "../store";
import { swapService } from "../services";
import { SwapName } from "../services/swap/types/enums";
import { Shroff } from "../services/swap/shroff";
import {
  DepositError,
  LiquidityError,
  ServiceUnavailableError,
  SwapError,
  WithdrawError,
} from "../services/swap/errors/types";
import { Quote } from "../services/swap/quote";
import { useAgent, useAuth } from "@nfid/identitykit/react";

export function useSwapSlippage() {
  const dispatch = useDispatch();

  // Use useAppSelector to get state
  const slippage = useSelector((state) => state.swap.slippage);

  return {
    slippage,
    setSlippage: (slippage: number) => dispatch(setSlippage(slippage)),
  };
}

const QUOTE_REFETCH_TIMER = 30;

export function useSwap({
  fromToken,
  toToken,
  amount,
  onSuccess,
}: {
  fromToken: string;
  toToken: string;
  amount: string;
  slippage: number;
  onSuccess: () => unknown;
}) {
  const agent = useAgent();
  const { user } = useAuth();
  const [swapProviders, setSwapProviders] = useState<
    Map<SwapName, Shroff | undefined>
  >(new Map());
  const [shroff, setShroff] = useState<Shroff | undefined>();
  const [quote, setQuote] = useState<Quote | undefined>();
  const [isShroffLoading, setIsShroffLoading] = useState(true);
  const [isQuoteLoading, setIsQuoteLoading] = useState(false);
  const [shroffError, setShroffError] = useState<Error | undefined>();
  const [quoteTimer, setQuoteTimer] = useState(QUOTE_REFETCH_TIMER);
  const [swapError, setSwapError] = useState<
    WithdrawError | SwapError | DepositError | undefined
  >();
  const [slippageQuoteError, setSlippageQuoteError] = useState<
    string | undefined
  >();
  const [liquidityError, setLiquidityError] = useState<Error | undefined>();
  const [providerError, setProviderError] = useState<boolean>(false);
  const [isSwapSuccess, setIsSwapSuccess] = useState(false);

  const getProviders = useCallback(async () => {
    if (!agent || !user?.principal) return;
    try {
      
      console.log("clear quote", fromToken, toToken);
      setQuote(undefined);
      setShroff(undefined);
      if (quoteIntervalRef.current) clearInterval(quoteIntervalRef.current);
      setLiquidityError(undefined);
      setProviderError(false);
      const providers = await swapService.getSwapProviders(
        fromToken,
        toToken,
        agent,
        user.principal.toString()
      );
      console.log("providers", providers);
      setSwapProviders(providers);
    } catch (error) {
      if (error instanceof LiquidityError) {
        setSwapProviders(new Map());
        setLiquidityError(error);
      }
      if (error instanceof ServiceUnavailableError) {
        setProviderError(true);
      }
    }
  }, [fromToken, toToken, agent, user?.principal]);

  useEffect(() => {
    getProviders();
  }, [getProviders]);

  useEffect(() => {
    const getShroff = async () => {
      try {
        const shroff = await swapService.getBestShroff(swapProviders, amount);
        setShroff(shroff);
      } catch (error) {
        setShroff(undefined);
        if (error instanceof LiquidityError) {
          setLiquidityError(error);
        } else {
          console.error("Quote error: ", error);
          setShroffError(error as Error);
          throw error;
        }
      } finally {
        setIsShroffLoading(false);
      }
    };

    getShroff();
  }, [shroffError, amount, swapProviders]);

  const refetchQuote = useCallback(async () => {
    try {
      setIsQuoteLoading(true);
      setLiquidityError(undefined);
      if (!amount || !Number(amount) || !shroff) return;
      return await shroff.getQuote(amount).then(setQuote);
    } catch (error) {
      if (error instanceof LiquidityError) setLiquidityError(error);
    } finally {
      setIsQuoteLoading(false);
    }
  }, [amount, shroff]);

  useEffect(() => {
    setQuote(undefined);
    if (quoteIntervalRef.current) clearInterval(quoteIntervalRef.current);
    refetchQuote();
  }, [refetchQuote]);

  const quoteIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    if (!quote) return;

    // Clear any existing interval immediately
    if (quoteIntervalRef.current) {
      clearInterval(quoteIntervalRef.current);
    }

    quoteIntervalRef.current = setInterval(() => {
      setQuoteTimer((prev) => {
        if (prev === 1) {
          refetchQuote();
          setSlippageQuoteError(undefined);
          return QUOTE_REFETCH_TIMER;
        }
        return prev - 1;
      });
    }, 1000);

    return () => {
      if (quoteIntervalRef.current) {
        clearInterval(quoteIntervalRef.current);
        quoteIntervalRef.current = null;
      }
    };
  }, [quote, refetchQuote, fromToken, toToken]);

  useEffect(() => {
    if (isSwapSuccess) {
      setQuoteTimer(QUOTE_REFETCH_TIMER);
    }
  }, [isSwapSuccess]);

  const swap = useCallback(async () => {
    const sourceAmount = quote?.getSourceAmountPrettifiedWithSymbol();
    console.log("sourceAmount", sourceAmount);
    const targetAmount = quote?.getTargetAmountPrettifiedWithSymbol();
    console.log("targetAmount", targetAmount);
    const sourceUsdAmount = quote?.getSourceAmountUSD();
    console.log("sourceUsdAmount", sourceUsdAmount);
    const targetUsdAmount = quote?.getTargetAmountUSD();
    console.log("targetUsdAmount", targetUsdAmount);

    if (!sourceAmount || !targetAmount || !sourceUsdAmount || !targetUsdAmount)
      return;

    if (!shroff) return;
    
    try {
      await shroff.swap();
      setIsSwapSuccess(true);
      onSuccess();
    } catch (error) {
      setSwapError(error as WithdrawError | SwapError | DepositError);
      throw error;
    }
  }, [quote, shroff, onSuccess]);

  return {
    swap,
    isShroffLoading,
    swapError,
    slippageQuoteError,
    liquidityError,
    providerError,
    quote,
    quoteTimer,
    isQuoteLoading,
    isSuccess: isSwapSuccess,
  };
}
