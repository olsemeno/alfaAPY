import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { paymentsService, PaymentType, Payment } from "../../services";

export const fetchPayments = createAsyncThunk<
  Array<Payment>,
  { user?: string; type?: PaymentType; from?: string; to?: string } | undefined
>("payments/fetchPayments", async (params) => {
  const response = await paymentsService.getPayments(params);
  return response;
});

export const paymentsSlice = createSlice({
  name: "payments",
  initialState: {
    status: "idle",
  } as {
    status: "idle" | "loading" | "succeeded" | "failed";
    payments?: Array<Payment>;
  },
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(fetchPayments.pending, (state) => {
        state.status = "loading";
      })
      .addCase(fetchPayments.fulfilled, (state, action) => {
        state.status = "succeeded";
        state.payments = action.payload;
      })
      .addCase(fetchPayments.rejected, (state) => {
        state.status = "failed";
      });
  },
});

export const paymentsReducer = paymentsSlice.reducer;
