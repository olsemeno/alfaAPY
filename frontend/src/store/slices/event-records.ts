import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { eventRecordsService, EventRecordType, EventRecord } from "../../services";

export const fetchEventRecords = createAsyncThunk<
  Array<EventRecord>,
  { user?: string; type?: EventRecordType; from?: string; to?: string } | undefined
>("EventRecords/fetchEventRecords", async (params) => {
  const response = await eventRecordsService.getEventRecords(params);
  return response;
});

export const EventRecordsSlice = createSlice({
  name: "EventRecords",
  initialState: {
    status: "idle",
  } as {
    status: "idle" | "loading" | "succeeded" | "failed";
    eventRecords?: Array<EventRecord>;
  },
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(fetchEventRecords.pending, (state) => {
        state.status = "loading";
      })
      .addCase(fetchEventRecords.fulfilled, (state, action) => {
        state.status = "succeeded";
        state.eventRecords = action.payload;
      })
      .addCase(fetchEventRecords.rejected, (state) => {
        state.status = "failed";
      });
  },
});

export const eventRecordsReducer = EventRecordsSlice.reducer;
