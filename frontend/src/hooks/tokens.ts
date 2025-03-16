import { useEffect } from "react";
import { fetchTokens, Status, useDispatch, useSelector } from "../store";

export function useTokens() {
  const dispatch = useDispatch();

  const { tokens, status, error } = useSelector((state) => state.tokens);

  useEffect(() => {
    if (!tokens.length && status === Status.IDLE) {
      dispatch(fetchTokens());
    }
  }, [tokens, status, dispatch]);

  return {
    error,
    tokens,
    loading: status === Status.IDLE || status === Status.LOADING,
  };
}
