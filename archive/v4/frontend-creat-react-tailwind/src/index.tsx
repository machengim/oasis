import React from 'react';
import ReactDOM from 'react-dom';
import { Router } from 'react-router';
import { BrowserRouter, Switch, Route } from 'react-router-dom';
import { createBrowserHistory } from 'history';
import Home from './pages/Home';
import Login from './pages/Login';
import Setup from './pages/Setup';
import './index.css';

const history = createBrowserHistory();

ReactDOM.render(
  <React.StrictMode>
    <Router history={history}>
      <App />
    </Router>
  </React.StrictMode>,
  document.getElementById('root')
);

function App() {
  return (
    <BrowserRouter>
      <Switch>
        <Route path="/login">
          <Login />
        </Route>
        <Route path="/setup">
          <Setup />
        </Route>
        <Route path="/">
          <Home />
        </Route>
      </Switch>
    </BrowserRouter>
  );
}
