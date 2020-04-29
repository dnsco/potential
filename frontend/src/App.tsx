import React from 'react';
import './App.css';
import { connect } from 'react-redux';
import { IActivity, IRootState } from './store';

type IProps = { activities: IActivity[] };

function App({ activities }: IProps) {
  return (
    <div className="App">
      <header className="App-header">{activities.length} activities</header>
    </div>
  );
}

export default connect((state: IRootState) => {
  return { activities: state.activities.activities };
})(App);
