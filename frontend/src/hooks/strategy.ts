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
import toaster from "../components/ui/toast";

export function usePools(pools_symbols: string[]) {
  const dispatch = useDispatch();

  const {
    fetchPools: { pools, status },
  } = useSelector((state) => state.strategy);

  useEffect(() => {
    if (status === Status.IDLE && !pools.length)
      dispatch(fetchPools());
  }, [status, pools, dispatch, pools_symbols]);

  return {
    pools,
    resetPools: () => dispatch(resetPools()),
  };
}

export function useDeposit() {
  const dispatch = useDispatch();

  const {
    deposit: { status, error },
  } = useSelector((state) => state.strategy);

  useEffect(() => {
    if (status === Status.SUCCEEDED) {
      toaster.success("Successfully deposited");
      setTimeout(() => {
        window.location.reload();
      }, 500);
    } else if (status === Status.FAILED && error) {
      toaster.error(error);
    }
  }, [status, error]);

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
    withdraw: { status, error },
  } = useSelector((state) => state.strategy);

  useEffect(() => {
    if (status === Status.SUCCEEDED) {
      toaster.success("Successfully withdrawed");
      setTimeout(() => {
        window.location.reload();
      }, 500);
    } else if (status === Status.FAILED && error) {
      toaster.error(error);
    }
  }, [status, error]);

  return {
    withdraw: (...params: Parameters<typeof deposit>) =>
      dispatch(withdraw(...params)),
    isWithdrawing: status === Status.LOADING,
    withdrawDisabled: !useStrategies().service,
  };
}
