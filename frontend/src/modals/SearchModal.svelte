<script lang="ts">
  import Modal from "../components/Modal.svelte";
  import { t } from "svelte-i18n";
  import { useNavigate } from "svelte-navigator";
  import Icon from "../components/Icon.svelte";
  import { EIconColor, EIconType } from "../utils/enums";
  import type { IFile } from "../utils/types";
  import * as api from "../utils/api";
  import { setNotification } from "../utils/store";
  import Spinner from "../components/Spinner.svelte";
  import FileIcon from "../components/FileIcon.svelte";
  import { onDestroy, onMount } from "svelte";

  export let onClose: () => void;
  const navigate = useNavigate();
  const localStorageKey = "oa_keywords";
  const historySize = 10; // how many recent search keywords are saved
  let inputElement: HTMLInputElement;
  let keyword: string;
  let fetchTimeout: NodeJS.Timeout;
  let searchResults: Array<IFile> = [];
  let isLoading = false;
  let loaded = false;
  let historyKeywords: Array<string> = [];
  let useHistoryKeyword = false; // When using recent keyword, start fetching immediately

  onMount(() => {
    if (inputElement) {
      inputElement.focus();
    }
    initHistoryKeywords();
  });

  onDestroy(() => {
    clearFetchTimeout();
    keyword = null;
    searchResults = [];
  });

  $: if (keyword && keyword.trim().length > 0) {
    searchKeywords();
  } else {
    clearFetchTimeout();
    loaded = false;
    searchResults = [];
  }

  const initHistoryKeywords = () => {
    const keywordsInLocalStorage = localStorage.getItem(localStorageKey);
    if (keywordsInLocalStorage) {
      historyKeywords = JSON.parse(keywordsInLocalStorage);
    }
  };

  const clearFetchTimeout = () => {
    if (fetchTimeout) {
      clearTimeout(fetchTimeout);
    }
  };

  const searchKeywords = () => {
    clearFetchTimeout();
    if (useHistoryKeyword) {
      fetchSearchResults();
    } else {
      fetchTimeout = setTimeout(() => {
        fetchSearchResults();
      }, 1000);
    }
  };

  const fetchSearchResults = async () => {
    if (keyword.trim().length === 0) return;

    isLoading = true;
    const currentKeyword = keyword;
    let keywords = keyword.trim().split(/\s+/).join("+").toLowerCase();
    try {
      const results: Array<IFile> = await api.get(
        "/api/file/search?keywords=" + encodeURIComponent(keywords),
        "json"
      );
      if (keyword === currentKeyword) {
        searchResults = results;
        saveKeywordToHistory(currentKeyword);
        loaded = true;
      }
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.search_file"));
    } finally {
      isLoading = false;
      useHistoryKeyword = false;
    }
  };

  const saveKeywordToHistory = (currentKeyword: string) => {
    const index = historyKeywords.findIndex((k) => k === currentKeyword.trim());
    if (index >= 0) {
      historyKeywords.splice(index, 1);
    }
    let newKeywords = [currentKeyword.trim()].concat(historyKeywords);
    if (newKeywords.length > historySize) {
      newKeywords = newKeywords.slice(0, historySize);
    }
    setLocalKeywords(newKeywords);
    historyKeywords = newKeywords;
  };

  const removeKeywordFromHistory = (e: Event, targetKeyword: string) => {
    e.stopPropagation();

    const index = historyKeywords.findIndex((k) => k === targetKeyword.trim());

    if (index >= 0) {
      const newKeywords = historyKeywords;
      newKeywords.splice(index, 1);
      setLocalKeywords(newKeywords);
      historyKeywords = newKeywords;
    }
  };

  const clearAllKeywordsFromHistory = () => {
    historyKeywords = [];
    setLocalKeywords(historyKeywords);
  };

  const setLocalKeywords = (data: Array<string>) => {
    localStorage.setItem(localStorageKey, JSON.stringify(data));
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
      placeholder=".txt readme"
    />
    <div class="flex flex-row items-center">
      <Icon
        type={EIconType.close}
        color={EIconColor.gray}
        hoverColor={EIconColor.black}
        size="small"
        className="cursor-pointer"
        onClick={() => (keyword = null)}
      />
      <div
        class="border rounded ml-1 px-1 cursor-pointer hover:bg-blue-400 hover:text-white"
        on:click={onClose}
      >
        esc
      </div>
    </div>
  </div>
  <hr />
  {#if isLoading}
    <Spinner />
  {:else if !loaded}
    <div class="flex-grow overflow-y-auto p-4 text-lg">
      <div class="mb-2 flex flex-row justify-between items-center">
        <div><b>{$t("modal.search_file.recent")}</b></div>
        <div
          class="rounded px-2 cursor-pointer hover:bg-blue-400 hover:text-white"
          on:click={clearAllKeywordsFromHistory}
        >
          {$t("modal.search_file.clear_all")}
        </div>
      </div>
      {#each historyKeywords as keyword}
        <div
          class="flex flex-row justify-between items-center hover:bg-gray-200 cursor-pointer p-2 rounded"
          on:click={() => chooseHistoryKeyword(keyword)}
        >
          {keyword}
          <Icon
            type={EIconType.close}
            color={EIconColor.gray}
            hoverColor={EIconColor.black}
            size="small"
            className="cursor-pointer"
            onClick={(e) => removeKeywordFromHistory(e, keyword)}
          />
        </div>
      {/each}
    </div>
  {:else if searchResults.length === 0}
    <div class="h-full p-4 text-lg">
      <b>{$t("modal.search_file.no_results")}</b>
    </div>
  {:else}
    <div class="flex-grow overflow-y-auto">
      <div class="p-4 text-lg">
        <b>{searchResults.length} {$t("modal.search_file.results")}</b>
      </div>
      {#each searchResults as result}
        <div
          class="grid grid-cols-3 p-2 lg:p-4 bg-white text-lg hover:bg-gray-200 cursor-pointer"
          on:click={() => selectFile(result)}
        >
          <div class="col-span-3 lg:col-span-1 px-2 flex flex-row items-center">
            <FileIcon file={result} />
            <div class="ml-2 text-black break-all overflow-ellipsis">
              {result.filename}
            </div>
          </div>
          <div
            class="col-span-3 lg:col-span-2 px-2 ml-1 lg:px-0 text-sm text-gray-400 break-all overflow-ellipsis my-auto"
          >
            {"/" + result.dir}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</Modal>
