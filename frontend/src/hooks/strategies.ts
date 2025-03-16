import { useEffect } from "react";
import {
  fetchStrategies,
  fetchStrategiesBalances,
  initStrategies,
  Status,
  useDispatch,
  useSelector,
} from "../store";
import { useAgent } from "@nfid/identitykit/react";

export function useStrategies(user?: string) {
  const dispatch = useDispatch();
  const agent = useAgent();

  const {
    strategies: { data, status },
    balances: { data: balances, status: balancesStatus },
    service,
  } = useSelector((state) => state.strategies);

  useEffect(() => {
    if (!data && status === Status.IDLE) {
      dispatch(fetchStrategies());
    }
  }, [status, data, dispatch]);

  useEffect(() => {
    if (agent) dispatch(initStrategies(agent));
  }, [agent, dispatch]);

  useEffect(() => {
    if (data?.length && user && balancesStatus === Status.IDLE)
      dispatch(
        fetchStrategiesBalances({
          principal: user,
        })
      );
  }, [data, user, dispatch, balancesStatus]);

  return {
    loading: status === Status.IDLE || status === Status.LOADING,
    strategies: data,
    balances,
    service: service.data,
  };
}
