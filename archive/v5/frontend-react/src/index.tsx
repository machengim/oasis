import { useEffect, useContext } from "react";
import ReactDOM from "react-dom";
import { Router } from "react-router";
import { Switch, Route } from "react-router-dom";
import { createBrowserHistory } from "history";
import { AppContext, AppCtxProvider } from "./context";
import Header from "./sections/Header";
import Home from "./pages/Home";
import Login from "./pages/Login";
import Setup from "./pages/Setup";
import Error404 from "./pages/Error404";
import "./index.css";

ReactDOM.render(
  <Router history={createBrowserHistory()}>
    <AppCtxProvider>
      <App />
    </AppCtxProvider>
  </Router>,
  document.getElementById("root")
);

function App() {
  const path = window.location.pathname;
  const appContext = useContext(AppContext);
  const { sections, setSections } = appContext;

  useEffect(() => {
    if (sections.length === 0 && sections[0] !== path) {
      setSections([path]);
    }
  }, []);

  const renderPage = (path: string) => {
    switch (path) {
      case "/":
        return <Home />;
      case "/setup":
        return <Setup />;
      case "/login":
        return <Login />;
      default:
        return <Error404 />;
    }
  };

  return (
    <>
      <Header />
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
