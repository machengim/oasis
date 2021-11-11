<script lang="ts">
  import { useNavigate } from "svelte-navigator";
  import { t } from "svelte-i18n";
  import {
    setNotification,
    dirsStore,
    filesStore,
    titleStore,
    uploadTaskStore,
    resetTitle,
    clickStore,
  } from "../utils/store";
  import type {
    IFile,
    IFileOrder,
    IMousePosition,
    IUploadTask,
  } from "../utils/types";
  import { EUploadStatus } from "../utils/types";
  import { EIconType } from "../utils/types";
  import * as api from "../utils/api";
  import Icon from "../components/Icon.svelte";
  import Spinner from "../components/Spinner.svelte";
  import BreadCrum from "../components/BreadCrum.svelte";
  import { formatSize, compareFile } from "../utils/util";
  import FileIcon from "../components/FileIcon.svelte";
  import Button from "../components/Button.svelte";
  import PromptModal from "../modals/PromptModal.svelte";
  import NewDirNameModal from "../modals/NewDirNameModal.svelte";
  import DeleteFileModal from "../modals/DeleteFileModal.svelte";
  import ContextMenu from "../sections/ContextMenu.svelte";
  import { onDestroy } from "svelte";

  const navigate = useNavigate();
  let dirs = $dirsStore;
  let files: Array<IFile> = $filesStore;
  let order: IFileOrder = { key: "name", asc: true };
  let isLoading = false;
  let fileSelector: HTMLInputElement;
  let title: string;
  let text: string;
  let result = false;
  let resultForAll = false;
  let showPromptModal = false;
  let showNewMenu = false;
  let showNewDirNameModal = false;
  let showContextMenu = false;
  let showDeleteFileModal = false;
  let contextPos: IMousePosition;
  let contextFile: IFile;

  const unsubscribeDirs = dirsStore.subscribe((d) => (dirs = d));

  const unsubscribeFiles = filesStore.subscribe((f) => (files = f));

  const unsubscribeClick = clickStore.subscribe((click) => {
    if (click > 0) {
      if (showNewMenu) {
        showNewMenu = false;
      }
      if (showContextMenu) {
        showContextMenu = false;
        contextFile = null;
      }
    }
  });

  onDestroy(() => {
    unsubscribeDirs();
    unsubscribeFiles();
    unsubscribeClick();
  });

  $: if (dirs.length >= 1) {
    titleStore.set(dirs[dirs.length - 1]);
  } else {
    resetTitle();
  }

  $: fetchDirContent(dirs);

  $: if (files.length > 0 && order) {
    orderFiles();
  }

  const fetchDirContent = async (targetDirs: Array<string>) => {
    isLoading = true;

    let endpoint = "/api/dir";
    if (targetDirs.length > 0) {
      endpoint += "?path=" + encodeURIComponent(targetDirs.join("/"));
    }

    try {
      const newFiles: Array<IFile> = await api.get(endpoint);
      if (dirs === targetDirs) {
        filesStore.set(newFiles);
      }
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.read_dir_error"));
    }

    isLoading = false;
  };

  const changeOrder = (key: "name" | "type" | "size") => {
    let newOrder = Object.assign({}, order);
    if (key === order.key) {
      newOrder.asc = !order.asc;
    } else {
      newOrder.key = key;
      newOrder.asc = true;
    }

    order = newOrder;
  };

  const orderFiles = () => {
    let dirs = files.filter((f) => f.file_type.toUpperCase() === "DIR");
    let others = files.filter((f) => f.file_type.toUpperCase() !== "DIR");

    dirs.sort((a, b) => compareFile(a, b, order));
    others.sort((a, b) => compareFile(a, b, order));

    files = dirs.concat(others);
  };

  const selectFile = (file: IFile) => {
    let path: string;
    if (file.file_type.toLowerCase() === "dir") {
      path = dirs.join("/") + "/" + file.filename;
    } else {
      path = dirs.join("/") + "?view=" + file.filename;
    }

    navigateFile(path);
  };

  const backToParentDir = () => {
    const path = dirs.slice(0, -1).join("/");
    navigateFile(path);
  };

  const navigateFile = (path: string) => {
    const pathPrefix = path.startsWith("/") ? path : "/" + path;
    const targetPath = "/files" + pathPrefix;

    navigate(targetPath);
  };

  const openSelectFileDialog = () => {
    if (fileSelector) {
      fileSelector.click();
    }
  };

  const selectUploadFile = async (e: Event) => {
    const target = e.target as HTMLInputElement;
    const filelist = target.files as FileList;

    for (const file of filelist) {
      if (
        !resultForAll &&
        files.findIndex((f) => f.filename === file.name) >= 0
      ) {
        title = "Filename existed";
        text = `File <b>${file.name}</b> already existed. Are you sure you want to overwrite it?`;
        result = false;
        showPromptModal = true;

        while (showPromptModal) {
          await new Promise((r) => setTimeout(r, 200));
        }

        if (!result && !resultForAll) {
          continue;
        }
      }

      const upload: IUploadTask = {
        file,
        targetDir: dirs,
        status: EUploadStatus.waiting,
        progress: 0,
      };

      const tasks = $uploadTaskStore;
      tasks.push(upload);
      uploadTaskStore.set(tasks);
    }

    resultForAll = false;
  };

  const setResult = (r: boolean) => {
    result = r;
  };

  const setResultAll = (r: boolean) => {
    resultForAll = r;
  };

  const closePrompModal = () => {
    showPromptModal = false;
  };

  const openNewDirNameModal = () => {
    showNewDirNameModal = true;
  };

  const closeNewDirNameModal = () => {
    showNewDirNameModal = false;
    contextFile = null;
  };

  const toggleShowNewMenu = (e: Event) => {
    e.stopPropagation();
    showNewMenu = !showNewMenu;
  };

  const openContextMenu = (e: MouseEvent, file: IFile) => {
    contextFile = file;
    contextPos = { x: e.clientX, y: e.clientY };
    showContextMenu = true;
  };

  const onContextAction = (action: "rename" | "delete" | "close") => {
    showContextMenu = false;

    switch (action) {
      case "rename":
        showNewDirNameModal = true;
        break;
      case "delete":
        showDeleteFileModal = true;
        break;
      case "close":
        contextFile = null;
        break;
      default:
        break;
    }
  };

  const closeDeleteFileModal = () => {
    showDeleteFileModal = false;
    contextFile = null;
  };
