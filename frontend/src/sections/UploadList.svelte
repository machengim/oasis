<script lang="ts">
  import { uploadTaskStore } from "../utils/store";
  import Icon from "../components/Icon.svelte";
  import { EIconColor, EIconType, EUploadStatus } from "../utils/types";
  import type { IUploadTask } from "../utils/types";
  import { onDestroy } from "svelte";
  import PromptModal from "../modals/PromptModal.svelte";

  let showList = true;
  let uploadTasks: Array<IUploadTask> = [];
  let result = false;
  let showPromptModal = false;
  let text: string;

  const unsubscribeUploadTasks = uploadTaskStore.subscribe((task) => {
    if (task) {
      uploadTasks.push(task);
      uploadTasks = uploadTasks;
    }
  });

  onDestroy(() => {
    unsubscribeUploadTasks();
  });

  const toggleShowList = () => {
    showList = !showList;
  };

  const removeAllTasks = async () => {
    const unfinishedTasks = uploadTasks.filter(
      (t) =>
        t.status !== EUploadStatus.success && t.status !== EUploadStatus.failed
    ).length;

    if (unfinishedTasks === 0) {
      uploadTasks = [];
    } else {
      const is_text = unfinishedTasks > 1 ? "are" : "is";
      const tasks_text = unfinishedTasks > 1 ? "tasks" : "task";
      const them_text = unfinishedTasks > 1 ? "them" : "it";
      text = `There ${is_text} <b>${unfinishedTasks} unfinished</b> upload ${tasks_text} in the job queue. Are you sure you want to cancel ${them_text}?`;
      result = false;
      showPromptModal = true;

      while (showPromptModal) {
        await new Promise((r) => setTimeout(r, 200));
      }

      if (result) {
        uploadTasks = [];
      }
    }
  };

  const removeTask = async (task: IUploadTask) => {
    const index = uploadTasks.findIndex((t) => t.file === task.file);
    if (index >= 0) {
      text = `Are you sure you want to cancel the upload task of file <b>${task.file.name}</b> ?`;
      result = false;
      showPromptModal = true;

      while (showPromptModal) {
        await new Promise((r) => setTimeout(r, 200));
      }

      if (result) {
        uploadTasks.splice(index, 1);
        uploadTasks = uploadTasks;
      }
    }
  };

  const closePrompModal = () => {
    showPromptModal = false;
  };

  const setResult = (r: boolean) => {
    result = r;
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
      <div class="text-lg text-blue-400">Upload list</div>
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
              <div class="mr-1">{task.status}</div>
              {#if task.status === EUploadStatus.finishing || task.status === EUploadStatus.success}
                <Icon
                  type={EIconType.success}
                  color={EIconColor.green}
                  size="small"
                  className="cursor-pointer"
                  onClick={() => removeTask(task)}
                />
              {:else}
                <Icon
                  type={EIconType.close}
                  color={EIconColor.red}
                  size="small"
                  className="cursor-pointer"
                  onClick={() => removeTask(task)}
                />
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
{/if}
