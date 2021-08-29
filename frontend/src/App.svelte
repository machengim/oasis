<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import Tailwind from "./components/Tailwind.svelte";
  import Notification from "./sections/Notification.svelte";
  import Header from "./sections/Header.svelte";
  import Login from "./pages/Login.svelte";
  import Setup from "./pages/Setup.svelte";
  import Home from "./pages/Home.svelte";
</script>

<main>
  <Router>
    <Header />
    <Notification />

    <Route path="/login" component={Login} />
    <Route path="/setup" component={Setup} />
    <Route path="/" component={Home} />
  </Router>
</main>
