<script lang="ts">
  import { uploadTaskStore } from "../utils/store";
  import { t } from "svelte-i18n";
  import Icon from "../components/Icon.svelte";
  import { EIconColor, EIconType, EUploadStatus } from "../utils/types";
  import type { IUploadTask } from "../utils/types";
  import { onDestroy } from "svelte";
  import PromptModal from "../modals/PromptModal.svelte";
  import { upload, cancelUploads } from "../utils/upload";
  import { sleep } from "../utils/util";

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
      text = $t("modal.upload.cancel_uploads_warn");
      result = false;
      showPromptModal = true;

      while (showPromptModal) {
        await sleep(200);
      }

      if (result) {
        try {
          await cancelUploads(uploadTasks);
        } catch (e) {
          console.error(e);
        }

        uploadTaskStore.set([]);
      }
    }
  };

  const removeTask = async (task: IUploadTask, finished: boolean) => {
    const index = uploadTasks.findIndex((t) => t.file === task.file);
    result = finished;
    if (index >= 0 && !result) {
      text =
        $t("modal.upload.cancel_upload_file") +
        " <b>" +
        task.file.name +
        "</b> ?";
      showPromptModal = true;

      while (showPromptModal) {
        await sleep(200);
      }
    }

    if (result) {
      try {
        await cancelUploads([task]);
      } catch (e) {
        console.error(e);
      }

      uploadTasks.splice(index, 1);
      uploadTaskStore.set(uploadTasks);
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
    title={$t("modal.upload.title_cancel")}
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
      <div class="text-lg text-blue-400">
        {$t("component.upload_list.title")}
        {uploadInfo}
      </div>
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
