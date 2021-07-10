import React from 'react';
import ReactDom from 'react-dom';
import { Router } from 'react-router';
import { createBrowserHistory } from 'history';
import Home from './pages/Home';
import Login from './pages/Login';
import Setup from './pages/Setup';
import Error404 from './pages/Error404';
import { BrowserRouter, Switch, Route } from 'react-router-dom';

const history = createBrowserHistory();

ReactDom.render(
  <BrowserRouter history={history}>
    <App />
  </BrowserRouter>,
  document.getElementById('root')
);

function App() {
  const path = window.location.pathname;

  const renderPage = (path: string) => {
    switch (path) {
      case '/':
        return <Home />;
      case '/setup':
        return <Setup />;
      case '/login':
        return <Login />;
      default:
        return <Error404 />;
    }
  };

  return (
    <>
      {renderPage(path)}

      <Switch>
        <Route path="/about">
          <About />
        </Route>
        <Route path="/users">
          <Users />
        </Route>
      </Switch>
    </>
  );
}

function About() {
  return <h2>About</h2>;
}

function Users() {
  return <h2>Users</h2>;
}
