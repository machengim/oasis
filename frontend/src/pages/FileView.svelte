<script lang="ts">
  import { onDestroy } from "svelte";
  import { dirsStore, filesStore, setNotification } from "../utils/store";
  import type { IFile } from "../utils/types";
  import { FileType } from "../utils/types";
  import Spinner from "../components/Spinner.svelte";
  import BreadCrum from "../components/BreadCrum.svelte";
  import VideoPlayer from "../players/VideoPlayer.svelte";
  import { inferFileType, compareArray } from "../utils/util";
  import * as api from "../utils/api";

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
      siblings = $filesStore.filter((f) => f.file_type === fileType);
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
            <VideoPlayer {dirs} {filename} />
          {/if}
        </div>
        <div class="flex flex-col w-full lg:w-1/4 lg:pl-4 mt-4 lg:mt-0">
          File list
          {#each siblings as sibling}
            <div>
              {sibling.filename}
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>
