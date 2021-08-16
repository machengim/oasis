<script lang="ts">
  import { onDestroy } from "svelte";
  import { uploadTaskStore, progressStore } from "../utils/store";
  import type { IProgress, IUploadTask } from "../utils/types";
  import Icon from "../components/Icon.svelte";
  import * as api from "../utils/api";

  let showFiles = true;
  let tasks: IUploadTask[] = [];
  let currentTask: IUploadTask = null;

  const unsubscribeTask = uploadTaskStore.subscribe((value) => {
    if (value.length > 0) {
      const newTasks = tasks.concat(value);
      tasks = newTasks;
      uploadTaskStore.set([]);
    }
  });

  const unsubscribeProgress = progressStore.subscribe((value) => {
    if (value && tasks.length > 0) {
      updateTask(value);
      progressStore.set(null);
    }
  });

  onDestroy(() => {
    unsubscribeTask();
    unsubscribeProgress();
  });

  $: if (!currentTask && tasks.length > 0) {
    currentTask = tasks.find((t) => t.status === "pending");
  }

  $: if (currentTask) {
    startTask(currentTask);
  }

  const startTask = async (task: IUploadTask) => {
    task.status = "uploading";
    await api.upload(task);
  };

  const updateTask = (progress: IProgress) => {
    let index = tasks.findIndex((t) => t.id === progress.id);
    let newTask = Object.assign({}, tasks[index]);
    newTask.progress = progress.progress;

    if (newTask.progress === 1) {
      newTask.status = "complete";
      currentTask = null;
    }

    tasks[index] = newTask;
  };

  const toggleShowFiles = () => {
    showFiles = !showFiles;
  };

  const clearTaskPanel = () => {
    if (tasks.findIndex((t) => t.status !== "complete") >= 0) {
      let confirm = window.confirm(
        "Tasks still in progress. Are you sure to remove them?"
      );
      if (!confirm) return;
    }

    tasks = [];
  };

  const removeTask = (id: number, complete: boolean) => {
    if (!complete) {
      let confirm = window.confirm(
        "Task still in progress. Are you sure to remove it?"
      );
      if (!confirm) return;
    }
    const newTasks = tasks.filter((t) => t.id !== id);
    tasks = newTasks;

    if (currentTask && currentTask.id === id) {
      currentTask = null;
    }
  };
</script>

{#if tasks.length > 0}
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
        <span class="cursor-pointer" on:click={clearTaskPanel}>
          <Icon type="close" color="white" size="small" on:cli />
        </span>
      </div>
    </div>
    {#if showFiles}
      <div>
        {#each tasks as task}
          <div class="flex flex-row justify-between px-4 py-2">
            <div>{task.file.name}</div>
            <div class="flex flex-row items-center">
              <span>{task.progress * 100}%</span>
              {#if task.status === "complete"}
                <Icon
                  type="success"
                  color="green"
                  size="tiny"
                  className="ml-1 cursor-pointer"
                  onClick={() => removeTask(task.id, true)}
                />
              {:else}
                <span>
                  <Icon
                    type="close"
                    color="red"
                    size="tiny"
                    className="ml-1 cursor-pointer"
                    onClick={() => removeTask(task.id, false)}
                  />
                </span>
              {/if}
            </div>
          </div>
          <hr />
        {/each}
      </div>
    {/if}
  </div>
{/if}
