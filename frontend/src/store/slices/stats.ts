import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";

export const fetchStats = createAsyncThunk("stats/fetch", async () => {
  //   const response = await eventRecordsService.getEventRecords(params);
  //   return response;
});

export const statsSlice = createSlice({
  name: "stats",
  initialState: {
    stats: {},
    status: "idle",
  },
  reducers: {},
  extraReducers: (builder) => {
    builder.addCase(fetchStats.fulfilled, (state, action) => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      state.stats = action.payload as any;
    });
  },
});

export const statsReducer = statsSlice.reducer;
