import { useDispatch, useSelector } from "../store";
import { fetchPayments } from "../store/slices";
import { useEffect } from "react";

export const usePayments = () => {
  const dispatch = useDispatch();
  const payments = useSelector((state) => state.payments.payments);
  const status = useSelector((state) => state.payments.status);

  useEffect(() => {
    if (status === "idle" && !payments) {
      dispatch(fetchPayments());
    }
  }, [dispatch, payments, status]);

  return { payments, status };
};
