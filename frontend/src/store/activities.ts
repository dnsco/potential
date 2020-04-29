import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import axios from 'axios';
import type { AppDispatch, AppThunk } from '../store';

export type Status = 'waiting' | 'fetching' | 'success' | 'failed';
const WAITING = 'waiting';
const FETCHING = 'fetching';
const SUCCESS = 'success';
const FAILED = 'failed';

export type Activity = { name: string };
export type StateActivities = { activities: Activity[]; status: Status };
const initialState: StateActivities = { activities: [], status: WAITING };
export const activities = createSlice({
  name: 'activities',
  initialState,
  reducers: {
    setStatusFetching(state: StateActivities) {
      return { ...state, status: FETCHING };
    },
    setStatusFailed(state: StateActivities) {
      return { ...state, status: FAILED };
    },
    getActivitesSuccess(
      state: StateActivities,
      action: PayloadAction<Activity[]>
    ) {
      return {
        ...state,
        activities: action.payload,
        status: SUCCESS,
      };
    },
  },
});

const {
  getActivitesSuccess,
  setStatusFailed,
  setStatusFetching,
} = activities.actions;

export const fetchActivities = (): AppThunk => (dispatch: AppDispatch) => {
  dispatch(setStatusFetching());
  axios.get<Activity[]>('http://localhost:8080').then(
    ({ data }) => dispatch(getActivitesSuccess(data)),
    () => dispatch(setStatusFailed())
  );
};
