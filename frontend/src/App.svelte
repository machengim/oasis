<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Router, Route } from "svelte-navigator";
  import { init, locale, t } from "svelte-i18n";
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
  import type {
    ISiteBrief,
    IUpdateAppNeedRespose,
    IUpdateAppInfo,
  } from "./utils/types";
  import Spinner from "./components/Spinner.svelte";
  import {
    siteStore,
    setNotification,
    userStore,
    titleStore,
  } from "./utils/store";
  import { compareVersion, getLocale } from "./utils/util";
  import UpdateModal from "./modals/UpdateModal.svelte";

  let language = "";
  let sitename = "";
  let isLoading = true;
  let showUpdateModal = false;
  let updateInfo: IUpdateAppInfo;

  const unsubscribeSite = siteStore.subscribe((site) => {
    if (site) {
      sitename = site.name;
      language = site.language;
    }
  });

  const unsubscribeUser = userStore.subscribe((user) => {
    if (user && user.permission === 9) {
      checkUpdate();
    }
  });

  const unsubscribeTitle = titleStore.subscribe((title) => {
    if (title) {
      document.title = title;
    }
  });

  onMount(() => initApp());

  onDestroy(() => {
    unsubscribeSite();
    unsubscribeUser();
    unsubscribeTitle;
  });

  $: if (language) {
    locale.set(language);
  }

  // $: document.title = sitename;

  $: if (updateInfo) {
    processUpdateInfo();
  }

  const initApp = async () => {
    init({
      fallbackLocale: "en",
      initialLocale: getLocale(),
    });

    try {
      const siteReq: Promise<ISiteBrief> = api.get(
        "/api/sys/config?mode=brief"
      );
      const tokenReq = api.refresh_token();
      const values: [ISiteBrief, void] = await Promise.all([siteReq, tokenReq]);
      const site = values[0];
      if (site) {
        siteStore.set({ ...site, storage: "" });
      }
      titleStore.set(site.name);
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.read_site_error"));
    }

    isLoading = false;

    // Check if need to refresh token every 2 mins
    // Condition is user in store and the token is almost expired (5 mins)
    setInterval(async () => {
      if (needRefreshToken()) {
        await api.refresh_token();
      }
    }, 1000 * 120);
  };

  const needRefreshToken = (): boolean => {
    if (!$userStore) return false;

    let currentTime = new Date().getTime();
    return $userStore.expire - currentTime <= 300;
  };

  const checkUpdate = async () => {
    try {
      const response: IUpdateAppNeedRespose = await api.get("/api/sys/update");
      if (response.need) {
        updateInfo = await api.get(response.url);
      }
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.get_update_info_error"));
    }

    return false;
  };

  const processUpdateInfo = async () => {
    const site = $siteStore;
    if (!site || !updateInfo) return;
    if (compareVersion(updateInfo.version, site.version) > 0) {
      showUpdateModal = true;
    }
  };
</script>

{#if isLoading}
  <Spinner />
{:else}
  <main>
    <Router>
      <Header />
      <Notification />

      {#if showUpdateModal}
        <UpdateModal onClose={() => (showUpdateModal = false)} {updateInfo} />
      {/if}
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
