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
    ISiteFull,
  } from "./utils/types";
  import Spinner from "./components/Spinner.svelte";
  import {
    siteStore,
    setNotification,
    userStore,
    titleStore,
    clickStore,
    setUserExpire,
  } from "./utils/store";
  import { compareVersion, getLocale } from "./utils/util";
  import UpdateModal from "./modals/UpdateModal.svelte";
  import UploadList from "./sections/UploadList.svelte";
  import ForgotPassword from "./pages/ForgotPassword.svelte";
  import ResetPassword from "./pages/ResetPassword.svelte";

  let language = "";
  let isLoading = true;
  let showUpdateModal = false;
  let updateInfo: IUpdateAppInfo;

  const unsubscribeSite = siteStore.subscribe((site) => {
    if (site) {
      language = site.language;
    }
  });

  const unsubscribeUser = userStore.subscribe(async (user) => {
    if (user && user.permission === 9) {
      await Promise.all([checkUpdate(), fetchSiteFullConfig()]);
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
    unsubscribeTitle();
  });

  $: if (language) {
    locale.set(language);
  }

  $: if (updateInfo) {
    processUpdateInfo();
  }

  const fetchSiteFullConfig = async () => {
    isLoading = true;
    const endpoint = "/api/sys/config?mode=full";
    try {
      const site: ISiteFull = await api.get(endpoint, "json");
      siteStore.set(site);
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.read_site_error"));
    } finally {
      isLoading = false;
    }
  };

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
      const [site, user] = await Promise.all([siteReq, tokenReq]);
      if (site) {
        siteStore.set({ ...site, storage: "" });
        titleStore.set(site.name);
        userStore.set(user);
      }
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.read_site_error"));
    } finally {
      isLoading = false;
    }

    // Check if need to refresh token every 3 mins
    // Condition is user in store and the token is almost expired (5 mins)
    setInterval(async () => {
      if (needRefreshToken()) {
        const user = await api.refresh_token();
        if (
          user.username === $userStore.username &&
          user.permission === $userStore.permission &&
          user.expire > $userStore.expire
        ) {
          setUserExpire(user.expire);
        }
      }
    }, 1000 * 180);
  };

  const needRefreshToken = (): boolean => {
    if (!$userStore) return false;

    let currentTime = new Date().getTime() / 1000;
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

  const onClickEvent = () => {
    clickStore.set(new Date().getTime());
  };
</script>

{#if isLoading}
  <Spinner />
{:else}
  <main class="min-h-screen" on:click={onClickEvent}>
    <Router>
      <Header />
      <Notification />
      <UploadList />

      {#if showUpdateModal}
        <UpdateModal onClose={() => (showUpdateModal = false)} {updateInfo} />
      {/if}
      <Route path="/login" component={Login} />
      <Route path="/setup" component={Setup} />
      <Route path="/files" component={Files} primary={false} />
      <Route path="/files/*" component={Files} primary={false} />
      <Route path="/settings" component={Settings} />
      <Route path="/profile" component={Profile} />
      <Route
        path="/forgot-password"
        component={ForgotPassword}
        primary={false}
      />
      <Route
        path="/reset-password/*"
        component={ResetPassword}
        primary={false}
      />
      <Route path="/" component={Home} primary={false} />
    </Router>
  </main>
{/if}
