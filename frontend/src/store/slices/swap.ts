import { createSlice, createAction } from "@reduxjs/toolkit";

export const setSlippage = createAction<number>("swap/setSlippage");

const swapSlice = createSlice({
  name: "swap",
  initialState: {
    slippage: 1,
  } as {
    slippage: number;
  },
  reducers: {},
  extraReducers: (builder) => {
    builder.addCase(setSlippage, (state, action) => {
      state.slippage = action.payload;
    });
  },
});

export const swapReducer = swapSlice.reducer;
