<script lang="ts">
  import { onDestroy } from "svelte";
  import { useNavigate } from "svelte-navigator";
  import {
    dirsStore,
    filesStore,
    setNotification,
    autoPlayStore,
  } from "../utils/store";
  import type { IFile, IFileOrder } from "../utils/types";
  import { FileType } from "../utils/types";
  import Spinner from "../components/Spinner.svelte";
  import Switch from "../components/Switch.svelte";
  import BreadCrum from "../components/BreadCrum.svelte";
  import VideoPlayer from "../players/VideoPlayer.svelte";
  import { inferFileType, compareArray, compareFile } from "../utils/util";
  import * as api from "../utils/api";

  const navigate = useNavigate();
  export let dirs: Array<string>;
  export let filename: string;
  let siblings: Array<IFile>;
  let filesInStore: Array<IFile>;
  let isLoading = false;
  let fileType: FileType;

  const unsubscribeFiles = filesStore.subscribe((files) => {
    filesInStore = files;
  });

  onDestroy(() => {
    unsubscribeFiles();
  });

  $: if (filename) {
    extractFileType(filename);
  }

  $: if (fileType && filesInStore) {
    if (compareArray(dirs, $dirsStore)) {
      const order: IFileOrder = { key: "name", asc: true };
      const siblingFiles = $filesStore.filter((f) => f.file_type === fileType);
      siblingFiles.sort((a, b) => compareFile(a, b, order));
      siblings = siblingFiles;
    } else {
      fetchDirContent(dirs);
    }
  }

  const fetchDirContent = async (dirs: Array<string>) => {
    let endpoint = "/api/dir";
    if (dirs.length > 0) {
      endpoint += "?path=" + encodeURIComponent(dirs.join("/"));
    }

    isLoading = true;
    try {
      let files: Array<IFile> = await api.get(endpoint);
      dirsStore.set(dirs);
      filesStore.set(files);
    } catch (e) {
      console.error(e);
      setNotification("error", "Cannot read directory");
    }

    isLoading = false;
  };

  const extractFileType = (filename: string) => {
    const splits = filename.split(".");
    let file_ext: string;

    if (splits.length < 2) {
      file_ext = null;
    } else {
      file_ext = splits.slice(-1)[0].toLowerCase();
    }

    fileType = inferFileType(file_ext);
  };

  const selectSibling = (index: number) => {
    const file = siblings[index];
    const path = dirs.join("/") + "?view=" + file.filename;

    navigateFile(path);
  };

  const navigateFile = (path: string) => {
    const pathPrefix = path.startsWith("/") ? path : "/" + path;
    const targetPath = "/files" + pathPrefix;

    navigate(targetPath);
  };

  const toggleAutoPlay = (autoPlay: boolean) => {
    autoPlayStore.set(autoPlay);
  };

  const onMediaComplete = () => {
    let currentIndex = siblings.findIndex((s) => s.filename === filename);
    if (currentIndex + 1 < siblings.length) {
      selectSibling(currentIndex + 1);
    }
  };
</script>

<div class="relative w-full h-full">
  <div class="w-11/12 lg:w-4/5 h-full mx-auto my-4 lg:mt-4 lg:mb-10">
    <BreadCrum {dirs} {filename} className="px-1 py-2" />
    {#if isLoading}
      <Spinner />
    {:else}
      <div class="flex flex-row flex-wrap mt-4">
        <div class="w-full lg:w-3/4">
          {#if fileType === FileType.Video}
            <VideoPlayer {dirs} {filename} onComplete={onMediaComplete} />
          {/if}
        </div>
        <div
          class="flex flex-col h-80 w-full lg:w-1/5 lg:ml-6 mt-4 lg:mt-0 p-1 bg-gray-100"
        >
          <div class="flex flex-row items-center justify-between px-2 py-1">
            <div class="text-xl">File list</div>
            <div class="flex flex-row items-center">
              <span class="mr-2">Auto play</span>
              <Switch
                toggleCheck={toggleAutoPlay}
                defaultCheck={$autoPlayStore}
              />
            </div>
          </div>
          <div class="overflow-y-auto flex-grow">
            {#each siblings as sibling, i}
              {#if filename === sibling.filename}
                <div class="bg-blue-400 text-white rounded">
                  <span class="px-2 py-1">
                    {sibling.filename}
                  </span>
                </div>
              {:else}
                <div
                  class="rounded cursor-pointer hover:bg-blue-400 hover:text-white"
                  on:click={() => selectSibling(i)}
                >
                  <span class="px-2 py-1">
                    {sibling.filename}
                  </span>
                </div>
              {/if}
            {/each}
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>
