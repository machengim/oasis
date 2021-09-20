<script lang="ts">
  import { onMount } from "svelte";
  import { t, isLoading as isLoadingI18N, locale } from "svelte-i18n";
  import { useNavigate } from "svelte-navigator";
  import Spinner from "../components/Spinner.svelte";
  import Button from "../components/Button.svelte";
  import DirBrowser from "../sections/DirBrowser.svelte";
  import * as api from "../utils/api";
  import { setNotification, sectionStore } from "../utils/store";
  import type { ISiteFull, IUpdateConfigRequest } from "../utils/types";

  const navigate = useNavigate();
  let isLoading = false;
  let form: HTMLFormElement;
  let isOpenDirBrowser = false;
  let selectedDir = null;
  let isNoStorageError = false;
  let language: string;
  let update_freq: string;

  sectionStore.set("settings");

  onMount(() => fetchConfig());

  $: if (language) {
    locale.set(language);
  }

  const fetchConfig = async () => {
    isLoading = true;
    const endpoint = "/api/sys/config?mode=full";
    try {
      const site: ISiteFull = await api.get(endpoint, "json");
      language = site.language;
      update_freq = site.update_freq;
      selectedDir = site.storage;
    } catch (e) {
      console.error(e);
      setNotification("error", "Cannot read site info");
    }

    isLoading = false;
  };

  const onConfirm = async (e: Event) => {
    e.preventDefault();

    if (!validateForm()) {
      return;
    }

    isLoading = true;
    try {
      await sendUpdateConfigRequest();
      navigate("/");
    } catch (e) {
      console.error(e);
      setNotification("error", "Cannot update config");
    }

    isLoading = false;
  };

  const validateForm = (): boolean => {
    if (!selectedDir || selectedDir.trim().length === 0) {
      isNoStorageError = true;
      return false;
    }

    return true;
  };

  const sendUpdateConfigRequest = async () => {
    const payload: IUpdateConfigRequest = {
      language,
      storage: encodeURIComponent(selectedDir),
      update_freq,
    };

    const endpoint = "/api/sys/config";
    try {
      await api.put(endpoint, payload, false);
    } catch (e) {
      throw e;
    }
  };

  const setLang = (e: any) => {
    language = e.target.value;
  };

  const setUpdateFreq = (e: any) => {
    update_freq = e.target.value;
  };

  const onCancel = () => {
    navigate(-1);
  };
</script>

<div class="absolute w-full">
  {#if isOpenDirBrowser}
    <DirBrowser
      onClose={() => (isOpenDirBrowser = false)}
      onSelect={(v) => {
        selectedDir = v;
        isNoStorageError = false;
      }}
    />
  {/if}
  <form on:submit={onConfirm} bind:this={form}>
    <div
      class="panel-width mx-auto mt-20 bg-gray-50 shadow rounded-lg flex flex-col items-center p-2 md:p-8"
    >
      <div class="text-xl font-bold mb-8 text-gray-700">
        {$t("component.settings.title")}
      </div>
      <div class="w-full grid grid-cols-4 mb-4">
        <div>{$t("component.settings.version")}:</div>
        <div class="col-span-3">
          <span class="ml-2 px-2">0.1</span>
        </div>
      </div>
      <div class="w-full grid grid-cols-4 mb-4">
        <div>{$t("component.settings.language")}:</div>
        <div class="col-span-3">
          <!-- svelte-ignore a11y-no-onchange -->
          <select class="ml-2 px-2 border bg-gray-50" on:change={setLang}>
            <option value="en" selected={language === "en"}>English</option>
            <option value="cn" selected={language === "cn"}>中文</option>
          </select>
        </div>
      </div>
      <div class="w-full grid grid-cols-4 mb-4">
        <div>{$t("component.settings.update")}:</div>
        <div class="col-span-3">
          <!-- svelte-ignore a11y-no-onchange -->
          <select class="ml-2 px-2 border bg-gray-50" on:change={setUpdateFreq}>
            <option value="daily" selected={update_freq === "daily"}
              >{$t("component.settings.daily")}</option
            >
            <option value="weekly" selected={update_freq === "weekly"}
              >{$t("component.settings.weekly")}</option
            >
            <option value="monthly" selected={update_freq === "monthly"}
              >{$t("component.settings.monthly")}</option
            >
            <option value="never" selected={update_freq === "never"}
              >{$t("component.settings.never")}</option
            >
          </select>
        </div>
      </div>
      <div class="w-full grid grid-cols-4 mb-8">
        <div>{$t("form.storage")}:</div>
        <div class="col-span-3 pl-2">
          {#if selectedDir}
            <Button
              value={$t("button.change")}
              size="small"
              onClick={() => (isOpenDirBrowser = true)}
            />
            <div class="mt-2 break-words">{selectedDir}</div>
          {:else}
            <Button
              value={$t("button.select")}
              size="small"
              onClick={() => (isOpenDirBrowser = true)}
            />
          {/if}
          {#if isNoStorageError}
            <div class="text-red-500">{$t("message.error.no_storage")}</div>
          {/if}
        </div>
      </div>
      <div class="mb-2 flex justify-center">
        <Button
          value={isLoading ? $t("button.changing") : $t("button.change")}
          onClick={onConfirm}
          disabled={isLoading}
          size="big"
          color="blue"
          type="submit"
          className="mr-8"
        />
        <Button value={$t("button.cancel")} onClick={onCancel} size="big" />
      </div>
    </div>
  </form>
</div>

<style>
  .panel-width {
    width: 24rem;
    max-width: 90vw;
  }
</style>
