import { configureStore } from "@reduxjs/toolkit";
import {
  tokensReducer,
  balancesReducer,
  balanceReducer,
  strategiesReducer,
  strategyReducer,
  swapReducer,
  paymentsReducer,
} from "./slices";
import { eventRecordsReducer } from "./slices/event-records";

export const store = configureStore({
  reducer: {
    tokens: tokensReducer,
    balances: balancesReducer,
    balance: balanceReducer,
    strategies: strategiesReducer,
    strategy: strategyReducer,
    swap: swapReducer,
    payments: paymentsReducer,
    eventRecords: eventRecordsReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;

export type Dispatch = typeof store.dispatch;
