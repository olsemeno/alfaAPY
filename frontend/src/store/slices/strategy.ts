import { createAction, createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { Status } from "../types";
import { RootState } from "../store";
import { Principal } from "@dfinity/principal";
import { Agent } from "@dfinity/agent";
import { poolStatsService } from "../../services";
import { PoolMetrics } from "../../idl/pool_stats";
import { userService } from "../../services/strategies/user-service";

export const fetchPools = createAsyncThunk(
  "strategy/fetchPools",
  async () => {
    const response = await poolStatsService.get_all_pool_metrics();
    return response ;
  }
);

export const deposit = createAsyncThunk(
  "strategy/deposit",
  async (
    {
      amount,
      strategyId,
      ledger,
      agent,
    }: {
      amount: bigint;
      strategyId: number;
      ledger: string;
      principal: Principal;
      agent: Agent;
    },
    { getState, rejectWithValue }
  ) => {
    const strategiesService = (getState() as RootState).strategies.service.data;
    if (!strategiesService)
      return rejectWithValue("Strategies service not inited");
    return await userService.accept_investment(
      strategyId,
      ledger,
      amount,
      agent
    );
  }
);

export const withdraw = createAsyncThunk(
  "strategy/withdraw",
  async (
    {
      amount,
      strategyId,
      ledger,
      agent,
    }: {
      amount: bigint;
      strategyId: number;
      ledger: string;
      principal: Principal;
      agent: Agent;
    },
    { getState, rejectWithValue }
  ) => {
    const strategiesService = (getState() as RootState).strategies.service.data;
    if (!strategiesService)
      return rejectWithValue("Strategies service not inited");
    return await userService.withdraw(strategyId, ledger, amount, agent);
  }
);

export const resetPools = createAction("strategy/resetPools");

const strategySlice = createSlice({
  name: "strategy",
  initialState: {
    fetchPools: {
      status: Status.IDLE,
      pools: [],
    },
    deposit: {
      status: Status.IDLE,
    },
    withdraw: {
      status: Status.IDLE,
    },
  } as {
    fetchPools: {
      status: Status;
      error?: string;
      pools: [string, PoolMetrics][];
    };
    deposit: {
      status: Status;
      error?: string;
    };
    withdraw: {
      status: Status;
      error?: string;
    };
  },
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(fetchPools.pending, (state) => {
        state.fetchPools.status = Status.LOADING;
      })
      .addCase(fetchPools.fulfilled, (state, action) => {
        state.fetchPools.status = Status.SUCCEEDED;
        state.fetchPools.pools = action.payload;
      })
      .addCase(fetchPools.rejected, (state, action) => {
        state.fetchPools.status = Status.FAILED;
        state.fetchPools.error = action.error.message;
      })
      .addCase(resetPools, (state) => {
        state.fetchPools.status = Status.IDLE;
        state.fetchPools.pools = [];
      })
      .addCase(deposit.pending, (state) => {
        state.deposit.status = Status.LOADING;
      })
      .addCase(deposit.fulfilled, (state) => {
        state.deposit.status = Status.SUCCEEDED;
      })
      .addCase(deposit.rejected, (state, action) => {
        state.deposit.status = Status.FAILED;
        state.deposit.error = action.error.message;
      })
      .addCase(withdraw.pending, (state) => {
        state.withdraw.status = Status.LOADING;
      })
      .addCase(withdraw.fulfilled, (state) => {
        state.withdraw.status = Status.SUCCEEDED;
      })
      .addCase(withdraw.rejected, (state, action) => {
        state.withdraw.status = Status.FAILED;
        state.withdraw.error = action.error.message;
      });
  },
});

export const strategyReducer = strategySlice.reducer;
