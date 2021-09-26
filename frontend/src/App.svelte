<script lang="ts">
  import { onMount, onDestroy } from "svelte";
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
  import Settings from "./pages/Settings.svelte";
  import Profile from "./pages/Profile.svelte";
  import * as api from "./utils/api";
  import type { ISiteBrief } from "./utils/types";
  import Spinner from "./components/Spinner.svelte";
  import { siteStore, setNotification, userStore } from "./utils/store";
  import { getLocale } from "./utils/util";

  let isLoading = true;
  let language = "";
  let sitename = "";

  const unsubscribeSite = siteStore.subscribe((site) => {
    if (site) {
      sitename = site.name;
      language = site.language;
    }
  });

  onMount(() => initApp());

  onDestroy(() => {
    unsubscribeSite();
  });

  $: if (language) {
    locale.set(language);
  }

  $: document.title = sitename;

  const initApp = async () => {
    init({
      fallbackLocale: "en",
      initialLocale: getLocale(),
    });

    try {
      const siteReq = api.get("/api/sys/config?mode=brief", "json");
      const tokenReq = api.refresh_token();
      Promise.all([siteReq, tokenReq]).then((values: [ISiteBrief, void]) => {
        const site = values[0];
        if (site) {
          siteStore.set({ ...site, storage: "" });
        }
      });
    } catch (e) {
      console.error(e);
      setNotification("error", "Cannot read site info");
    }

    await api.refresh_token();
    isLoading = false;

    // Check if need to refresh token every 2 mins
    // Condition is no user in store or the token is almost expired (5 mins)
    setInterval(async () => {
      if (needRefresh()) {
        await api.refresh_token();
      }
    }, 1000 * 120);

    const needRefresh = (): boolean => {
      let user = $userStore;
      if (!user) return true;

      let currentTimeUtc = new Date().getTime();
      if (user.expire - currentTimeUtc <= 300) {
        return true;
      }

      return false;
    };
  };
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
      <Route path="/files" component={Files} primary={false} />
      <Route path="/files/*" component={Files} primary={false} />
      <Route path="/settings" component={Settings} />
      <Route path="/profile" component={Profile} />
      <Route path="/" component={Home} primary={false} />
    </Router>
  </main>
{/if}
