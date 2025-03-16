import { useEffect } from "react";
import {
  deposit,
  fetchPools,
  resetPools,
  Status,
  useDispatch,
  useSelector,
  withdraw,
} from "../store";
import { useStrategies } from "./strategies";

export function usePools(pools_symbols: string[]) {
  const dispatch = useDispatch();

  const {
    fetchPools: { pools, status },
  } = useSelector((state) => state.strategy);

  useEffect(() => {
    if (status === Status.IDLE && !pools.length)
      dispatch(fetchPools(pools_symbols));
  }, [status, pools, dispatch, pools_symbols]);

  return {
    pools,
    resetPools: () => dispatch(resetPools()),
  };
}

export function useDeposit() {
  const dispatch = useDispatch();

  const {
    deposit: { status },
  } = useSelector((state) => state.strategy);

  return {
    deposit: (...params: Parameters<typeof deposit>) =>
      dispatch(deposit(...params)),
    isDepositing: status === Status.LOADING,
    depositDisabled: !useStrategies().service,
  };
}

export function useWithdraw() {
  const dispatch = useDispatch();

  const {
    withdraw: { status },
  } = useSelector((state) => state.strategy);

  return {
    withdraw: (...params: Parameters<typeof deposit>) =>
      dispatch(withdraw(...params)),
    isWithdrawing: status === Status.LOADING,
    withdrawDisabled: !useStrategies().service,
  };
}
