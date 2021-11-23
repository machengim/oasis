<script lang="ts">
  import Modal from "../components/Modal.svelte";
  import { useNavigate } from "svelte-navigator";
  import Icon from "../components/Icon.svelte";
  import { EIconColor, EIconType } from "../utils/enums";
  import type { IFile } from "../utils/types";
  import * as api from "../utils/api";
  import { setNotification, keywordsStore } from "../utils/store";
  import Spinner from "../components/Spinner.svelte";
  import FileIcon from "../components/FileIcon.svelte";
  import { onDestroy, onMount } from "svelte";

  export let onClose: () => void;
  const navigate = useNavigate();
  let inputElement: HTMLInputElement;
  let keyword: string;
  let fetchTimeout: NodeJS.Timeout;
  let searchResults: Array<IFile> = [];
  let isLoading = false;
  let loaded = false;
  let useHistoryKeyword = false;

  onMount(() => {
    if (inputElement) {
      inputElement.focus();
    }
  });

  onDestroy(() => {
    if (fetchTimeout) {
      clearTimeout(fetchTimeout);
    }
    keyword = null;
    searchResults = [];
  });

  $: if (keyword && keyword.trim().length > 0) {
    if (fetchTimeout) {
      clearTimeout(fetchTimeout);
    }

    if (useHistoryKeyword) {
      fetchSearchResults();
    } else {
      fetchTimeout = setTimeout(() => {
        fetchSearchResults();
      }, 1000);
    }
  } else {
    if (fetchSearchResults) {
      clearTimeout(fetchTimeout);
    }

    loaded = false;
    searchResults = [];
  }

  const fetchSearchResults = async () => {
    if (keyword.trim().length === 0) return;

    const currentKeyword = keyword;
    isLoading = true;
    let keywords = keyword.trim().split(/\s+/).join("+").toLowerCase();
    try {
      const results: Array<IFile> = await api.get(
        "/api/file/search?keywords=" + encodeURIComponent(keywords),
        "json"
      );
      if (keyword === currentKeyword) {
        searchResults = results;
        const historyKeywords = $keywordsStore;
        const index = historyKeywords.findIndex((k) => k === currentKeyword);
        if (index >= 0) {
          historyKeywords.splice(index, 1);
        }
        const newKeywords = [currentKeyword].concat(historyKeywords);
        keywordsStore.set(newKeywords);
        loaded = true;
      }
    } catch (e) {
      console.error(e);
      setNotification("error", "Search failed");
    } finally {
      isLoading = false;
      useHistoryKeyword = false;
    }
  };

  const selectFile = (file: IFile) => {
    let path = "/files/" + file.dir;
    if (file.file_type.toLowerCase() === "dir") {
      path += "/" + file.filename;
    } else {
      path += "?view=" + file.filename;
    }

    navigate(path);
    onClose();
  };

  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === "Escape") {
      onClose();
    }
  };

  const chooseHistoryKeyword = (historyKeyword: string) => {
    useHistoryKeyword = true;
    keyword = historyKeyword;
  };
</script>

<svelte:window on:keydown={handleKeydown} />

<Modal {onClose} size="large" showTitle={false} clickOutToClose={true}>
  <div class="flex flex-row justify-between items-center p-4">
    <Icon type={EIconType.search} size="small" color={EIconColor.blue} />
    <input
      type="text"
      class="w-full ml-2 border-none focus:outline-none"
      bind:value={keyword}
      bind:this={inputElement}
      placeholder="Search.."
    />
    <div class="flex flex-row items-center">
      <Icon
        type={EIconType.close}
        color={EIconColor.gray}
        size="small"
        className="cursor-pointer"
        onClick={() => (keyword = null)}
      />
      <div class="border rounded ml-1 px-1 cursor-pointer" on:click={onClose}>
        esc
      </div>
    </div>
  </div>
  <hr />
  {#if isLoading}
    <Spinner />
  {:else if !loaded}
    <div class="p-4 text-lg">
      <div class="mb-2"><b>Recent search</b></div>
      {#each $keywordsStore as keyword}
        <div
          class="hover:bg-gray-200 cursor-pointer p-2 rounded"
          on:click={() => chooseHistoryKeyword(keyword)}
        >
          {keyword}
        </div>
      {/each}
    </div>
  {:else if searchResults.length === 0}
    <div class="p-4 text-lg">No results</div>
  {:else}
    <div class="result-list overflow-y-auto">
      <div class="p-4 text-lg">{searchResults.length} results found:</div>
      {#each searchResults as result}
        <div
          class="grid grid-cols-3 p-4 bg-white text-lg hover:bg-gray-200 cursor-pointer"
          on:click={() => selectFile(result)}
        >
          <div class="col-span-3 lg:col-span-1 px-2 flex flex-row items-center">
            <FileIcon file={result} />
            <div class="ml-2 text-black break-all overflow-ellipsis">
              {result.filename}
            </div>
          </div>
          <div
            class="col-span-3 lg:col-span-2 px-4 lg:px-0 text-sm text-gray-400 break-all overflow-ellipsis my-auto"
          >
            {"/" + result.dir}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</Modal>

<style>
  @media only screen and (min-width: 320px) {
    .result-list {
      height: 20rem;
    }
  }

  @media only screen and (min-width: 768px) {
    .result-list {
      height: 30rem;
    }
  }
</style>
