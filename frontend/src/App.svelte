<script lang="ts">
  import { Router, Link, Route } from "svelte-navigator";
  import Tailwind from "./components/Tailwind.svelte";
  import Header from "./sections/Header.svelte";
  import Notification from "./sections/Notification.svelte";
  import UploadTasks from "./sections/UploadTasks.svelte";
  import Home from "./pages/Home.svelte";
  import Login from "./pages/Login.svelte";
  import Setup from "./pages/Setup.svelte";
  import Error404 from "./pages/Error404.svelte";
  import { clickEventStore } from "./utils/store";

  export let path: string;

  const clicked = () => {
    clickEventStore.set(+new Date());
  };

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

<main class="relative min-h-screen" on:click={clicked}>
  <Router url={path}>
    <Header {path} />

    <Notification />
    <UploadTasks />

    {@html renderPage(path)}

    <Route path="login" component={Login} />
    <Route path="setup" component={Setup} />
    <Route path="/"><Home /></Route>
  </Router>
</main>
