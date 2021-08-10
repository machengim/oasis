<script lang="ts">
  import { onDestroy } from "svelte";
  import { uploadTaskStore } from "../utils/store";
  import type { IUploadTask } from "../utils/types";
  import Icon from "../components/Icon.svelte";

  let showFiles = true;
  let tasks: IUploadTask[] = [];
  let currentTask: IUploadTask = null;

  const unsubscribe = uploadTaskStore.subscribe((value) => {
    if (value.length > 0) {
      const newTasks = tasks.concat(value);
      tasks = newTasks;
      uploadTaskStore.set([]);
    }
  });

  onDestroy(() => {
    unsubscribe();
  });

  $: if (!currentTask && tasks.length > 0) {
    currentTask = tasks[0];
  }

  // TODO: upload file.
  $: if (currentTask) {
  }

  const toggleShowFiles = () => {
    showFiles = !showFiles;
  };
</script>

<div
  class="absolute bg-white right-4 bottom-4 w-80 overflow-x-hidden z-50 flex flex-col border rounded-lg shadow-lg"
>
  <div class="flex flex-row justify-between bg-gray-700  py-2 px-4">
    <div class="text-white text-lg">Uploading files</div>
    <div class="flex flex-row">
      <span class="mr-2 cursor-pointer" on:click={toggleShowFiles}>
        {#if showFiles}
          <Icon type="down" color="white" size="small" />
        {:else}
          <Icon type="up" color="white" size="small" />
        {/if}
      </span>
      <span class="cursor-pointer">
        <Icon type="close" color="white" size="small" />
      </span>
    </div>
  </div>
  {#if showFiles}
    <div>
      {#each tasks as file}
        <div class="flex flex-row justify-between px-4 py-2">
          <div>{file.filename}</div>
          <div class="flex flex-row items-center">
            <span>{file.progress}</span>
            <Icon
              type="close"
              color="red"
              size="tiny"
              className="ml-1 cursor-pointer"
            />
          </div>
        </div>
        <hr />
      {/each}
    </div>
  {/if}
</div>
