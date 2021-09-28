<script lang="ts">
  import { onMount } from "svelte";
  import { t } from "svelte-i18n";
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import type {
    ILink,
    IUpdateAppNeedRespose,
    IUpdateAppInfo,
  } from "../utils/types";
  import { setNotification, siteStore } from "../utils/store";
  import * as api from "../utils/api";
  import { compareVersion } from "../utils/util";

  export let onClose: () => void;
  export let setUpdateInfo: (info: IUpdateAppInfo) => void;
  let isLoading = false;
  let website: ILink = null;
  let repo: ILink = null;
  let links: Array<ILink> = [];

  onMount(() => initLinks());

  const initLinks = () => {
    website = {
      name: "chengma.dev/oasis",
      url: "https://chengma.dev/oasis",
    };

    repo = {
      name: "Github repo",
      url: "https://github.com/machengim/oasis",
    };

    links = [
      { name: "Svelte", url: "https://svelte.dev" },
      { name: "Rocket", url: "https://rocket.rs" },
      { name: "Tailwind", url: "https://tailwindcss.com/" },
      { name: "PDFjs", url: "https://mozilla.github.io/pdf.js/" },
      { name: "Plyr", url: "https://plyr.io/" },
    ];
  };

  const checkUpdate = async () => {
    try {
      const response: IUpdateAppNeedRespose = await api.get("/api/sys/update");
      const updateInfo: IUpdateAppInfo = await api.get(response.url);
      if (
        $siteStore &&
        compareVersion(updateInfo.version, $siteStore.version)
      ) {
        setUpdateInfo(updateInfo);
      }
    } catch (e) {
      console.error(e);
      setNotification("error", "Cannot get update info");
    }

    return false;
  };
</script>

<Modal {onClose} title={$t("modal.about.title")}>
  <div class="p-4 text-lg">
    <div>
      <span>{$t("modal.about.version")}: </span>
      <span class="ml-2 px-1">0.1</span>
    </div>
    {#if website}
      <div>
        {$t("modal.about.website")}:
        <a
          href={website.url}
          target="_blank"
          class="px-1 ml-2 rounded-sm hover:bg-blue-400 hover:text-white"
        >
          {website.name}
        </a>
      </div>
    {/if}
    {#if repo}
      <div>
        {$t("modal.about.repo")}:
        <a
          href={repo.url}
          target="_blank"
          class="px-1 ml-2 rounded-sm hover:bg-blue-400 hover:text-white"
        >
          {repo.name}
        </a>
      </div>
    {/if}
    <div class="mt-4">{$t("modal.about.thanks")} ❤</div>
    <div>
      {#each links as link}
        <a
          href={link.url}
          target="_blank"
          class="px-1 rounded-sm hover:bg-blue-400 hover:text-white"
          >{link.name}</a
        >
        |{" "}
      {/each}
      ...
    </div>
  </div>

  <div class="w-full p-4 flex flex-row justify-end">
    <Button
      onClick={checkUpdate}
      color="blue"
      value={isLoading
        ? $t("button.check_updating")
        : $t("button.check_update")}
      className="mr-4"
      disabled={isLoading}
    />
    <Button onClick={onClose} value={$t("button.close")} />
  </div>
</Modal>
