import { createSlice, createAsyncThunk } from "@reduxjs/toolkit";
import { icrc1OracleService } from "../../services";
import { Status } from "../types";
import { Principal } from "@dfinity/principal";
import { ICRC1 } from "../../idl/icrc1_oracle";
import { fetchBalance } from "./balance";

export const fetchBalances = createAsyncThunk(
  "balances/fetchBalances",
  async ({
    principal,
    canisters,
  }: {
    principal: Principal;
    canisters: ICRC1[];
  }) => {
    const response = await icrc1OracleService.getBalances(principal, canisters);
    return response;
  }
);

const balancesSlice = createSlice({
  name: "balances",
  initialState: {
    balances: {},
    status: Status.IDLE,
  } as {
    balances: Record<
      string,
      { balance: string; usdBalance: string; token: ICRC1 }
    >;
    status: Status;
    error?: string;
  },
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(fetchBalances.pending, (state) => {
        state.status = Status.LOADING;
      })
      .addCase(fetchBalances.fulfilled, (state, action) => {
        state.status = Status.SUCCEEDED;
        state.balances = action.payload
          ? action.payload?.reduce((acc, value) => {
              return { ...acc, [value.token.ledger]: value };
            }, {})
          : {};
      })
      .addCase(fetchBalance.fulfilled, (state, action) => {
        state.status = Status.SUCCEEDED;
        state.balances = {
          ...state.balances,
          [action.payload.token.ledger]: action.payload,
        };
      })
      .addCase(fetchBalances.rejected, (state, action) => {
        state.status = Status.FAILED;
        state.error = action.error.message;
      });
  },
});

export const balancesReducer = balancesSlice.reducer;
