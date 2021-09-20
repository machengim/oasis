<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import { init, locale } from "svelte-i18n";
  import Tailwind from "./components/Tailwind.svelte";
  import "./i18n";
  import Notification from "./sections/Notification.svelte";
  import Header from "./sections/Header.svelte";
  import Login from "./pages/Login.svelte";
  import Setup from "./pages/Setup.svelte";
  import Home from "./pages/Home.svelte";
  import Files from "./pages/Files.svelte";
  import * as api from "./utils/api";
  import type { ISiteBasic } from "./utils/types";
  import Spinner from "./components/Spinner.svelte";
  import { setNotification } from "./utils/store";

  let isLoading = true;

  const initApp = async () => {
    init({
      fallbackLocale: "en",
      initialLocale: "en",
    });

    try {
      const site: ISiteBasic = await api.get("/api/site/basic", "json");
      locale.set(site.language);
    } catch (e) {
      console.error(e);
      setNotification("error", "Cannot read site info");
    }
  };

  initApp().then(() => (isLoading = false));
</script>

{#if isLoading}
  <Spinner />
{:else}
  <main>
    <Router>
      <Header />
      <Notification />

      <Route path="/login" component={Login} />
      <Route path="/setup" component={Setup} />
      <Route path="/files" component={Files} />
      <Route path="/files/*" component={Files} />
      <Route path="/" component={Home} />
    </Router>
  </main>
{/if}
