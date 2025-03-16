import { useCallback } from "react";
import { fetchBalance, Status, useDispatch, useSelector } from "../store";
import { useAuth } from "@nfid/identitykit/react";
import { ICRC1 } from "../idl/icrc1_oracle";

export function useBalances() {
  const dispatch = useDispatch();
  const { user } = useAuth();

  // Use useAppSelector to get state
  const { balances } = useSelector((state) => state.balances);
  const { status, error } = useSelector((state) => state.balance);

  const refetchBalanceByCanister = useCallback(
    (canister: ICRC1) => {
      if (!user?.principal) return;
      dispatch(fetchBalance({ principal: user?.principal, canister }));
    },
    [dispatch, user?.principal]
  );

  return {
    error,
    balances,
    refetchBalanceByCanister,
    loading: status === Status.IDLE || status === Status.LOADING,
  };
}
