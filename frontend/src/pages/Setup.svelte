<script lang="ts">
  import { t, isLoading as isLoadingI18N, locale } from "svelte-i18n";
  import { useNavigate } from "svelte-navigator";
  import Spinner from "../components/Spinner.svelte";
  import Button from "../components/Button.svelte";
  import DirBrowser from "../sections/DirBrowser.svelte";
  import * as api from "../utils/api";
  import type { ISetupRequest } from "../utils/types";
  import {
    getSitename,
    getLang,
    siteStore,
    setNotification,
    sectionStore,
  } from "../utils/store";

  const navigate = useNavigate();
  let sitename = getSitename();
  let language = getLang();
  let username = "";
  let password = "";
  let selectedDir = "";
  let form: HTMLFormElement;
  let isLoading = false;
  let isNoStorageError = false;
  let isOpenDirBrowser = false;

  sectionStore.set("setup");

  $: locale.set(language);

  const onConfirm = async (e: Event) => {
    e.preventDefault();

    if (!validateForm()) {
      return;
    }

    isLoading = true;
    const result = await sendSetupRequest();
    isLoading = false;

    if (result) {
      navigate("/login");
    }
  };

  const validateForm = (): boolean => {
    if (!form.checkValidity()) {
      form.reportValidity();
      return false;
    }

    if (!selectedDir || selectedDir.trim().length === 0) {
      isNoStorageError = true;
      return false;
    }

    return true;
  };

  const setLang = (e: any) => {
    language = e.target.value;
  };

  const sendSetupRequest = async (): Promise<boolean> => {
    const payload: ISetupRequest = {
      sitename,
      username,
      password,
      language,
      storage: encodeURIComponent(selectedDir),
    };

    try {
      await api.post("/api/sys/setup", payload, false);
      updateSiteStore();
      setNotification("success", "Launched successfully.");
    } catch (e) {
      console.error(e);
      if (e.message === "409") {
        setNotification("error", "Username existed.");
      } else {
        setNotification("error", "Launch failed");
      }
      return false;
    }

    return true;
  };

  const updateSiteStore = () => {
    const site = $siteStore;
    site.language = language;
    site.name = sitename;
    siteStore.set(site);
  };
</script>

{#if $isLoadingI18N}
  <Spinner />
{:else}
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
        class="w-96 mx-auto mt-28 bg-gray-50 shadow rounded-lg flex flex-col items-center p-8"
      >
        <div class="text-xl font-bold mb-8 text-gray-700">
          {$t("component.setup.title")}
        </div>
        <div class="w-full grid grid-cols-4 mb-4">
          <div>{$t("form.language")}:</div>
          <div class="col-span-3">
            <!-- svelte-ignore a11y-no-onchange -->
            <select class="ml-2 px-2 border bg-gray-50" on:change={setLang}>
              <option value="en" selected={language === "en"}>English</option>
              <option value="cn" selected={language === "cn"}>中文</option>
            </select>
          </div>
        </div>
        <div class="w-full grid grid-cols-4 mb-4">
          <div>{$t("form.sitename")}:</div>
          <div class="col-span-3">
            <input
              required
              minLength={1}
              maxLength={16}
              class="ml-2 w-40 border rounded focus:outline-none px-2"
              bind:value={sitename}
            />
          </div>
        </div>
        <div class="w-full grid grid-cols-4 mb-4">
          <div>{$t("form.username")}:</div>
          <div class="col-span-3">
            <input
              required
              minLength={1}
              maxLength={16}
              class="ml-2 w-40 border rounded focus:outline-none px-2"
              bind:value={username}
            />
          </div>
        </div>
        <div class="w-full grid grid-cols-4 mb-4">
          <div>{$t("form.password")}:</div>
          <div class="col-span-3">
            <input
              required
              type="password"
              minLength={6}
              maxLength={16}
              class="ml-2 w-40 border rounded focus:outline-none px-2"
              bind:value={password}
            />
          </div>
        </div>
        <div class="w-full grid grid-cols-4 mb-12">
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
        <div class="mb-2">
          <Button
            value={isLoading ? $t("button.launching") : $t("button.launch")}
            onClick={onConfirm}
            disabled={isLoading}
            size="big"
            color="blue"
            type="submit"
          />
        </div>
      </div>
    </form>
  </div>
{/if}
