import { createSlice, createAsyncThunk } from "@reduxjs/toolkit";
import { StrategiesService } from "../../services/strategies/service";
import { StrategyResponse } from "../../services/strategies/idl/vault";
import { Agent } from "@dfinity/agent";
import { Status } from "../types";
import { Principal } from "@dfinity/principal";

export const fetchStrategies = createAsyncThunk(
  "strategies/fetch",
  async () => {
    try {
      const response = await StrategiesService.get_strategies();
      return response;
    } catch (e) {
      console.error(e);
    }
  }
);

export const fetchStrategiesBalances = createAsyncThunk(
  "strategies/fetchBalances",
  async ({ principal }: { principal: string }) => {
    try {
      const balances: Array<{
        strategy_id: number;
        user_shares: number;
        total_shares: number;
        price: string;
        usd_balance: number;
        amount_0: number;
        amount_1: number;
      }> = await StrategiesService.get_user_strategies(
        Principal.from(principal)
      );
      return balances.reduce(
        (acc, value) => ({
          ...acc,
          [value.strategy_id]: value,
        }),
        {}
      );
    } catch (e) {
      console.error(e);
    }
  }
);

export const initStrategies = createAsyncThunk(
  "strategies/init",
  async (agent: Agent) => {
    const response = await StrategiesService.build(agent);
    return response;
  }
);

const strategiesSlice = createSlice({
  name: "strategies",
  initialState: {
    strategies: {
      status: Status.IDLE,
    },
    service: {
      status: Status.IDLE,
    },
    balances: {
      status: Status.IDLE,
      data: {},
    },
  } as {
    strategies: {
      data?: Array<StrategyResponse>;
      status: Status;
      error?: string;
    };
    service: {
      data?: StrategiesService;
      status: Status;
      error?: string;
    };
    balances: {
      data?: Record<
        string,
        {
          user_shares: number;
          total_shares: number;
          price: string;
          usd_balance: number;
          amount_0: number;
          amount_1: number;
        }
      >;
      status: Status;
      error?: string;
    };
  },
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(initStrategies.pending, (state) => {
        state.service.status = Status.LOADING;
      })
      .addCase(initStrategies.fulfilled, (state, action) => {
        state.service.status = Status.SUCCEEDED;
        state.service.data = action.payload;
      })
      .addCase(initStrategies.rejected, (state, action) => {
        state.service.status = Status.FAILED;
        state.service.error = action.error.message;
      })
      .addCase(fetchStrategies.pending, (state) => {
        state.strategies.status = Status.LOADING;
      })
      .addCase(fetchStrategies.fulfilled, (state, action) => {
        state.strategies.status = Status.SUCCEEDED;
        state.strategies.data = action.payload;
      })
      .addCase(fetchStrategies.rejected, (state, action) => {
        state.strategies.status = Status.FAILED;
        state.strategies.error = action.error.message;
      })
      .addCase(fetchStrategiesBalances.pending, (state) => {
        state.balances.status = Status.LOADING;
      })
      .addCase(fetchStrategiesBalances.fulfilled, (state, action) => {
        state.strategies.status = Status.SUCCEEDED;
        state.balances.data = action.payload;
      })
      .addCase(fetchStrategiesBalances.rejected, (state, action) => {
        state.strategies.status = Status.FAILED;
        state.balances.error = action.error.message;
      });
  },
});

export const strategiesReducer = strategiesSlice.reducer;
