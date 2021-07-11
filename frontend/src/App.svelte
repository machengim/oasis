<script lang="ts">
  import { Router, Route, Link } from "svelte-routing";
  import Tailwind from "./components/Tailwind.svelte";
  import Header from "./sections/Header.svelte";
  import Notification from "./components/Notification.svelte";
  import Home from "./pages/Home.svelte";
  import Login from "./pages/Login.svelte";
  import Setup from "./pages/Setup.svelte";
  import Error404 from "./pages/Error404.svelte";

  export let path: string;

  const renderPage = (path: string) => {
    switch (path) {
      case "/":
        return `<Home />`;
      case "/setup":
        return `<Setup />`;
      case "/login":
        return `<Login />`;
      default:
        return `<Error404 />`;
    }
  };
</script>

<main class="relative">
  <Router url={path}>
    <Header {path} />

    <Notification />

    {@html renderPage(path)}

    <Route path="login" component={Login} />
    <Route path="setup" component={Setup} />
    <Route path="/"><Home /></Route>
  </Router>
</main>
