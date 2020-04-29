import React, { useEffect } from 'react';
import './App.css';
import { useDispatch, useSelector } from 'react-redux';
import { fetchActivities } from './store/activities';
import { RootState } from './store';

function App() {
  const dispatch = useDispatch();
  const { activities, status } = useSelector(
    (state: RootState) => state.activities
  );
  useEffect(() => {
    dispatch(fetchActivities());
  }, [dispatch]);

  return (
    <div className="App">
      <header className="App-header">
        {activities.length} activities – {status}
      </header>
    </div>
  );
}

export default App;
