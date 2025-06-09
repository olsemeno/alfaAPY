import { useDispatch, useSelector } from "../store";
import { fetchEventRecords } from "../store/slices/event-records";
import { useEffect } from "react";

export const useEventRecords = () => {
  const dispatch = useDispatch();
  const eventRecords = useSelector((state) => state.eventRecords.eventRecords);
  const status = useSelector((state) => state.eventRecords.status);

  useEffect(() => {
    if (status === "idle" && !eventRecords) {
      dispatch(fetchEventRecords());
    }
  }, [dispatch, eventRecords, status]);

  return { eventRecords, status };
};
