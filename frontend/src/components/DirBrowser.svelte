<script lang="ts">
  import { onMount } from "svelte";
  import Button from "./Button.svelte";
  import Spinner from "./Spinner.svelte";
  import * as api from "../utils/api";
  import { setNotification } from "../utils/util";

  export let onClose = () => {};
  export let onSelect = (v: string) => {};

  let volumes: string[] = [];
  let dirs: string[] = [];
  let selectedVolume = "";
  let selectedDir = "";
  let currentDir = "";
  let level = 0;
  let back = false;
  let isLoading = false;

  onMount(async () => {
    try {
      isLoading = true;
      volumes = await api.get("/api/sys/volumes");
    } catch (e) {
      console.log(e);
      setNotification("error", "Cannot read volumes");
    } finally {
      isLoading = false;
    }
  });

  $: if (volumes.length > 0) {
    selectedVolume = volumes[0];
  }

  $: if (selectedVolume) {
    level = 0;
    selectedDir = selectedVolume;
  }

  $: if (selectedDir) {
    fetchDirs(selectedDir);
  }

  const selectVolume = (e: any) => {
    selectedVolume = e.target.value;
  };

  const fetchDirs = async (dir: string) => {
    try {
      isLoading = true;
      dirs = await api.get("/api/sys/dirs/" + dir);
      currentDir = selectedDir;
      level = back ? level - 1 : level + 1;
      back = false;
    } catch (e) {
      console.log(e);
      setNotification("error", "Cannot read directory");
    } finally {
      isLoading = false;
    }
  };

  const goToParentDir = (): void => {
    const dirSplit = currentDir.split("/").filter((e) => e.length > 0);
    if (currentDir.startsWith("/") && dirSplit.length > 0) {
      dirSplit[0] = "/" + dirSplit[0];
    }

    dirSplit.pop();
    const parentDir =
      dirSplit.length > 0 && dirSplit[0] ? dirSplit.join("/") : "/";
    back = true;
    selectedDir = parentDir;
  };

  const formatDir = (dir: string): string => {
    const dirSplit = dir.split("/");
    return dirSplit[dirSplit.length - 1];
  };
</script>

<div class="fixed z-10 left-0 top-0 w-full min-h-full bg-black bg-opacity-40">
  <div
    class="bg-white w-96 mt-20 mx-auto p-4 border rounded-lg border-gray-50 flex flex-col"
  >
    <div class="mb-4 text-xl mx-auto text-gray-700">Directory Browser</div>
    <!-- Volume selector -->
    <div class="mb-4 flex flex-row items-center">
      <span class="mr-4">Volumes:</span>
      <select
        class="px-2 border bg-gray-50 max-w-10"
        on:change={selectVolume}
        on:blur
      >
        {#each volumes as volume}
          <option value={volume}>
            {volume}
          </option>
        {/each}
      </select>
    </div>
    <hr />

    <!-- Selected directory -->
    <div class="mt-2">Selected Directory:</div>
    <div class="text-gray-700 font-bold break-words mb-2">{currentDir}</div>

    <!-- Sub directory list -->
    <div
      class="mb-2 py-2 border rounded h-60 overflow-y-auto overflow-x-hidden"
    >
      {#if isLoading}
        <Spinner />
      {:else}
        {#if level > 1}
          <div
            class="mx-2 px-2 rounded hover:bg-gray-200 cursor-pointer break-words"
            on:click={goToParentDir}
          >
            ..
          </div>
        {/if}

        {#each dirs as dir}
          <div
            class="mx-2 px-2 rounded hover:bg-gray-200 cursor-pointer break-words"
            on:click={() => (selectedDir = dir)}
          >
            {formatDir(dir)}
          </div>
        {/each}
      {/if}
    </div>

    <!-- footer buttons -->
    <div class="mx-auto my-4 flex flex-row">
      <Button
        value="Confirm"
        className="mr-4"
        disabled={!currentDir}
        onClick={() => {
          onSelect(currentDir);
          onClose();
        }}
      />
      <Button value="Cancel" onClick={onClose} />
    </div>
  </div>
</div>

<style>
  .max-w-10 {
    max-width: 10rem;
  }
</style>
