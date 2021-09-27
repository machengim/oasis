<script lang="ts">
  import { onMount } from "svelte";
  import { t } from "svelte-i18n";
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import Spinner from "../components/Spinner.svelte";
  import * as api from "../utils/api";
  import { checkMobile } from "../utils/util";

  export let onClose: () => void;
  export let filename: string;
  export let filePath: string;
  // Clipboard permission may be unavailabel on mobiles.
  const isMobile = checkMobile();
  let textarea: HTMLTextAreaElement;
  let link: string;
  let copied = false;
  let isLoading = false;

  $: if (copied) {
    setTimeout(() => {
      copied = false;
      clearSelection();
    }, 2000);
  }

  onMount(() => {
    requestShareLink();
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

  const copyLink = async () => {
    if (!textarea || !link) return;

    textarea.select();
    textarea.setSelectionRange(0, 99999);
    await navigator.clipboard.writeText(textarea.value);
    copied = true;
  };

  const clearSelection = () => {
    if (window.getSelection) {
      window.getSelection().removeAllRanges();
    }
  };
</script>

<Modal {onClose} title="File share link">
  {#if isLoading}
    <Spinner />
  {:else}
    <div class="p-4 text-lg">
      <div>File: {filename}</div>
      <div class="mb-4">Expire: 6 hours</div>
      <textarea
        bind:this={textarea}
        readonly
        class="border rounded w-full p-2 overflow-y-hidden focus:outline-none"
        on:click={copyLink}>{link}</textarea
      >
    </div>
    <div class="w-full p-4 flex flex-row justify-end">
      {#if !isMobile}
        <Button
          onClick={copyLink}
          color={"blue"}
          value={copied ? "Link copied!" : "Copy link"}
          className="mr-4"
        />
      {/if}
      <Button onClick={onClose} value="Close" />
    </div>
  {/if}
</Modal>
