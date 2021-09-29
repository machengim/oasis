<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { t } from "svelte-i18n";
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import Spinner from "../components/Spinner.svelte";
  import * as api from "../utils/api";
  import copy from "copy-to-clipboard";

  export let onClose: () => void;
  export let filename: string;
  export let filePath: string;
  let textarea: HTMLTextAreaElement;
  let link: string;
  let copied = false;
  let isLoading = false;
  let timeout: NodeJS.Timeout;

  $: if (copied) {
    timeout = setTimeout(() => {
      copied = false;
      clearSelection();
    }, 2000);
  }

  onMount(() => {
    requestShareLink();
  });

  onDestroy(() => {
    clearSelection();
    timeout = null;
  });

  $: if (link && textarea) {
    textarea.style.height = textarea.scrollHeight + "px";
  }

  const requestShareLink = async () => {
    isLoading = true;
    const endpoint = "/api/file/share";
    const expire = Math.floor(new Date().getTime() / 1000 + 6 * 60 * 60);
    const payload = {
      path: filePath.split("/").pop(),
      expire,
    };

    try {
      const response = await api.post(endpoint, payload, false);
      link =
        window.location.protocol +
        "//" +
        window.location.host +
        "/api/file/share?" +
        response;
    } catch (e) {
      console.error(e);
    }

    isLoading = false;
  };

  const copyLink = () => {
    if (!textarea || !link) return;
    textarea.select();
    textarea.setSelectionRange(0, 99999);

    copy(textarea.value);
    copied = true;
  };

  const clearSelection = () => {
    if (window.getSelection) {
      window.getSelection().removeAllRanges();
    }
  };
</script>

<Modal {onClose} title={$t("modal.file_share.title")}>
  {#if isLoading}
    <Spinner />
  {:else}
    <div class="p-4 text-lg">
      <div>{$t("modal.file_share.file")}: {filename}</div>
      <div class="mb-4">
        {$t("modal.file_share.valid_period")}: 6 {$t("modal.file_share.hours")}
      </div>
      <textarea
        id="textarea"
        bind:this={textarea}
        readonly
        class="border ta-height rounded w-full p-2 leading-normal overflow-y-auto focus:outline-none"
        on:click={copyLink}>{link}</textarea
      >
    </div>
    <div class="w-full p-4 flex flex-row justify-end">
      <Button
        onClick={copyLink}
        color={"blue"}
        value={copied ? $t("button.link_copied") : $t("button.copy_link")}
        className="mr-4"
      />
      <Button onClick={onClose} value={$t("button.close")} />
    </div>
  {/if}
</Modal>

<style>
  @media only screen and (min-width: 320px) {
    .ta-height {
      max-height: 15rem;
    }
  }
  @media only screen and (min-width: 768px) {
    .ta-height {
      max-height: 25rem;
    }
  }
</style>
