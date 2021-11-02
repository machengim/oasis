<script lang="ts">
  import { terminateWorkers, uploadTaskStore } from "../utils/store";
  import Icon from "../components/Icon.svelte";
  import { EIconColor, EIconType, EUploadStatus } from "../utils/types";
  import type { IUploadTask } from "../utils/types";
  import { onDestroy } from "svelte";
  import PromptModal from "../modals/PromptModal.svelte";
  import { upload } from "../utils/upload";
  import { sleep } from "../utils/util";
  import * as api from "../utils/api";

  let showList = true;
  let uploadTasks: Array<IUploadTask> = [];
  let currentTask: IUploadTask = null;
  // Whether the user confirmed to remove the task
  let result = false;
  let showPromptModal = false;
  let text: string;
  let uploadInfo = "";

  const unsubscribeUploadTasks = uploadTaskStore.subscribe((tasks) => {
    if (tasks) {
      uploadTasks = tasks;
    }
  });

  onDestroy(() => {
    unsubscribeUploadTasks();
  });

  $: if (uploadTasks) {
    formatUploadInfo();

    if (
      !currentTask ||
      currentTask.status === EUploadStatus.success ||
      currentTask.status === EUploadStatus.failed ||
      uploadTasks.findIndex((t) => t.file === currentTask.file) < 0
    ) {
      currentTask = findNextTask();
    }
  }

  $: if (currentTask) {
    processCurrentTask();
  }

  const findNextTask = () => {
    if (uploadTasks.length === 0) {
      return null;
    }

    const nextIndex = uploadTasks.findIndex(
      (t) => t.status === EUploadStatus.waiting
    );
    return uploadTasks[nextIndex];
  };

  const processCurrentTask = async () => {
    currentTask.status = EUploadStatus.preparing;
    uploadTasks = uploadTasks;
    await upload(currentTask);
  };

  const toggleShowList = () => {
    showList = !showList;
  };

  const removeAllTasks = async () => {
    const unfinishedTasks = uploadTasks.filter(
      (t) => t.status !== EUploadStatus.success
    ).length;

    if (unfinishedTasks === 0) {
      uploadTaskStore.set([]);
    } else {
      const is_text = unfinishedTasks > 1 ? "are" : "is";
      const tasks_text = unfinishedTasks > 1 ? "tasks" : "task";
      const them_text = unfinishedTasks > 1 ? "them" : "it";
      text = `There ${is_text} <b>${unfinishedTasks} unfinished</b> upload ${tasks_text} in the job queue. Are you sure you want to cancel ${them_text}?`;
      result = false;
      showPromptModal = true;

      while (showPromptModal) {
        await sleep(200);
      }

      if (result) {
        const hashesToRemove = uploadTasks
          .map((t) => t.hash)
          .filter((h) => !!h);
        console.log(hashesToRemove);

        terminateWorkers();
        uploadTaskStore.set([]);

        if (hashesToRemove.length > 0) {
          try {
            const payload = { hashes: hashesToRemove };
            await api.remove(`/api/upload`, payload, false);
          } catch (e) {
            console.error(e);
          }
        }
      }
    }
  };

  const removeTask = async (task: IUploadTask, finished: boolean) => {
    const index = uploadTasks.findIndex((t) => t.file === task.file);
    result = finished;
    if (index >= 0 && !result) {
      text = `Are you sure you want to cancel the upload task of file <b>${task.file.name}</b> ?`;
      showPromptModal = true;

      while (showPromptModal) {
        await sleep(200);
      }
    }

    if (result) {
      if (finished) {
        uploadTasks.splice(index, 1);
        uploadTaskStore.set(uploadTasks);
      } else {
        const hashToRemove = uploadTasks[index].hash;
        terminateWorkers();
        uploadTasks.splice(index, 1);
        uploadTaskStore.set(uploadTasks);

        if (hashToRemove) {
          try {
            const payload = { hashes: [hashToRemove] };
            await api.remove(`/api/upload`, payload, false);
          } catch (e) {
            console.error(e);
          }
        }
      }
    }
  };

  const closePrompModal = () => {
    showPromptModal = false;
  };

  const setResult = (r: boolean) => {
    result = r;
  };

  // Input format: 0.1225, output: 12.3%
  const formatProgress = (progress: number) => {
    const value = (Math.round(progress * 10000) / 100).toFixed(1);
    return `${value}%`;
  };

  const formatUploadInfo = () => {
    const successTasks = uploadTasks.filter(
      (t) => t.status === EUploadStatus.success
    );
    const successNumber = successTasks.length;
    const allTaskNumber = uploadTasks.length;
    uploadInfo = `(${successNumber} / ${allTaskNumber})`;
  };
</script>

{#if showPromptModal}
  <PromptModal
    title="Cancel upload task"
    {text}
    onClose={closePrompModal}
    setResult={(r) => setResult(r)}
  />
{/if}
{#if uploadTasks.length > 0}
  <div
    class="fixed w-60 lg:w-72 bottom-2 right-2 lg:bottom-8 lg:right-8 z-20 border rounded shadow bg-white bg-opacity-100"
  >
    <div class="px-2 py-2 flex flex-row justify-between ">
      <div class="text-lg text-blue-400">Upload list {uploadInfo}</div>
      <div class="flex flex-row">
        {#if showList}
          <Icon
            type={EIconType.down}
            color={EIconColor.black}
            size="small"
            className="mr-2 cursor-pointer"
            onClick={toggleShowList}
          />
        {:else}
          <Icon
            type={EIconType.up}
            color={EIconColor.black}
            size="small"
            className="mr-2 cursor-pointer"
            onClick={toggleShowList}
          />
        {/if}
        <Icon
          type={EIconType.close}
          color={EIconColor.black}
          size="small"
          className="cursor-pointer"
          onClick={removeAllTasks}
        />
      </div>
    </div>
    {#if showList && uploadTasks.length > 0}
      <hr />
      <div class="py-2">
        {#each uploadTasks as task}
          <div class="flex flex-row justify-between px-2 items-center">
            <div
              class="w-40 lg:w-48 mr-4 whitespace-nowrap overflow-hidden overflow-ellipsis"
            >
              {task.file.name}
            </div>
            <div class="flex flex-row">
              {#if task.status === EUploadStatus.uploading}
                <div class="mr-1">{formatProgress(task.progress)}</div>
              {:else}
                <div class="mr-1">{task.status}</div>
              {/if}
              {#if task.status === EUploadStatus.success}
                <Icon
                  type={EIconType.success}
                  color={EIconColor.green}
                  size="small"
                  className="cursor-pointer"
                  onClick={() => removeTask(task, true)}
                />
              {:else}
                <Icon
                  type={EIconType.close}
                  color={EIconColor.red}
                  size="small"
                  className="cursor-pointer"
                  onClick={() => removeTask(task, false)}
                />
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
{/if}
