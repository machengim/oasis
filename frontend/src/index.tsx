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
  console.log('location: ', window.location);
  const path = window.location.pathname;
  const urlParams = new URLSearchParams(window.location.search);
  const page = urlParams.get('page');

  const matchList = [
    {
      path: '/',
      page: 'home'
    },
    {
      path: '/setup',
      page: 'setup'
    },
    {
      path: '/login',
      page: 'login'
    }
  ];

  const getServerRoutePage = () => {
    console.log('path ' + path + ', page: ' + page);
    let filteredList = matchList.filter((v) => v.path === path);
    return filteredList.length > 0 ? filteredList[0].page : null;
  };

  const serverRoute = getServerRoutePage();
  console.log(serverRoute);

  const renderPage = (serverRoute: string) => {
    console.log('got page: ', page);
    switch (serverRoute) {
      case 'home':
        return <Home />;
      case 'setup':
        history.replace('/setup');
        return <Setup />;
      case 'login':
        return <Login />;
      default:
        return <Error404 />;
    }
  };

  return (
    <>
      {serverRoute && renderPage(serverRoute)}

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
