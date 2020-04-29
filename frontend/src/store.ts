import { configureStore, createSlice } from '@reduxjs/toolkit';

export type IActivity = { name: string };
export type IStateActivities = { activities: IActivity[] };
const initialState: IStateActivities = { activities: [] };
const activities = createSlice({
  name: 'activities',
  initialState,
  reducers: {},
});

export const store = configureStore({
  reducer: {
    activities: activities.reducer,
  },
});

export type IRootState = ReturnType<typeof store.getState>;
// export type AppDispatch = typeof store.dispatch;
