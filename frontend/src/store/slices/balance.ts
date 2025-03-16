import { Principal } from "@dfinity/principal";
import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { ICRC1 } from "../../idl/icrc1_oracle";
import { icrc1OracleService } from "../../services";
import { Status } from "../types";

export const fetchBalance = createAsyncThunk(
  "balances/fetchBalance",
  async ({
    principal,
    canister,
  }: {
    principal: Principal;
    canister: ICRC1;
  }) => {
    const response = await icrc1OracleService.getBalance(principal, canister);
    return response;
  }
);

const balanceSlice = createSlice({
  name: "balances",
  initialState: {
    status: Status.IDLE,
  } as {
    status: Status;
    error?: string;
  },
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(fetchBalance.pending, (state) => {
        state.status = Status.LOADING;
      })
      .addCase(fetchBalance.fulfilled, (state) => {
        state.status = Status.SUCCEEDED;
      })
      .addCase(fetchBalance.rejected, (state, action) => {
        state.status = Status.FAILED;
        state.error = action.error.message;
      });
  },
});

export const balanceReducer = balanceSlice.reducer;
