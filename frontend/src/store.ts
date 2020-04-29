import { Action, configureStore, ThunkAction } from '@reduxjs/toolkit';
import { activities } from './store/activities';

export const store = configureStore({
  reducer: {
    activities: activities.reducer,
  },
});
export type RootState = ReturnType<typeof store.getState>;
export type AppThunk = ThunkAction<void, RootState, unknown, Action<string>>;
export type AppDispatch = typeof store.dispatch;
