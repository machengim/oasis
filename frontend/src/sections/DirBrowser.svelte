<script lang="ts">
  import { onMount } from "svelte";
  import { t } from "svelte-i18n";
  import Button from "../components/Button.svelte";
  import Spinner from "../components/Spinner.svelte";
  import * as api from "../utils/api";
  import { setNotification } from "../utils/store";

  export let onClose = () => {};
  export let onSelect = (v: string) => {};
  let volumes: string[] = [];
  let path: string[] = null;
  let sub_dirs: string[] = [];
  let selectedVolume = "";
  let currentDir = "";
  let isLoading = false;

  onMount(() => {
    fetchVolumes();
  });

  $: if (volumes.length > 0) {
    selectedVolume = volumes[0];
  }

  $: if (selectedVolume) {
    path = [];
  }

  $: if (path) {
    currentDir = buildFullPath();
    fetchSubDirs();
  }

  const fetchVolumes = async () => {
    isLoading = true;
    try {
      const volumes_res: Array<string> = await api.get("/api/sys/volumes");
      volumes = volumes_res.map((s) => removeVolumeTailChar(s));
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.read_volume_error"));
    }
    isLoading = false;
  };

  const fetchSubDirs = async () => {
    const currentPath = path;
    isLoading = true;
    try {
      const endpoint = "/api/sys/dirs/" + encodeURIComponent(buildFullPath());
      // Tricky part is, on Windows, the result may be `C:Windows`
      // if the special character has been cleaned up in the volume name.
      const sub_dirs_res: Array<string> = await api.get(endpoint);
      if (currentPath === path) {
        sub_dirs_res.sort();
        sub_dirs = sub_dirs_res
          .map((s) => formatDir(s))
          .filter((s) => s[0] !== "." && s[0] !== "$");
      }
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.read_dir_error"));
      if (currentPath === path && path.length > 0) {
        path = path.slice(0, -1);
      }
    }

    isLoading = false;
  };

  const selectVolume = (e: any) => {
    selectedVolume = e.target.value.toString();
  };

  const selectDir = (dir: string) => {
    path = [...path, dir];
  };

  const goToParentDir = () => {
    if (path && path.length > 0) {
      path = path.slice(0, -1);
    }
  };

  // For Linux and Mac, it looks like `/home/user/Documents`,
  // For Windows, it looks like `C:/Windows/System` or `C:\\Windows\\System.
  // And volume names may vary from `/` to `/run/media/user/Toshiba`.
  const buildFullPath = () => {
    let fullPath = selectedVolume;

    for (let pathComponent of path) {
      const lastChar = fullPath.charAt(fullPath.length - 1);
      const appendPath = lastChar === "/" ? pathComponent : "/" + pathComponent;
      fullPath += appendPath;
    }

    return fullPath;
  };

  const formatDir = (dir: string) => {
    // Windows may return the full dir path back, such as `C:\\Windows\\System`.
    // Only the final part is expected.
    const dirSplit = dir.split(/[\/\\]/).filter((s) => s.trim().length > 0);
    if (dirSplit.length > 0) {
      return dirSplit.pop();
    }

    return null;
  };

  const removeVolumeTailChar = (vol: string) => {
    if (vol.length <= 1) {
      return vol;
    }
    if (vol[vol.length - 1] === "\\") {
      vol = vol.slice(0, -1) + "/";
    }

    return vol;
  };
</script>

<div class="fixed z-10 left-0 top-0 w-full min-h-full bg-black bg-opacity-40">
  <div
    class="bg-white w-96 mt-20 mx-auto p-4 border rounded-lg border-gray-50 flex flex-col"
  >
    <div class="mb-4 text-xl mx-auto text-gray-700">
      {$t("component.dir_browser.title")}
    </div>
    <!-- Volume selector -->
    <div class="mb-4 flex flex-row items-center">
      <span class="mr-4">{$t("component.dir_browser.volumes")}:</span>
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
    <div class="mt-2">{$t("component.dir_browser.selected_dir")}:</div>
    <div class="text-gray-700 font-bold break-words mb-2">{currentDir}</div>

    <!-- Sub directory list -->
    <div
      class="mb-2 py-2 border rounded h-60 overflow-y-auto overflow-x-hidden"
    >
      {#if isLoading}
        <Spinner />
      {:else}
        {#if path && path.length > 0}
          <div
            class="mx-2 px-2 rounded hover:bg-gray-200 cursor-pointer break-words"
            on:click={goToParentDir}
          >
            ..
          </div>
        {/if}

        {#each sub_dirs as dir}
          <div
            class="mx-2 px-2 rounded hover:bg-gray-200 cursor-pointer break-words"
            on:click={() => selectDir(dir)}
          >
            {dir}
          </div>
        {/each}
      {/if}
    </div>

    <!-- footer buttons -->
    <div class="mx-auto my-4 flex flex-row">
      <Button
        value={$t("button.confirm")}
        className="mr-4"
        disabled={!currentDir}
        onClick={() => {
          onSelect(currentDir);
          onClose();
        }}
      />
      <Button value={$t("button.cancel")} onClick={onClose} />
    </div>
  </div>
</div>

<style>
  .max-w-10 {
    max-width: 10rem;
  }
</style>
