// import { useEffect } from "react";
// import { useDispatch, useSelector } from "../store";
// import { fetchStats } from "../store/slices";

// export const useStats = () => {
//   const dispatch = useDispatch();
//   const stats = useSelector((state) => state.stats.stats);
//   const status = useSelector((state) => state.stats.status);

//   useEffect(() => {
//     if (status === "idle" && !stats) {
//       dispatch(fetchStats());
//     }
//   }, [dispatch, stats, status]);

//   return {
//     stats,
//   };
// };