</script>

{#if showContextMenu}
  <ContextMenu pos={contextPos} {onContextAction} />
{/if}
{#if showPromptModal}
  <PromptModal
    {title}
    {text}
    onClose={closePrompModal}
    setResult={(r) => setResult(r)}
    hasExtraResult={true}
    setExtraResult={(r) => setResultAll(r)}
  />
{/if}
{#if showNewDirNameModal}
  <NewDirNameModal
    {dirs}
    {files}
    {contextFile}
    onClose={closeNewDirNameModal}
  />
{/if}
{#if showDeleteFileModal}
  <DeleteFileModal {dirs} {contextFile} onClose={closeDeleteFileModal} />
{/if}
<div class="relative w-full h-full">
  <div class="w-11/12 lg:w-4/5 h-full mx-auto my-4 lg:mt-4 lg:mb-10">
    <div class="flex flex-row items-center justify-between">
      <BreadCrum {dirs} className="py-1" />
      <div class="relative flex flex-row justify-end">
        <Button
          onClick={toggleShowNewMenu}
          value={$t("component.dir_list.new")}
          color="blue"
        />
        <input
          type="file"
          class="hidden"
          bind:this={fileSelector}
          multiple
          on:change={selectUploadFile}
        />
        {#if showNewMenu}
          <div
            class="absolute w-32 top-9 right-0 py-1 shadow-sm rounded-sm bg-white border"
          >
            <div
              class="px-2 py-1 hover:bg-gray-400 hover:text-white cursor-pointer text-center"
              on:click={openNewDirNameModal}
            >
              {$t("component.dir_list.create_folder")}
            </div>
            <div
              class="px-2 py-1 hover:bg-gray-400 hover:text-white cursor-pointer text-center"
              on:click={openSelectFileDialog}
            >
              {$t("component.dir_list.upload_files")}
            </div>
          </div>
        {/if}
      </div>
    </div>
    {#if isLoading}
      <Spinner />
    {:else}
      <div class="grid grid-cols-5 border-b border-gray-200 py-2 font-bold">
        <div class="col-span-3 px-2 flex flex-row items-center">
          <span
            class="cursor-pointer hover:text-gray-400"
            on:click={() => changeOrder("name")}
            >{$t("component.dir_list.filename")}</span
          >
          {#if order.key === "name" && order.asc}
            <Icon type={EIconType.up} size="tiny" className="ml-2" />
          {:else if order.key === "name"}
            <Icon type={EIconType.down} size="tiny" className="ml-2" />
          {/if}
        </div>
        <div class="px-2  flex flex-row items-center">
          <span
            class="cursor-pointer hover:text-gray-400"
            on:click={() => changeOrder("type")}
            >{$t("component.dir_list.type")}</span
          >
          {#if order.key === "type" && order.asc}
            <Icon type={EIconType.up} size="tiny" className="ml-2" />
          {:else if order.key === "type"}
            <Icon type={EIconType.down} size="tiny" className="ml-2" />
          {/if}
        </div>
        <div class="px-2  flex flex-row items-center">
          <span
            class="cursor-pointer hover:text-gray-400"
            on:click={() => changeOrder("size")}
            >{$t("component.dir_list.size")}
          </span>
          {#if order.key === "size" && order.asc}
            <Icon type={EIconType.up} size="tiny" className="ml-2" />
          {:else if order.key === "size"}
            <Icon type={EIconType.down} size="tiny" className="ml-2" />
          {/if}
        </div>
      </div>
      {#if dirs && dirs.length > 0}
        <div
          class="grid grid-cols-5 border-b border-gray-200 py-2 hover:bg-gray-200 cursor-pointer"
          on:click={backToParentDir}
        >
          <div class="col-span-3 px-2">..</div>
          <div class="px-2" />
          <div class="px-2" />
          <div class="px-2" />
        </div>
      {/if}
      {#each files as file}
        <div
          class="grid grid-cols-5 border-b border-gray-200 py-2 hover:bg-gray-200 cursor-pointer"
          on:contextmenu|preventDefault={(e) => openContextMenu(e, file)}
          on:click={() => selectFile(file)}
        >
          <div class="col-span-3 px-2 flex flex-row items-center">
            <FileIcon {file} />
            <span class="ml-2 break-all">
              {file.filename}
            </span>
          </div>
          <div class="px-2 my-auto">
            {$t("filetype." + file.file_type)}
          </div>
          <div class="px-2 my-auto">{formatSize(file.size)}</div>
        </div>
      {/each}
    {/if}
  </div>
</div>
